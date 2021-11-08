use crate::{identifier::ident, spec, Config, PropertyName, ResolvedSchema, Spec};
use autorust_openapi::{DataType, ReferenceOr, Schema, SchemaCommon};
use heck::CamelCase;
use once_cell::sync::Lazy;
use proc_macro2::TokenStream;
use quote::quote;
use regex::Regex;
use serde_json::Value;
use std::path::{Path, PathBuf};

/// code generation context
pub struct CodeGen {
    config: Config,
    pub spec: Spec,
}

impl CodeGen {
    pub fn new(config: Config) -> Result<Self, Error> {
        let spec = Spec::read_files(&config.input_files).map_err(Error::Spec)?;
        Ok(Self { config, spec })
    }

    pub fn input_files(&self) -> &[PathBuf] {
        &self.config.input_files
    }

    pub fn output_folder(&self) -> &Path {
        &self.config.output_folder
    }

    pub fn should_workaround_case(&self) -> bool {
        if let Some(title) = self.spec.title() {
            self.config.fix_case_properties.contains(title)
        } else {
            false
        }
    }

    pub fn should_force_optional(&self, prop_nm: &PropertyName) -> bool {
        self.config.optional_properties.contains(prop_nm)
    }

    pub fn should_force_obj(&self, prop_nm: &PropertyName) -> bool {
        self.config.invalid_types.contains(prop_nm)
    }

    pub fn should_box_property(&self, prop_nm: &PropertyName) -> bool {
        self.config.box_properties.contains(prop_nm)
    }

