pub mod routes;
pub mod websocket;
pub mod auth;
pub mod state;
pub mod error;

pub use routes::*;
pub use websocket::*;
pub use auth::*;
pub use state::*;
pub use error::*;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use tower_http::cors::CorsLayer;
use std::sync::Arc;

pub fn create_api_router(state: Arc<AppState>) -> Router {
    Router::new()
        // Flow management
        .route("/api/flows", get(routes::flows::list_flows).post(routes::flows::create_flow))
        .route("/api/flows/:id", 
            get(routes::flows::get_flow)
            .put(routes::flows::update_flow)
            .delete(routes::flows::delete_flow))
        .route("/api/flows/:id/validate", post(routes::flows::validate_flow))
        .route("/api/flows/:id/execute", post(routes::flows::execute_flow))
        
        // Execution management
        .route("/api/executions", get(routes::executions::list_executions))
        .route("/api/executions/:id", get(routes::executions::get_execution))
        .route("/api/executions/:id/cancel", post(routes::executions::cancel_execution))
        
        // Node catalog
        .route("/api/nodes", get(routes::nodes::list_nodes))
        .route("/api/nodes/:id", get(routes::nodes::get_node))
        
        // WebSocket for real-time updates
        .route("/ws", get(websocket::websocket_handler))
        
        // Health check
        .route("/health", get(routes::health::health_check))
        
        .layer(CorsLayer::permissive())
        .with_state(state)
}