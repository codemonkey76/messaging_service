pub mod accounts;
pub mod automations;
pub mod contacts;
pub mod error;
pub mod messaging;
pub mod other;
pub mod post;

use base64::{engine::general_purpose, Engine as _};
use error::{AppError, AppResult};
use reqwest::{
    blocking::{Client, RequestBuilder},
    header::{self, HeaderMap, HeaderValue},
};
use serde::{de::DeserializeOwned, Serialize};

pub struct ClickSendClient {
    pub base_url: String,
    pub username: String,
    pub api_key: String,
    pub version: String,
    pub client: Client,
}

impl ClickSendClient {
    pub fn new(base_url: &str, username: &str, api_key: &str, version: &str) -> AppResult<Self> {
        let credentials = format!("{}:{}", username, api_key);
        let encoded_creds = general_purpose::STANDARD.encode(credentials);

        let mut headers = HeaderMap::new();

        headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );

        let auth_header_value = format!("Basic {}", encoded_creds);
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&auth_header_value).map_err(|_| {
                AppError::ClickSendApiError("Unable to construct authorization header".to_string())
            })?,
        );

        Ok(Self {
            api_key: api_key.to_string(),
            base_url: base_url.to_string(),
            version: version.to_string(),
            client: Client::builder()
                .default_headers(headers)
                .build()
                .map_err(|_| {
                    AppError::ClickSendApiError("Unable to contruct request client".into())
                })?,
            username: username.to_string(),
        })
    }

    pub fn post<T: DeserializeOwned, U: Serialize>(
        &self,
        endpoint: &str,
        data: &U,
    ) -> AppResult<T> {
        let url = format!("{}/{}/{}", self.base_url, self.version, endpoint);

        let request_builder = self.client.post(&url).json(data);
        match self.send_request(request_builder)? {
            Some(data) => Ok(data),
            None => Err(AppError::ClickSendApiError(
                "Expected response data, but received none".to_string(),
            )),
        }
    }

    fn send_request<T: DeserializeOwned>(
        &self,
        request_builder: RequestBuilder,
    ) -> AppResult<Option<T>> {
        let response = request_builder
            .send()
            .map_err(|e| AppError::ClickSendApiError(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .unwrap_or_else(|_| "No error details available".into());

            return Err(AppError::ClickSendApiError(format!(
                "Request returned error {}: {}",
                status, error_text
            )));
        }

        let response_text = response.text().map_err(|e| {
            AppError::ClickSendApiError(format!("Failed to read response text: {}", e))
        })?;

        if response_text.trim().is_empty() {
            return Ok(None);
        }
        dbg!(&response_text);

        let parsed_response: T = serde_json::from_str(&response_text)
            .map_err(|e| AppError::ClickSendApiError(format!("Failed to parse response: {}", e)))?;

        Ok(Some(parsed_response))
    }
}
