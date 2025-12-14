use axum::{Router, extract::Path, response::Json, routing::get};
use serde::Serialize;
use std::net::SocketAddr;

pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
}

async fn root() -> String {
    greet("World")
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

async fn greet_name(Path(name): Path<String>) -> String {
    greet(&name)
}

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/greet/{name}", get(greet_name))
}

#[tokio::main]
async fn main() {
    let app = create_router();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running at http://localhost:3000");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[test]
    fn test_greet() {
        assert_eq!(greet("world"), "Hello, world!");
        assert_eq!(greet("Rust"), "Hello, Rust!");
    }

    #[tokio::test]
    async fn test_root() {
        let server = TestServer::new(create_router()).unwrap();
        let response = server.get("/").await;
        response.assert_status_ok();
        response.assert_text("Hello, World!");
    }

    #[tokio::test]
    async fn test_health() {
        let server = TestServer::new(create_router()).unwrap();
        let response = server.get("/health").await;
        response.assert_status_ok();
        response.assert_json(&serde_json::json!({"status": "ok"}));
    }

    #[tokio::test]
    async fn test_greet_name() {
        let server = TestServer::new(create_router()).unwrap();
        let response = server.get("/greet/Austin").await;
        response.assert_status_ok();
        response.assert_text("Hello, Austin!");
    }
}
