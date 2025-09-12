use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        ConnectInfo, Query, State,
    },
    response::Response,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::{AppState, ApiResult};
use ghostflow_schema::ExecutionStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    #[serde(rename = "type")]
    pub message_type: WebSocketMessageType,
    pub data: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebSocketMessageType {
    // Client to Server
    Subscribe,
    Unsubscribe,
    Ping,
    
    // Server to Client
    ExecutionStarted,
    ExecutionProgress,
    ExecutionCompleted,
    ExecutionFailed,
    NodeStarted,
    NodeCompleted,
    NodeFailed,
    FlowUpdated,
    Pong,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeMessage {
    pub flow_id: Option<String>,
    pub execution_id: Option<String>,
    pub event_types: Vec<WebSocketMessageType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionEvent {
    pub execution_id: String,
    pub flow_id: String,
    pub status: ExecutionStatus,
    pub progress: Option<ExecutionProgress>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionProgress {
    pub current_node: String,
    pub total_nodes: u32,
    pub completed_nodes: u32,
    pub percentage: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeEvent {
    pub execution_id: String,
    pub flow_id: String,
    pub node_id: String,
    pub node_type: String,
    pub status: NodeExecutionStatus,
    pub duration_ms: Option<u64>,
    pub output_data: Option<serde_json::Value>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeExecutionStatus {
    Started,
    Completed,
    Failed,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowUpdateEvent {
    pub flow_id: String,
    pub update_type: FlowUpdateType,
    pub updated_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FlowUpdateType {
    Created,
    Updated,
    Deleted,
    StatusChanged,
}

#[derive(Debug, Deserialize)]
pub struct WebSocketQuery {
    pub token: Option<String>,
    pub workspace_id: Option<String>,
}

pub struct WebSocketConnection {
    pub id: String,
    pub user_id: Option<String>,
    pub workspace_id: Option<String>,
    pub subscriptions: HashMap<String, SubscribeMessage>,
    pub sender: tokio::sync::mpsc::UnboundedSender<Message>,
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Query(query): Query<WebSocketQuery>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<AppState>>,
) -> Response {
    // TODO: Validate token and get user info
    let user_id = query.token.map(|_| "user_123".to_string());
    let workspace_id = query.workspace_id.unwrap_or_else(|| "default".to_string());
    
    log::info!("WebSocket connection from {}", addr);
    
    ws.on_upgrade(move |socket| websocket_connection_handler(socket, state, user_id, workspace_id))
}

async fn websocket_connection_handler(
    socket: WebSocket,
    state: Arc<AppState>,
    user_id: Option<String>,
    workspace_id: String,
) {
    let connection_id = Uuid::new_v4().to_string();
    let (sender, mut receiver) = socket.split();
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    
    // Create connection record
    let connection = WebSocketConnection {
        id: connection_id.clone(),
        user_id: user_id.clone(),
        workspace_id: workspace_id.clone(),
        subscriptions: HashMap::new(),
        sender: tx.clone(),
    };
    
    // Store connection (TODO: implement proper connection management)
    log::info!("WebSocket connection established: {}", connection_id);
    
    // Spawn task to handle outgoing messages
    let outgoing_task = tokio::spawn(handle_outgoing_messages(sender, rx));
    
    // Handle incoming messages
    let incoming_task = tokio::spawn(handle_incoming_messages(
        receiver,
        state.clone(),
        connection,
    ));
    
    // Wait for either task to complete
    tokio::select! {
        _ = outgoing_task => {
            log::info!("WebSocket outgoing task completed for {}", connection_id);
        }
        _ = incoming_task => {
            log::info!("WebSocket incoming task completed for {}", connection_id);
        }
    }
    
    log::info!("WebSocket connection closed: {}", connection_id);
}

async fn handle_outgoing_messages(
    mut sender: axum::extract::ws::WebSocketSender,
    mut rx: tokio::sync::mpsc::UnboundedReceiver<Message>,
) {
    while let Some(msg) = rx.recv().await {
        if sender.send(msg).await.is_err() {
            break;
        }
    }
}

async fn handle_incoming_messages(
    mut receiver: axum::extract::ws::WebSocketReceiver,
    state: Arc<AppState>,
    mut connection: WebSocketConnection,
) {
    while let Some(msg) = receiver.recv().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Err(e) = handle_text_message(&text, &mut connection, &state).await {
                    log::error!("Error handling WebSocket message: {}", e);
                    let error_msg = create_error_message(&format!("Error processing message: {}", e));
                    let _ = connection.sender.send(Message::Text(error_msg));
                }
            }
            Ok(Message::Binary(_)) => {
                let error_msg = create_error_message("Binary messages not supported");
                let _ = connection.sender.send(Message::Text(error_msg));
            }
            Ok(Message::Ping(data)) => {
                let _ = connection.sender.send(Message::Pong(data));
            }
            Ok(Message::Pong(_)) => {
                // Handle pong
            }
            Ok(Message::Close(_)) => {
                break;
            }
            Err(e) => {
                log::error!("WebSocket error: {}", e);
                break;
            }
        }
    }
}

async fn handle_text_message(
    text: &str,
    connection: &mut WebSocketConnection,
    state: &AppState,
) -> Result<(), String> {
    let msg: WebSocketMessage = serde_json::from_str(text)
        .map_err(|e| format!("Invalid JSON: {}", e))?;
    
    match msg.message_type {
        WebSocketMessageType::Subscribe => {
            handle_subscribe_message(&msg.data, connection).await
        }
        WebSocketMessageType::Unsubscribe => {
            handle_unsubscribe_message(&msg.data, connection).await
        }
        WebSocketMessageType::Ping => {
            let pong_msg = WebSocketMessage {
                message_type: WebSocketMessageType::Pong,
                data: serde_json::json!({}),
                timestamp: Utc::now(),
            };
            let pong_text = serde_json::to_string(&pong_msg)
                .map_err(|e| format!("Failed to serialize pong: {}", e))?;
            connection.sender.send(Message::Text(pong_text))
                .map_err(|_| "Failed to send pong".to_string())?;
            Ok(())
        }
        _ => Err("Invalid message type from client".to_string()),
    }
}

async fn handle_subscribe_message(
    data: &serde_json::Value,
    connection: &mut WebSocketConnection,
) -> Result<(), String> {
    let subscribe: SubscribeMessage = serde_json::from_value(data.clone())
        .map_err(|e| format!("Invalid subscribe message: {}", e))?;
    
    let subscription_key = format!(
        "{}:{}",
        subscribe.flow_id.as_deref().unwrap_or("*"),
        subscribe.execution_id.as_deref().unwrap_or("*")
    );
    
    connection.subscriptions.insert(subscription_key.clone(), subscribe);
    
    log::info!("Client {} subscribed to {}", connection.id, subscription_key);
    
    // Send confirmation
    let confirmation = WebSocketMessage {
        message_type: WebSocketMessageType::ExecutionProgress,
        data: serde_json::json!({
            "subscribed": true,
            "subscription_key": subscription_key
        }),
        timestamp: Utc::now(),
    };
    
    let confirmation_text = serde_json::to_string(&confirmation)
        .map_err(|e| format!("Failed to serialize confirmation: {}", e))?;
    
    connection.sender.send(Message::Text(confirmation_text))
        .map_err(|_| "Failed to send confirmation".to_string())?;
    
    Ok(())
}

async fn handle_unsubscribe_message(
    data: &serde_json::Value,
    connection: &mut WebSocketConnection,
) -> Result<(), String> {
    let subscribe: SubscribeMessage = serde_json::from_value(data.clone())
        .map_err(|e| format!("Invalid unsubscribe message: {}", e))?;
    
    let subscription_key = format!(
        "{}:{}",
        subscribe.flow_id.as_deref().unwrap_or("*"),
        subscribe.execution_id.as_deref().unwrap_or("*")
    );
    
    connection.subscriptions.remove(&subscription_key);
    
    log::info!("Client {} unsubscribed from {}", connection.id, subscription_key);
    
    Ok(())
}

fn create_error_message(error: &str) -> String {
    let error_msg = WebSocketMessage {
        message_type: WebSocketMessageType::Error,
        data: serde_json::json!({
            "error": error
        }),
        timestamp: Utc::now(),
    };
    
    serde_json::to_string(&error_msg).unwrap_or_else(|_| {
        format!(r#"{{"type":"error","data":{{"error":"{}","timestamp":"{}"}}}}"#, error, Utc::now().to_rfc3339())
    })
}

// Public functions for broadcasting events

pub async fn broadcast_execution_event(
    state: &AppState,
    event: ExecutionEvent,
) {
    let message = WebSocketMessage {
        message_type: match event.status {
            ExecutionStatus::Running => WebSocketMessageType::ExecutionStarted,
            ExecutionStatus::Completed => WebSocketMessageType::ExecutionCompleted,
            ExecutionStatus::Failed => WebSocketMessageType::ExecutionFailed,
            _ => WebSocketMessageType::ExecutionProgress,
        },
        data: serde_json::to_value(event).unwrap_or_default(),
        timestamp: Utc::now(),
    };
    
    // TODO: Implement actual broadcasting to connected clients
    log::info!("Broadcasting execution event: {:?}", message.message_type);
}

pub async fn broadcast_node_event(
    state: &AppState,
    event: NodeEvent,
) {
    let message = WebSocketMessage {
        message_type: match event.status {
            NodeExecutionStatus::Started => WebSocketMessageType::NodeStarted,
            NodeExecutionStatus::Completed => WebSocketMessageType::NodeCompleted,
            NodeExecutionStatus::Failed => WebSocketMessageType::NodeFailed,
            NodeExecutionStatus::Skipped => WebSocketMessageType::NodeCompleted,
        },
        data: serde_json::to_value(event).unwrap_or_default(),
        timestamp: Utc::now(),
    };
    
    // TODO: Implement actual broadcasting to connected clients
    log::info!("Broadcasting node event: {:?}", message.message_type);
}

pub async fn broadcast_flow_update(
    state: &AppState,
    event: FlowUpdateEvent,
) {
    let message = WebSocketMessage {
        message_type: WebSocketMessageType::FlowUpdated,
        data: serde_json::to_value(event).unwrap_or_default(),
        timestamp: Utc::now(),
    };
    
    // TODO: Implement actual broadcasting to connected clients
    log::info!("Broadcasting flow update: {:?}", message.message_type);
}