    pub fn get_request_content_type_json(&self) -> String {
        let consumes = self.spec.consumes();
        consumes
            .into_iter()
            .filter(|x| x.starts_with("application/json"))
            .map(|x| x.to_string())
            .next()
            .unwrap_or_else(|| "application/json".to_string())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("SpecError: {0}")]
    Spec(#[from] spec::Error),
    #[error("ArrayExpectedToHaveItems")]
    ArrayExpectedToHaveItems,
    #[error("NoNameForRef")]
    NoNameForRef,
    #[error("creating function name: {0}")]
    FunctionName(#[source] crate::identifier::Error),
    #[error("creating type name for schema ref: {0}")]
    TypeNameForSchemaRef(#[source] crate::identifier::Error),
    #[error("creating name for status code: {0}")]
    StatusCodeName(#[source] crate::identifier::Error),
    #[error("creating type name for response: {0}")]
    ResponseTypeName(#[source] crate::identifier::Error),
    #[error("creating type for response: {0}")]
    ResponseType(#[source] crate::status_codes::Error),
    #[error("creating name for param: {0}")]
    ParamName(#[source] crate::identifier::Error),
    #[error("creating name for property: {0}")]
    PropertyName(#[source] crate::identifier::Error),
    #[error("creating name for module: {0}")]
    ModuleName(#[source] crate::identifier::Error),
    #[error("creating name for enum variant: {0}")]
    EnumVariantName(#[source] crate::identifier::Error),
    #[error("creating name for enum {property}: {source}")]
    EnumName {
        source: crate::identifier::Error,
        property: String,
    },
    #[error("creating name for enum value {property}: {source}")]
    EnumValueName {
        source: crate::identifier::Error,
        property: String,
    },
    #[error("creating name for Vec alias: {0}")]
    VecAliasName(#[source] crate::identifier::Error),
    #[error("creating name for struct: {0}")]
    StructName(#[source] crate::identifier::Error),
    #[error("creating name for field in struct: {0}")]
    StructFieldName(#[source] crate::identifier::Error),
    #[error("api-version is missing")]
    MissingApiVersion,
    #[error("operation {0} is missing an x-ms-examples")]
    OperationMissingExample(String),
    #[error("operation is missing responses")]
    OperationMissingResponses,
    #[error("creating path for example {0}")]
    ExamplePath(#[source] crate::path::Error),
    #[error("example path not utf8")]
    ExamplePathNotUtf8,
    #[error("status code required")]
    StatusCodeRequired,
    #[error("creating name for examples")]
    ExamplesName(#[source] crate::identifier::Error),
    #[error("status code: {0}")]
    StatusCode(#[from] crate::status_codes::Error),
}

pub fn is_vec(ts: &TokenStream) -> bool {
    ts.to_string().starts_with("Vec <")
}

pub fn is_array(schema: &SchemaCommon) -> bool {
    matches!(schema.type_, Some(DataType::Array))
}

pub fn is_string(schema: &SchemaCommon) -> bool {
    matches!(schema.type_, Some(DataType::String))
}

pub fn get_schema_array_items(schema: &SchemaCommon) -> Result<&ReferenceOr<Schema>, Error> {
    schema.items.as_ref().as_ref().ok_or(Error::ArrayExpectedToHaveItems)
}

/// A header placed at the top the file to say that it is generated by AutoRust.
pub fn create_generated_by_header() -> TokenStream {
    let version = env!("CARGO_PKG_VERSION");
    let comment = format!("generated by AutoRust {}", &version);
    quote! { #![doc = #comment] }
}

pub fn is_local_enum(property: &ResolvedSchema) -> bool {
    !property.schema.common.enum_.is_empty()
}

pub fn is_local_struct(property: &ResolvedSchema) -> bool {
    !property.schema.properties.is_empty()
}

pub fn is_basic_type(property: &ResolvedSchema) -> bool {
    matches!(
        property.schema.common.type_,
        Some(DataType::Integer | DataType::String | DataType::Number | DataType::Boolean)
    )
}

/// Wraps a type in an Option if is not required.
pub fn require(is_required: bool, tp: TokenStream) -> TokenStream {
    if is_required {
        tp
    } else {
        quote! { Option<#tp> }
    }
}

pub fn enum_values_as_strings(values: &[Value]) -> Vec<&str> {
    values
        .iter()
        .filter_map(|v| match v {
            Value::String(s) => Some(s.as_str()),
            _ => None,
        })
        .collect()
}

pub enum TypeName {
    Reference(String),
    Array(Box<TypeName>),
    Value,
    Bytes,
    Int32,
    Int64,
    Float32,
    Float64,
    Boolean,
    String,
}

impl TypeName {
    pub fn to_token_stream(&self, as_ref: bool, qualify_models: bool) -> Result<TokenStream, Error> {
        Ok(match self {
            TypeName::Reference(name) => {
                let idt = ident(&name.to_camel_case()).map_err(Error::TypeNameForSchemaRef)?;
                let idt = if qualify_models {
                    quote! { models::#idt }
                } else {
                    idt
                };
                match as_ref {
                    true => quote! { &#idt },
                    false => idt,
                }
            }
            TypeName::Array(vec_items_typ) => {
                let vec_items_typ = vec_items_typ.to_token_stream(as_ref, qualify_models)?;
                match as_ref {
                    true => quote! { &[#vec_items_typ] },
                    false => quote! { Vec<#vec_items_typ> },
                }
            }
            TypeName::Value => match as_ref {
                true => quote! { &serde_json::Value },
                false => quote! { serde_json::Value },
            },
            TypeName::Bytes => quote! { bytes::Bytes },
            TypeName::Int32 => quote! { i32 },
            TypeName::Int64 => quote! { i64 },
            TypeName::Float32 => quote! { f32 },
            TypeName::Float64 => quote! { f64 },
            TypeName::Boolean => quote! { bool },
            TypeName::String => match as_ref {
                true => quote! { &str },
                false => quote! { String },
            },
        })
    }
}

pub fn get_type_name_for_schema(schema: &SchemaCommon) -> Result<TypeName, Error> {
    Ok(if let Some(schema_type) = &schema.type_ {
        let format = schema.format.as_deref();
        match schema_type {
            DataType::Array => {
                let items = get_schema_array_items(schema)?;
                let vec_items_typ = get_type_name_for_schema_ref(items)?;
                TypeName::Array(Box::new(vec_items_typ))
            }
            DataType::Integer => {
                if format == Some("int32") {
                    TypeName::Int32
                } else {
                    TypeName::Int64
                }
            }
            DataType::Number => {
                if format == Some("float") {
                    TypeName::Float32
                } else {
                    TypeName::Float64
                }
            }
            DataType::String => TypeName::String,
            DataType::Boolean => TypeName::Boolean,
            DataType::Object => TypeName::Value,
            DataType::File => TypeName::Bytes,
        }
    } else {
        // eprintln!(
        //     "WARN unknown type in get_type_name_for_schema, description {:?}",
        //     schema.description
        // );
        TypeName::Value
    })
}

pub fn get_type_name_for_schema_ref(schema: &ReferenceOr<Schema>) -> Result<TypeName, Error> {
    Ok(match schema {
        ReferenceOr::Reference { reference, .. } => {
            let name = reference.name.as_ref().ok_or(Error::NoNameForRef)?;
            TypeName::Reference(name.to_owned())
        }
        ReferenceOr::Item(schema) => get_type_name_for_schema(&schema.common)?,
    })
}

pub fn create_mod(api_version: &str) -> TokenStream {
    quote! {
        pub mod models;
        pub mod operations;
        #[allow(dead_code)]
        pub const API_VERSION: &str = #api_version;
    }
}

// any word character or `-` between curly braces
pub static PARAM_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\{([\w-]+)\}").unwrap());

pub fn parse_params(path: &str) -> Vec<String> {
    // capture 0 is the whole match and 1 is the actual capture like other languages
    PARAM_RE.captures_iter(path).into_iter().map(|c| c[1].to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_enum_values_as_strings() {
        let values = vec![json!("/"), json!("/keys")];
        assert_eq!(enum_values_as_strings(&values), vec!["/", "/keys"]);
    }

    #[test]
    fn test_parse_params_keyvault() -> Result<(), Error> {
        assert_eq!(
            parse_params("/storage/{storage-account-name}/sas/{sas-definition-name}"),
            vec!["storage-account-name".to_owned(), "sas-definition-name".to_owned()]
        );
        Ok(())
    }
}
