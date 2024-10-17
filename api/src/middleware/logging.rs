use std::time::Instant;

use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    middleware::Next,
};
use chrono::Local;
use colored::*;
const LINE_LENGTH: usize = 110;

pub async fn logging(req: Request<Body>, next: Next) -> Result<Response<Body>, StatusCode> {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let start = Instant::now();
    let response = next.run(req).await;
    let duration = start.elapsed();

    // Format the elapsed time
    let duration_str = format!("~ {:?}", duration);
    let formatted_duration = duration_str.dimmed();

    // Calculate the number of dots to align the duration to the right
    let content = format!("{} {}", method, uri);
    let content_len = timestamp.len() + content.len() + duration_str.len() + 2;
    let dots_count = LINE_LENGTH.saturating_sub(content_len);

    // Generate the dots as a single string
    let mut dots = String::new();
    for _ in 0..dots_count {
        dots.push_str(".");
    }

    let dots = dots.dimmed();

    println!(
        "{} {} {} {}",
        timestamp.dimmed(),
        content.cyan(),
        dots,
        formatted_duration
    );

    Ok(response)
}
