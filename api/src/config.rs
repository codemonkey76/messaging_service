use config::Config;
use serde::Deserialize;

use crate::error::AppResult;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub port: String,
    pub clicksend_api_key: String,
    pub clicksend_username: String,
    pub clicksend_baseurl: String,
    pub clicksend_version: String,
}

impl AppConfig {
    pub fn from_env() -> AppResult<Self> {
        dotenv::dotenv().ok();

        Ok(Config::builder()
            .set_default("PORT", "3000")?
            .set_default("CLICKSEND_BASE_URL", "https://rest.clicksend.com")?
            .set_default("CLICKSEND_VERSION", "v3")?
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize()?)
    }
}
