use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flow {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub version: String,
    pub nodes: HashMap<String, FlowNode>,
    pub edges: Vec<FlowEdge>,
    pub triggers: Vec<FlowTrigger>,
    pub parameters: HashMap<String, FlowParameter>,
    pub secrets: Vec<String>,
    pub metadata: FlowMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowNode {
    pub id: String,
    pub node_type: String,
    pub name: String,
    pub description: Option<String>,
    pub parameters: HashMap<String, serde_json::Value>,
    pub position: NodePosition,
    pub retry_config: Option<RetryConfig>,
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowEdge {
    pub id: String,
    pub source_node: String,
    pub target_node: String,
    pub source_port: Option<String>,
    pub target_port: Option<String>,
    pub condition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowTrigger {
    pub id: String,
    pub trigger_type: TriggerType,
    pub config: HashMap<String, serde_json::Value>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "config")]
pub enum TriggerType {
    #[serde(rename = "webhook")]
    Webhook { path: String, method: String },
    #[serde(rename = "cron")]
    Cron { expression: String, timezone: Option<String> },
    #[serde(rename = "manual")]
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowParameter {
    pub name: String,
    pub param_type: ParameterType,
    pub description: Option<String>,
    pub default_value: Option<serde_json::Value>,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    String,
    Number,
    Boolean,
    Object,
    Array,
    Secret,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub delay_ms: u64,
    pub backoff_multiplier: f64,
    pub max_delay_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowMetadata {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_by: String,
    pub tags: Vec<String>,
    pub category: Option<String>,
}