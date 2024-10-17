use axum::{middleware, routing::get, Router};

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
