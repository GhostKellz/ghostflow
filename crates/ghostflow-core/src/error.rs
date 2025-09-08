use thiserror::Error;

#[derive(Error, Debug)]
pub enum GhostFlowError {
    #[error("Flow validation error: {message}")]
    ValidationError { message: String },
    
    #[error("Node execution error: {node_id} - {message}")]
    NodeExecutionError { node_id: String, message: String },
    
    #[error("Flow execution error: {flow_id} - {message}")]
    FlowExecutionError { flow_id: String, message: String },
    
    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Authentication error: {message}")]
    AuthenticationError { message: String },
    
    #[error("Authorization error: {message}")]
    AuthorizationError { message: String },
    
    #[error("Timeout error: operation timed out after {timeout_ms}ms")]
    TimeoutError { timeout_ms: u64 },
    
    #[error("Rate limit exceeded: {message}")]
    RateLimitError { message: String },
    
    #[error("Resource not found: {resource_type} with id {id}")]
    NotFoundError { resource_type: String, id: String },
    
    #[error("Internal error: {message}")]
    InternalError { message: String },
}

pub type Result<T> = std::result::Result<T, GhostFlowError>;