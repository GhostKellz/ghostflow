use ghostflow_core::NodeRegistry;
use ghostflow_engine::FlowRuntime;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub runtime: Arc<FlowRuntime>,
    pub node_registry: Arc<dyn NodeRegistry>,
    pub websocket_clients: Arc<RwLock<WebSocketClients>>,
}

pub type WebSocketClients = std::collections::HashMap<uuid::Uuid, tokio::sync::mpsc::UnboundedSender<String>>;

impl AppState {
    pub fn new(
        db_pool: PgPool,
        runtime: Arc<FlowRuntime>,
        node_registry: Arc<dyn NodeRegistry>,
    ) -> Self {
        Self {
            db_pool,
            runtime,
            node_registry,
            websocket_clients: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    pub async fn broadcast_message(&self, message: &str) {
        let clients = self.websocket_clients.read().await;
        for (_, tx) in clients.iter() {
            let _ = tx.send(message.to_string());
        }
    }
}