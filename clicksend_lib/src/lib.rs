pub mod accounts;
pub mod automations;
pub mod contacts;
pub mod error;
pub mod messaging;
pub mod other;
pub mod post;

use error::{AppError, AppResult};
use reqwest::{
    blocking::Client,
    header::{self, HeaderMap, HeaderValue},
};

pub struct ClickSendClient {
    base_url: String,
    api_key: String,
    version: String,
    client: Client,
}

impl ClickSendClient {
    pub fn new(base_url: &str, api_key: &str, version: &str) -> AppResult<Self> {
        let mut headers = HeaderMap::new();

        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key)).map_err(|_| {
                AppError::ClickSendApiError("Unable to construct Authorization string".into())
            })?,
        );
        headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
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
        })
    }
}
