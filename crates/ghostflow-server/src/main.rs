use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

#[derive(Clone)]
struct AppState {}

async fn health() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "service": "ghostflow-server",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

async fn create_flow(State(_state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "message": "Flow creation not yet implemented"
    })))
}

async fn list_flows(State(_state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "flows": []
    })))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let state = AppState {};

    let app = Router::new()
        .route("/health", get(health))
        .route("/flows", get(list_flows).post(create_flow))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("GhostFlow server starting on {}", addr);
    
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}