use std::sync::Arc;

use axum::{middleware, routing::get, Extension, Json, Router};
use clicksend_lib::ClickSendClient;

use crate::middleware::logging::logging;

pub fn create_routes() -> Router {
    Router::new()
        .route("/hello", get(hello))
        .fallback(fallback_404)
        .layer(middleware::from_fn(logging))
}

async fn hello() -> &'static str {
    "Message Sent!"
}
async fn fallback_404() -> &'static str {
    "404 Error!"
}

pub fn send_sms_handler(
    Extension(clicksend_client): Extension<Arc<ClickSendClient>>,
    Json(payload): Json<SmsPayload>,
) -> Json<ResponseMessage> {
    let result = clicksend_client
        .send_sms(&payload.to, &payload.message)
        .await;
    match result {
        Ok(_) => Json(ResponseMessage {
            status: "Message sent successfully".into(),
        }),
        Err(_) => Json(ResponseMessage {
            status: "Failed to send message".into(),
        }),
    }
}

pub struct SmsPayload {
    pub to: String,
    pub message: String,
}

pub struct ResponseMessage {
    pub status: String,
}
