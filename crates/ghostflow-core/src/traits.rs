use async_trait::async_trait;
use ghostflow_schema::{ExecutionContext, NodeDefinition};
use crate::Result;
use std::collections::HashMap;
use std::sync::Arc;

#[async_trait]
pub trait Node: Send + Sync {
    fn definition(&self) -> NodeDefinition;
    
    async fn validate(&self, context: &ExecutionContext) -> Result<()>;
    
    async fn execute(&self, context: ExecutionContext) -> Result<serde_json::Value>;
    
    fn supports_retry(&self) -> bool {
        true
    }
    
    fn is_deterministic(&self) -> bool {
        true
    }
}

#[async_trait]
pub trait FlowStorage: Send + Sync {
    async fn save_flow(&self, flow: &ghostflow_schema::Flow) -> Result<()>;
    
    async fn get_flow(&self, flow_id: &uuid::Uuid) -> Result<Option<ghostflow_schema::Flow>>;
    
    async fn list_flows(&self) -> Result<Vec<ghostflow_schema::Flow>>;
    
    async fn delete_flow(&self, flow_id: &uuid::Uuid) -> Result<()>;
}

#[async_trait]
pub trait ExecutionStorage: Send + Sync {
    async fn save_execution(&self, execution: &ghostflow_schema::FlowExecution) -> Result<()>;
    
    async fn get_execution(&self, execution_id: &uuid::Uuid) -> Result<Option<ghostflow_schema::FlowExecution>>;
    
    async fn update_execution_status(
        &self, 
        execution_id: &uuid::Uuid, 
        status: ghostflow_schema::ExecutionStatus
    ) -> Result<()>;
    
    async fn list_executions(&self, flow_id: &uuid::Uuid) -> Result<Vec<ghostflow_schema::FlowExecution>>;
}

#[async_trait]
pub trait SecretsManager: Send + Sync {
    async fn get_secret(&self, key: &str) -> Result<Option<String>>;
    
    async fn set_secret(&self, key: &str, value: &str) -> Result<()>;
    
    async fn delete_secret(&self, key: &str) -> Result<()>;
    
    async fn list_secret_keys(&self) -> Result<Vec<String>>;
}

pub trait NodeRegistry: Send + Sync {
    fn register_node(&mut self, node_type: String, node: Arc<dyn Node>) -> Result<()>;
    
    fn get_node(&self, node_type: &str) -> Option<Arc<dyn Node>>;
    
    fn list_node_definitions(&self) -> Vec<NodeDefinition>;
    
    fn validate_node_type(&self, node_type: &str) -> bool;
}

pub struct BasicNodeRegistry {
    nodes: HashMap<String, Arc<dyn Node>>,
}

impl BasicNodeRegistry {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }
}

impl NodeRegistry for BasicNodeRegistry {
    fn register_node(&mut self, node_type: String, node: Arc<dyn Node>) -> Result<()> {
        self.nodes.insert(node_type, node);
        Ok(())
    }
    
    fn get_node(&self, node_type: &str) -> Option<Arc<dyn Node>> {
        self.nodes.get(node_type).cloned()
    }
    
    fn list_node_definitions(&self) -> Vec<NodeDefinition> {
        self.nodes.values().map(|node| node.definition()).collect()
    }
    
    fn validate_node_type(&self, node_type: &str) -> bool {
        self.nodes.contains_key(node_type)
    }
}

impl Default for BasicNodeRegistry {
    fn default() -> Self {
        Self::new()
    }
}