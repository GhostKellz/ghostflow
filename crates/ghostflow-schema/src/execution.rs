use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowExecution {
    pub id: Uuid,
    pub flow_id: Uuid,
    pub flow_version: String,
    pub status: ExecutionStatus,
    pub trigger: ExecutionTrigger,
    pub input_data: serde_json::Value,
    pub output_data: Option<serde_json::Value>,
    pub error: Option<ExecutionError>,
    pub node_executions: HashMap<String, NodeExecution>,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub execution_time_ms: Option<u64>,
    pub metadata: ExecutionMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
    Retrying,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionTrigger {
    pub trigger_type: String,
    pub source: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeExecution {
    pub node_id: String,
    pub status: ExecutionStatus,
    pub input_data: serde_json::Value,
    pub output_data: Option<serde_json::Value>,
    pub error: Option<ExecutionError>,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub execution_time_ms: Option<u64>,
    pub retry_count: u32,
    pub logs: Vec<ExecutionLog>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionError {
    pub error_type: ErrorType,
    pub message: String,
    pub details: Option<HashMap<String, serde_json::Value>>,
    pub retryable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorType {
    ValidationError,
    NetworkError,
    TimeoutError,
    AuthenticationError,
    AuthorizationError,
    NotFoundError,
    RateLimitError,
    InternalError,
    UserError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionLog {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub level: LogLevel,
    pub message: String,
    pub details: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetadata {
    pub executor_id: String,
    pub environment: String,
    pub correlation_id: Option<String>,
    pub trace_id: Option<String>,
    pub span_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContext {
    pub execution_id: Uuid,
    pub flow_id: Uuid,
    pub node_id: String,
    pub input: serde_json::Value,
    pub variables: HashMap<String, serde_json::Value>,
    pub secrets: HashMap<String, String>,
    pub artifacts: HashMap<String, ArtifactReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactReference {
    pub id: Uuid,
    pub name: String,
    pub content_type: String,
    pub size_bytes: u64,
    pub storage_path: String,
    pub checksum: String,
}