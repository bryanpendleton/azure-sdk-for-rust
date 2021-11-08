#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2021-07")]
pub mod package_2021_07;
#[cfg(all(feature = "package-2021-07", not(feature = "no-default-version")))]
pub use package_2021_07::{models, operations, operations::Error};
#[cfg(feature = "package-2021-03")]
pub mod package_2021_03;
#[cfg(all(feature = "package-2021-03", not(feature = "no-default-version")))]
pub use package_2021_03::{models, operations, operations::Error};
#[cfg(feature = "package-preview-2021-03")]
pub mod package_preview_2021_03;
#[cfg(all(feature = "package-preview-2021-03", not(feature = "no-default-version")))]
pub use package_preview_2021_03::{models, operations, operations::Error};
#[cfg(feature = "package-preview-2021-02")]
pub mod package_preview_2021_02;
#[cfg(all(feature = "package-preview-2021-02", not(feature = "no-default-version")))]
pub use package_preview_2021_02::{models, operations, operations::Error};
#[cfg(feature = "package-2020-08-31")]
pub mod package_2020_08_31;
#[cfg(all(feature = "package-2020-08-31", not(feature = "no-default-version")))]
pub use package_2020_08_31::{models, operations, operations::Error};
#[cfg(feature = "package-preview-2020-08-31")]
pub mod package_preview_2020_08_31;
#[cfg(all(feature = "package-preview-2020-08-31", not(feature = "no-default-version")))]
pub use package_preview_2020_08_31::{models, operations, operations::Error};
#[cfg(feature = "package-2020-08")]
pub mod package_2020_08;
#[cfg(all(feature = "package-2020-08", not(feature = "no-default-version")))]
pub use package_2020_08::{models, operations, operations::Error};
#[cfg(feature = "package-preview-2020-07")]
pub mod package_preview_2020_07;
#[cfg(all(feature = "package-preview-2020-07", not(feature = "no-default-version")))]
pub use package_preview_2020_07::{models, operations, operations::Error};
#[cfg(feature = "package-2020-06")]
pub mod package_2020_06;
#[cfg(all(feature = "package-2020-06", not(feature = "no-default-version")))]
pub use package_2020_06::{models, operations, operations::Error};
#[cfg(feature = "package-2020-04")]
pub mod package_2020_04;
#[cfg(all(feature = "package-2020-04", not(feature = "no-default-version")))]
pub use package_2020_04::{models, operations, operations::Error};
#[cfg(feature = "package-2020-03")]
pub mod package_2020_03;
#[cfg(all(feature = "package-2020-03", not(feature = "no-default-version")))]
pub use package_2020_03::{models, operations, operations::Error};
#[cfg(feature = "package-2019-11")]
pub mod package_2019_11;
#[cfg(all(feature = "package-2019-11", not(feature = "no-default-version")))]
pub use package_2019_11::{models, operations, operations::Error};
#[cfg(feature = "package-preview-2019-07")]
pub mod package_preview_2019_07;
#[cfg(all(feature = "package-preview-2019-07", not(feature = "no-default-version")))]
pub use package_preview_2019_07::{models, operations, operations::Error};
#[cfg(feature = "package-2019-03")]
pub mod package_2019_03;
#[cfg(all(feature = "package-2019-03", not(feature = "no-default-version")))]
pub use package_2019_03::{models, operations, operations::Error};
#[cfg(feature = "package-preview-2019-03")]
pub mod package_preview_2019_03;
#[cfg(all(feature = "package-preview-2019-03", not(feature = "no-default-version")))]
pub use package_preview_2019_03::{models, operations, operations::Error};
#[cfg(feature = "package-2018-12-preview")]
pub mod package_2018_12_preview;
#[cfg(all(feature = "package-2018-12-preview", not(feature = "no-default-version")))]
pub use package_2018_12_preview::{models, operations, operations::Error};
#[cfg(feature = "package-2018-04")]
pub mod package_2018_04;
#[cfg(all(feature = "package-2018-04", not(feature = "no-default-version")))]
pub use package_2018_04::{models, operations, operations::Error};
#[cfg(feature = "package-2018-01")]
pub mod package_2018_01;
#[cfg(all(feature = "package-2018-01", not(feature = "no-default-version")))]
pub use package_2018_01::{models, operations, operations::Error};
#[cfg(feature = "package-2017-07")]
pub mod package_2017_07;
#[cfg(all(feature = "package-2017-07", not(feature = "no-default-version")))]
pub use package_2017_07::{models, operations, operations::Error};
#[cfg(feature = "package-2017-01")]
pub mod package_2017_01;
#[cfg(all(feature = "package-2017-01", not(feature = "no-default-version")))]
pub use package_2017_01::{models, operations, operations::Error};
#[cfg(feature = "package-2016-02")]
pub mod package_2016_02;
#[cfg(all(feature = "package-2016-02", not(feature = "no-default-version")))]
pub use package_2016_02::{models, operations, operations::Error};
#[cfg(feature = "profile-hybrid-2020-09-01")]
pub mod profile_hybrid_2020_09_01;
use azure_core::setters;
#[cfg(all(feature = "profile-hybrid-2020-09-01", not(feature = "no-default-version")))]
pub use profile_hybrid_2020_09_01::{models, operations, operations::Error};
pub fn config(
    http_client: std::sync::Arc<dyn azure_core::HttpClient>,
    token_credential: Box<dyn azure_core::TokenCredential>,
) -> OperationConfigBuilder {
    OperationConfigBuilder {
        http_client,
        base_path: None,
        token_credential,
        token_credential_resource: None,
    }
}
pub struct OperationConfigBuilder {
    http_client: std::sync::Arc<dyn azure_core::HttpClient>,
    base_path: Option<String>,
    token_credential: Box<dyn azure_core::TokenCredential>,
    token_credential_resource: Option<String>,
}
impl OperationConfigBuilder {
    setters! { base_path : String => Some (base_path) , token_credential_resource : String => Some (token_credential_resource) , }
    pub fn build(self) -> OperationConfig {
        OperationConfig {
            http_client: self.http_client,
            base_path: self.base_path.unwrap_or_else(|| "https://management.azure.com".to_owned()),
            token_credential: Some(self.token_credential),
            token_credential_resource: self
                .token_credential_resource
                .unwrap_or_else(|| "https://management.azure.com/".to_owned()),
        }
    }
}
pub struct OperationConfig {
    http_client: std::sync::Arc<dyn azure_core::HttpClient>,
    base_path: String,
    token_credential: Option<Box<dyn azure_core::TokenCredential>>,
    token_credential_resource: String,
}
impl OperationConfig {
    pub fn http_client(&self) -> &dyn azure_core::HttpClient {
        self.http_client.as_ref()
    }
    pub fn base_path(&self) -> &str {
        self.base_path.as_str()
    }
    pub fn token_credential(&self) -> Option<&dyn azure_core::TokenCredential> {
        self.token_credential.as_deref()
    }
    pub fn token_credential_resource(&self) -> &str {
        self.token_credential_resource.as_str()
    }
}
