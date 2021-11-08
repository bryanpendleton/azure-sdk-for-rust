#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use super::{models, API_VERSION};
#[non_exhaustive]
#[derive(Debug, thiserror :: Error)]
#[allow(non_camel_case_types)]
pub enum Error {
    #[error(transparent)]
    Operations_List(#[from] operations::list::Error),
    #[error(transparent)]
    GetTestResultFile(#[from] get_test_result_file::Error),
}
pub mod operations {
    use super::{models, API_VERSION};
    pub async fn list(operation_config: &crate::OperationConfig) -> std::result::Result<models::OperationsListResult, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!("{}/providers/microsoft.insights/operations", operation_config.base_path(),);
        let mut url = url::Url::parse(url_str).map_err(list::Error::ParseUrlError)?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(list::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder.body(req_body).map_err(list::Error::BuildRequestError)?;
        let rsp = http_client.execute_request(req).await.map_err(list::Error::ExecuteRequestError)?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: models::OperationsListResult =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError(source, rsp_body.clone()))?;
                Ok(rsp_value)
            }
            status_code => Err(list::Error::DefaultResponse { status_code }),
        }
    }
    pub mod list {
        use super::{models, API_VERSION};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse { status_code: http::StatusCode },
            #[error("Failed to parse request URL: {0}")]
            ParseUrlError(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequestError(http::Error),
            #[error("Failed to execute request: {0}")]
            ExecuteRequestError(azure_core::HttpError),
            #[error("Failed to serialize request body: {0}")]
            SerializeError(serde_json::Error),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            DeserializeError(serde_json::Error, bytes::Bytes),
            #[error("Failed to get access token: {0}")]
            GetTokenError(azure_core::Error),
        }
    }
}
pub async fn get_test_result_file(
    operation_config: &crate::OperationConfig,
    resource_group_name: &str,
    subscription_id: &str,
    web_test_name: &str,
    geo_location_id: &str,
    time_stamp: i64,
    download_as: &str,
    test_successful_criteria: Option<bool>,
    continuation_token: Option<&str>,
) -> std::result::Result<models::TestResultFileResponse, get_test_result_file::Error> {
    let http_client = operation_config.http_client();
    let url_str = &format!(
        "{}/subscriptions/{}/resourcegroups/{}/providers/microsoft.insights/webtests/{}/getTestResultFile",
        operation_config.base_path(),
        subscription_id,
        resource_group_name,
        web_test_name
    );
    let mut url = url::Url::parse(url_str).map_err(get_test_result_file::Error::ParseUrlError)?;
    let mut req_builder = http::request::Builder::new();
    req_builder = req_builder.method(http::Method::POST);
    if let Some(token_credential) = operation_config.token_credential() {
        let token_response = token_credential
            .get_token(operation_config.token_credential_resource())
            .await
            .map_err(get_test_result_file::Error::GetTokenError)?;
        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
    }
    url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
    url.query_pairs_mut().append_pair("geoLocationId", geo_location_id);
    url.query_pairs_mut().append_pair("timeStamp", time_stamp.to_string().as_str());
    url.query_pairs_mut().append_pair("downloadAs", download_as);
    if let Some(test_successful_criteria) = test_successful_criteria {
        url.query_pairs_mut()
            .append_pair("testSuccessfulCriteria", test_successful_criteria.to_string().as_str());
    }
    if let Some(continuation_token) = continuation_token {
        url.query_pairs_mut().append_pair("continuationToken", continuation_token);
    }
    let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
    req_builder = req_builder.header(http::header::CONTENT_LENGTH, 0);
    req_builder = req_builder.uri(url.as_str());
    let req = req_builder.body(req_body).map_err(get_test_result_file::Error::BuildRequestError)?;
    let rsp = http_client
        .execute_request(req)
        .await
        .map_err(get_test_result_file::Error::ExecuteRequestError)?;
    match rsp.status() {
        http::StatusCode::OK => {
            let rsp_body = rsp.body();
            let rsp_value: models::TestResultFileResponse = serde_json::from_slice(rsp_body)
                .map_err(|source| get_test_result_file::Error::DeserializeError(source, rsp_body.clone()))?;
            Ok(rsp_value)
        }
        status_code => {
            let rsp_body = rsp.body();
            let rsp_value: models::ErrorResponse = serde_json::from_slice(rsp_body)
                .map_err(|source| get_test_result_file::Error::DeserializeError(source, rsp_body.clone()))?;
            Err(get_test_result_file::Error::DefaultResponse {
                status_code,
                value: rsp_value,
            })
        }
    }
}
pub mod get_test_result_file {
    use super::{models, API_VERSION};
    #[derive(Debug, thiserror :: Error)]
    pub enum Error {
        #[error("HTTP status code {}", status_code)]
        DefaultResponse {
            status_code: http::StatusCode,
            value: models::ErrorResponse,
        },
        #[error("Failed to parse request URL: {0}")]
        ParseUrlError(url::ParseError),
        #[error("Failed to build request: {0}")]
        BuildRequestError(http::Error),
        #[error("Failed to execute request: {0}")]
        ExecuteRequestError(azure_core::HttpError),
        #[error("Failed to serialize request body: {0}")]
        SerializeError(serde_json::Error),
        #[error("Failed to deserialize response: {0}, body: {1:?}")]
        DeserializeError(serde_json::Error, bytes::Bytes),
        #[error("Failed to get access token: {0}")]
        GetTokenError(azure_core::Error),
    }
}
