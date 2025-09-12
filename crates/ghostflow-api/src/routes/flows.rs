use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::{AppState, ApiError, ApiResult};
use ghostflow_schema::{Flow, FlowStatus, ExecutionStatus};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFlowRequest {
    pub name: String,
    pub description: Option<String>,
    pub nodes: Vec<FlowNodeRequest>,
    pub edges: Vec<FlowEdgeRequest>,
    pub triggers: Vec<FlowTriggerRequest>,
    pub schedule: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlowNodeRequest {
    pub id: String,
    pub node_type: String,
    pub position: Position,
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlowEdgeRequest {
    pub id: String,
    pub source_node: String,
    pub source_output: String,
    pub target_node: String,
    pub target_input: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlowTriggerRequest {
    pub trigger_type: String,
    pub configuration: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateFlowRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<FlowStatus>,
    pub nodes: Option<Vec<FlowNodeRequest>>,
    pub edges: Option<Vec<FlowEdgeRequest>>,
    pub triggers: Option<Vec<FlowTriggerRequest>>,
    pub schedule: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlowListQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub status: Option<FlowStatus>,
    pub search: Option<String>,
    pub workspace_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlowResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: FlowStatus,
    pub nodes: Vec<FlowNodeResponse>,
    pub edges: Vec<FlowEdgeResponse>,
    pub triggers: Vec<FlowTriggerResponse>,
    pub schedule: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_execution: Option<ExecutionSummary>,
    pub execution_count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlowNodeResponse {
    pub id: String,
    pub node_type: String,
    pub position: Position,
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlowEdgeResponse {
    pub id: String,
    pub source_node: String,
    pub source_output: String,
    pub target_node: String,
    pub target_input: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlowTriggerResponse {
    pub trigger_type: String,
    pub configuration: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionSummary {
    pub id: String,
    pub status: ExecutionStatus,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration_ms: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlowListResponse {
    pub flows: Vec<FlowSummary>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlowSummary {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: FlowStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_execution: Option<ExecutionSummary>,
    pub node_count: u32,
    pub execution_count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateFlowResponse {
    pub valid: bool,
    pub errors: Vec<FlowValidationError>,
    pub warnings: Vec<FlowValidationWarning>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlowValidationError {
    pub node_id: Option<String>,
    pub edge_id: Option<String>,
    pub error_type: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlowValidationWarning {
    pub node_id: Option<String>,
    pub warning_type: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteFlowRequest {
    pub input_data: Option<HashMap<String, serde_json::Value>>,
    pub manual_trigger: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteFlowResponse {
    pub execution_id: String,
    pub status: ExecutionStatus,
    pub started_at: DateTime<Utc>,
}

// Flow management handlers

pub async fn list_flows(
    Query(query): Query<FlowListQuery>,
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<FlowListResponse>> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20).min(100); // Cap at 100
    
    // TODO: Implement actual database query
    // For now, return mock data
    let sample_flows = vec![
        FlowSummary {
            id: "flow_001".to_string(),
            name: "Discord Alert System".to_string(),
            description: Some("Send security alerts to Discord channels".to_string()),
            status: FlowStatus::Active,
            created_at: Utc::now() - chrono::Duration::days(1),
            updated_at: Utc::now() - chrono::Duration::hours(2),
            last_execution: Some(ExecutionSummary {
                id: "exec_001".to_string(),
                status: ExecutionStatus::Completed,
                started_at: Utc::now() - chrono::Duration::minutes(30),
                completed_at: Some(Utc::now() - chrono::Duration::minutes(29)),
                duration_ms: Some(60000),
            }),
            node_count: 5,
            execution_count: 42,
        },
        FlowSummary {
            id: "flow_002".to_string(),
            name: "Proxmox VM Monitoring".to_string(),
            description: Some("Monitor VM resources and send alerts".to_string()),
            status: FlowStatus::Active,
            created_at: Utc::now() - chrono::Duration::days(3),
            updated_at: Utc::now() - chrono::Duration::hours(1),
            last_execution: Some(ExecutionSummary {
                id: "exec_002".to_string(),
                status: ExecutionStatus::Completed,
                started_at: Utc::now() - chrono::Duration::minutes(5),
                completed_at: Some(Utc::now() - chrono::Duration::minutes(4)),
                duration_ms: Some(30000),
            }),
            node_count: 8,
            execution_count: 156,
        },
    ];
    
    let response = FlowListResponse {
        flows: sample_flows,
        total: 2,
        page,
        limit,
    };
    
    Ok(Json(response))
}

pub async fn create_flow(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateFlowRequest>,
) -> ApiResult<Json<FlowResponse>> {
    let flow_id = Uuid::new_v4().to_string();
    let now = Utc::now();
    
    // TODO: Validate flow structure
    // TODO: Save to database
    
    let response = FlowResponse {
        id: flow_id,
        name: request.name,
        description: request.description,
        status: FlowStatus::Draft,
        nodes: request.nodes.into_iter().map(|n| FlowNodeResponse {
            id: n.id,
            node_type: n.node_type,
            position: n.position,
            parameters: n.parameters,
        }).collect(),
        edges: request.edges.into_iter().map(|e| FlowEdgeResponse {
            id: e.id,
            source_node: e.source_node,
            source_output: e.source_output,
            target_node: e.target_node,
            target_input: e.target_input,
        }).collect(),
        triggers: request.triggers.into_iter().map(|t| FlowTriggerResponse {
            trigger_type: t.trigger_type,
            configuration: t.configuration,
        }).collect(),
        schedule: request.schedule,
        created_at: now,
        updated_at: now,
        last_execution: None,
        execution_count: 0,
    };
    
    Ok(Json(response))
}

pub async fn get_flow(
    Path(flow_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<FlowResponse>> {
    // TODO: Get from database
    // For now, return mock data
    if flow_id == "flow_001" {
        let response = FlowResponse {
            id: flow_id,
            name: "Discord Alert System".to_string(),
            description: Some("Send security alerts to Discord channels with severity filtering".to_string()),
            status: FlowStatus::Active,
            nodes: vec![
                FlowNodeResponse {
                    id: "node_001".to_string(),
                    node_type: "wazuh_api".to_string(),
                    position: Position { x: 100.0, y: 100.0 },
                    parameters: {
                        let mut params = HashMap::new();
                        params.insert("base_url".to_string(), serde_json::Value::String("https://wazuh:55000".to_string()));
                        params.insert("operation".to_string(), serde_json::Value::String("get_alerts".to_string()));
                        params
                    },
                },
                FlowNodeResponse {
                    id: "node_002".to_string(),
                    node_type: "discord_alert_bot".to_string(),
                    position: Position { x: 400.0, y: 100.0 },
                    parameters: {
                        let mut params = HashMap::new();
                        params.insert("webhook_url".to_string(), serde_json::Value::String("https://discord.com/api/webhooks/...".to_string()));
                        params.insert("alert_type".to_string(), serde_json::Value::String("critical".to_string()));
                        params
                    },
                },
            ],
            edges: vec![
                FlowEdgeResponse {
                    id: "edge_001".to_string(),
                    source_node: "node_001".to_string(),
                    source_output: "alerts".to_string(),
                    target_node: "node_002".to_string(),
                    target_input: "trigger".to_string(),
                },
            ],
            triggers: vec![
                FlowTriggerResponse {
                    trigger_type: "schedule".to_string(),
                    configuration: {
                        let mut config = HashMap::new();
                        config.insert("cron".to_string(), serde_json::Value::String("0 */5 * * * *".to_string()));
                        config
                    },
                },
            ],
            schedule: Some("0 */5 * * * *".to_string()),
            created_at: Utc::now() - chrono::Duration::days(1),
            updated_at: Utc::now() - chrono::Duration::hours(2),
            last_execution: Some(ExecutionSummary {
                id: "exec_001".to_string(),
                status: ExecutionStatus::Completed,
                started_at: Utc::now() - chrono::Duration::minutes(30),
                completed_at: Some(Utc::now() - chrono::Duration::minutes(29)),
                duration_ms: Some(60000),
            }),
            execution_count: 42,
        };
        
        Ok(Json(response))
    } else {
        Err(ApiError::NotFound("Flow not found".to_string()))
    }
}

pub async fn update_flow(
    Path(flow_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(request): Json<UpdateFlowRequest>,
) -> ApiResult<Json<FlowResponse>> {
    // TODO: Update in database
    // For now, return updated mock data
    
    let mut response = FlowResponse {
        id: flow_id,
        name: request.name.unwrap_or_else(|| "Updated Flow".to_string()),
        description: request.description,
        status: request.status.unwrap_or(FlowStatus::Draft),
        nodes: request.nodes.unwrap_or_default().into_iter().map(|n| FlowNodeResponse {
            id: n.id,
            node_type: n.node_type,
            position: n.position,
            parameters: n.parameters,
        }).collect(),
        edges: request.edges.unwrap_or_default().into_iter().map(|e| FlowEdgeResponse {
            id: e.id,
            source_node: e.source_node,
            source_output: e.source_output,
            target_node: e.target_node,
            target_input: e.target_input,
        }).collect(),
        triggers: request.triggers.unwrap_or_default().into_iter().map(|t| FlowTriggerResponse {
            trigger_type: t.trigger_type,
            configuration: t.configuration,
        }).collect(),
        schedule: request.schedule,
        created_at: Utc::now() - chrono::Duration::days(1),
        updated_at: Utc::now(),
        last_execution: None,
        execution_count: 0,
    };
    
    Ok(Json(response))
}

pub async fn delete_flow(
    Path(flow_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> ApiResult<StatusCode> {
    // TODO: Delete from database
    // TODO: Cancel any running executions
    
    Ok(StatusCode::NO_CONTENT)
}

pub async fn validate_flow(
    Path(flow_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<ValidateFlowResponse>> {
    // TODO: Implement actual flow validation
    // Check for circular dependencies, missing connections, invalid parameters, etc.
    
    let response = ValidateFlowResponse {
        valid: true,
        errors: vec![],
        warnings: vec![
            FlowValidationWarning {
                node_id: Some("node_001".to_string()),
                warning_type: "performance".to_string(),
                message: "This node may run slowly with large datasets".to_string(),
            },
        ],
    };
    
    Ok(Json(response))
}

pub async fn execute_flow(
    Path(flow_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(request): Json<ExecuteFlowRequest>,
) -> ApiResult<Json<ExecuteFlowResponse>> {
    let execution_id = Uuid::new_v4().to_string();
    let now = Utc::now();
    
    // TODO: Start actual flow execution
    // TODO: Store execution record in database
    // TODO: Send WebSocket notification
    
    let response = ExecuteFlowResponse {
        execution_id,
        status: ExecutionStatus::Running,
        started_at: now,
    };
    
    Ok(Json(response))
}