use std::{env, sync::Arc};

use axum::{Extension, Router};
use clicksend_lib::ClickSendClient;
use colored::*;
use config::AppConfig;

pub mod config;
pub mod error;
pub mod middleware;
pub mod routes;

#[tokio::main]
async fn main() {
    let cfg = AppConfig::from_env()
        .expect("Expect to be able to construct config from environment and .env file");

    let addr = format!("0.0.0.0:{}", &cfg.port);

    let clicksend_client = Arc::new(ClickSendClient::new(
        &cfg.clicksend_baseurl,
        &cfg.clicksend_api_key,
        &cfg.clicksend_version,
    ));

    let app = Router::new()
        .nest("/api", routes::create_routes())
        .layer(Extension(clicksend_client));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    startup_message(&addr);

    // Serve the app
    axum::serve(listener, app).await.unwrap();
}

fn startup_message(address: &str) {
    let info = "INFO".white().on_bright_blue();
    let message = format!("Server running on [http://{address}].");
    let stop_message = "Press Ctrl+C to stop the server.".yellow();

    println!("  {} {}", info, message);
    println!("\n\n  {}\n", stop_message);
}
