use colored::*;
pub mod middleware;
pub mod routes;
#[tokio::main]
async fn main() {
    let app = routes::create_routes();
    let addr = "0.0.0.0:3000";

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
