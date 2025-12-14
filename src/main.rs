use axum::{
    Router,
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::env;
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

#[derive(Deserialize)]
struct ChatRequest {
    message: String,
    model: Option<String>,
}

#[derive(Serialize)]
struct ChatResponse {
    response: String,
    model: String,
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Serialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<AnthropicMessage>,
}

#[derive(Deserialize)]
struct AnthropicContentBlock {
    #[serde(rename = "type")]
    content_type: String,
    text: Option<String>,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContentBlock>,
    model: String,
}

async fn chat(
    Json(payload): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, (StatusCode, Json<ErrorResponse>)> {
    let api_key = env::var("ANTHROPIC_API_KEY").map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "ANTHROPIC_API_KEY not set".to_string(),
            }),
        )
    })?;

    let model = payload
        .model
        .unwrap_or_else(|| "claude-sonnet-4-20250514".to_string());

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", &api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&AnthropicRequest {
            model: model.clone(),
            max_tokens: 1024,
            messages: vec![AnthropicMessage {
                role: "user".to_string(),
                content: payload.message,
            }],
        })
        .send()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: error_text }),
        ));
    }

    let anthropic_response: AnthropicResponse = response.json().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
    })?;

    let text = anthropic_response
        .content
        .iter()
        .find(|block| block.content_type == "text")
        .and_then(|block| block.text.clone())
        .unwrap_or_default();

    Ok(Json(ChatResponse {
        response: text,
        model: anthropic_response.model,
    }))
}

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/greet/{name}", get(greet_name))
        .route("/ai/chat", post(chat))
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

    #[tokio::test]
    async fn test_chat_requires_api_key() {
        let server = TestServer::new(create_router()).unwrap();
        let response = server
            .post("/ai/chat")
            .json(&serde_json::json!({"message": "Hello"}))
            .await;
        response.assert_status(StatusCode::INTERNAL_SERVER_ERROR);
    }
}
