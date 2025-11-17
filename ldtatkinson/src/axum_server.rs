// Axum Server Main Application File

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tower_http::services::{ServeDir, ServeFile};

// --- STATE MANAGEMENT (Used for passing application state, like DB connections or API keys) ---
// Since we don't have an API key yet, we'll just use a simple state struct.
#[derive(Clone)]
struct AppState {
// You'll put things like the Gemini API key here later
app_name: String,
}

// --- API ROUTES ---

// Placeholder for the future AI chat API endpoint
// This function will be expanded later to call the Gemini API.
async fn chat_handler(Json(payload): Json<ChatRequest>) -> impl IntoResponse {
println!("Received chat request: {}", payload.message);

// Placeholder Logic: Echo the message and hint at the future use of the LLM
let response_text = format!(
"Rust Backend Acknowledged: '{}'.\n\n(Note: This will soon call the Gemini LLM for a real response!)",
payload.message
);

let response = ChatResponse {
response: response_text,
};

(StatusCode::OK, Json(response))
}

// Structs for API request and response bodies
#[derive(Deserialize)]
struct ChatRequest {
message: String,
}

#[derive(Serialize)]
struct ChatResponse {
response: String,
}

// --- MAIN FUNCTION ---

#[tokio::main]
pub async fn main() {
    // Initialize tracing for better logging
    tracing_subscriber::fmt::init();

    // The state we'll share across all handlers
    let shared_state = Arc::new(AppState {
        app_name: "LDT Atkinson Portfolio Backend".to_string(),
    });

    // Static file serving
    let root_path = std::path::PathBuf::from("dist");
    let not_found_path = root_path.join("index.html");

    let app = Router::new()
        // API Route: POST /api/chat
        .route("/api/chat", post(chat_handler))
        // Serve static files and fallback to index.html
        .fallback_service(
            ServeDir::new(root_path)
                .not_found_service(ServeFile::new(not_found_path)),
        )
        .with_state(shared_state);

    // Determine the port from the environment variable (Render standard) or default to 8080
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT environment variable must be a valid number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    tracing::debug!("Server listening on http://{}", addr);

    // Start the server using the new `axum::serve` API
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
