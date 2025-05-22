use warp::Filter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    message: String,
    status: String,
}

#[tokio::main]
async fn main() {
    // GET /
    let hello = warp::path::end()
        .map(|| {
            let response = ApiResponse {
                message: "Hello from Rust Docker container!".to_string(),
                status: "success".to_string(),
            };
            warp::reply::json(&response)
        });

    // GET /health
    let health = warp::path("health")
        .map(|| {
            let response = ApiResponse {
                message: "Service is healthy".to_string(),
                status: "ok".to_string(),
            };
            warp::reply::json(&response)
        });

    // GET /api/info
    let info = warp::path!("api" / "info")
        .map(|| {
            let response = serde_json::json!({
                "service": "rust-project",
                "version": "0.1.0",
                "description": "Rust web service running in Docker",
                "author": "Abdulwahed",
                "port": 8001
            });
            warp::reply::json(&response)
        });

    let routes = hello
        .or(health)
        .or(info)
        .with(warp::cors().allow_any_origin());

    println!("🚀 Server starting on http://0.0.0.0:8001");
    println!("📍 Endpoints:");
    println!("   GET /        - Welcome message");
    println!("   GET /health  - Health check");
    println!("   GET /api/info - Service information");

    warp::serve(routes)
        .bind(([0, 0, 0, 0], 8001))
        .await;
}