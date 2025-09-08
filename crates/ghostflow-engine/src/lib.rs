pub mod executor;
pub mod scheduler;
pub mod runtime;

pub use executor::*;
pub use scheduler::*;
pub use runtime::*;

#[cfg(test)]
mod tests {
    use super::*;
    use ghostflow_core::{BasicNodeRegistry, Node, NodeRegistry};
    use ghostflow_schema::*;
    use std::collections::HashMap;
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_basic_flow_execution() {
        // Create a simple mock node for testing
        let mut registry = BasicNodeRegistry::new();
        registry.register_node("test_node".to_string(), Arc::new(MockNode::new())).unwrap();
        
        let executor = FlowExecutor::new(Arc::new(registry));
        
        // Create a simple flow with one node
        let flow = Flow {
            id: Uuid::new_v4(),
            name: "Test Flow".to_string(),
            description: Some("A test flow".to_string()),
            version: "1.0.0".to_string(),
            nodes: {
                let mut nodes = HashMap::new();
                nodes.insert("node1".to_string(), FlowNode {
                    id: "node1".to_string(),
                    node_type: "test_node".to_string(),
                    name: "Test Node".to_string(),
                    description: None,
                    parameters: {
                        let mut params = HashMap::new();
                        params.insert("test_param".to_string(), serde_json::Value::String("test_value".to_string()));
                        params
                    },
                    position: NodePosition { x: 100.0, y: 100.0 },
                    retry_config: None,
                    timeout_ms: None,
                });
                nodes
            },
            edges: vec![],
            triggers: vec![],
            parameters: HashMap::new(),
            secrets: vec![],
            metadata: FlowMetadata {
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                created_by: "test".to_string(),
                tags: vec!["test".to_string()],
                category: Some("test".to_string()),
            },
        };

        let trigger = ExecutionTrigger {
            trigger_type: "manual".to_string(),
            source: None,
            metadata: HashMap::new(),
        };

        let input_data = serde_json::json!({
            "message": "Hello, GhostFlow!"
        });

        // Execute the flow
        let result = executor.execute_flow(&flow, input_data, trigger).await;
        
        assert!(result.is_ok());
        let execution = result.unwrap();
        assert_eq!(execution.status, ExecutionStatus::Completed);
        assert!(execution.output_data.is_some());
    }

    // Mock node implementation for testing
    struct MockNode;

    impl MockNode {
        fn new() -> Self {
            Self
        }
    }

    #[async_trait::async_trait]
    impl Node for MockNode {
        fn definition(&self) -> NodeDefinition {
            NodeDefinition {
                id: "test_node".to_string(),
                name: "Test Node".to_string(),
                description: "A simple test node".to_string(),
                category: NodeCategory::Action,
                version: "1.0.0".to_string(),
                inputs: vec![],
                outputs: vec![],
                parameters: vec![],
                icon: None,
                color: None,
            }
        }

        async fn validate(&self, _context: &ExecutionContext) -> ghostflow_core::Result<()> {
            Ok(())
        }

        async fn execute(&self, context: ExecutionContext) -> ghostflow_core::Result<serde_json::Value> {
            Ok(serde_json::json!({
                "node_id": context.node_id,
                "message": "Mock node executed successfully",
                "input": context.input
            }))
        }
    }
}