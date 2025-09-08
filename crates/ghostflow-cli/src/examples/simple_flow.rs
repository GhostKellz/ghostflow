use ghostflow_core::{BasicNodeRegistry, NodeRegistry};
use ghostflow_engine::FlowExecutor;
use ghostflow_nodes::{HttpRequestNode};
use ghostflow_schema::*;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Create node registry and register HTTP node
    let mut registry = BasicNodeRegistry::new();
    registry.register_node("http_request".to_string(), Arc::new(HttpRequestNode::new()))?;
    
    // Create executor
    let executor = FlowExecutor::new(Arc::new(registry));
    
    // Create a simple flow that makes an HTTP request
    let flow = Flow {
        id: Uuid::new_v4(),
        name: "Simple HTTP Flow".to_string(),
        description: Some("A flow that makes a simple HTTP request".to_string()),
        version: "1.0.0".to_string(),
        nodes: {
            let mut nodes = HashMap::new();
            nodes.insert("http_node".to_string(), FlowNode {
                id: "http_node".to_string(),
                node_type: "http_request".to_string(),
                name: "HTTP Request".to_string(),
                description: Some("Make HTTP request to httpbin.org".to_string()),
                parameters: {
                    let mut params = HashMap::new();
                    params.insert("method".to_string(), serde_json::Value::String("GET".to_string()));
                    params.insert("url".to_string(), serde_json::Value::String("https://httpbin.org/json".to_string()));
                    params.insert("timeout".to_string(), serde_json::Value::Number(serde_json::Number::from(30)));
                    params
                },
                position: NodePosition { x: 100.0, y: 100.0 },
                retry_config: None,
                timeout_ms: Some(30000),
            });
            nodes
        },
        edges: vec![],
        triggers: vec![FlowTrigger {
            id: "manual_trigger".to_string(),
            trigger_type: TriggerType::Manual,
            config: HashMap::new(),
            enabled: true,
        }],
        parameters: HashMap::new(),
        secrets: vec![],
        metadata: FlowMetadata {
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            created_by: "example".to_string(),
            tags: vec!["example".to_string(), "http".to_string()],
            category: Some("example".to_string()),
        },
    };

    let trigger = ExecutionTrigger {
        trigger_type: "manual".to_string(),
        source: Some("example".to_string()),
        metadata: HashMap::new(),
    };

    let input_data = serde_json::json!({
        "message": "Starting example flow execution"
    });

    println!("🚀 Executing flow: {}", flow.name);
    
    // Execute the flow
    let execution = executor.execute_flow(&flow, input_data, trigger).await?;
    
    println!("✅ Flow execution completed!");
    println!("   Status: {:?}", execution.status);
    println!("   Duration: {}ms", execution.execution_time_ms.unwrap_or(0));
    
    if let Some(output) = &execution.output_data {
        println!("   Output: {}", serde_json::to_string_pretty(output)?);
    }
    
    if let Some(error) = &execution.error {
        println!("❌ Error: {}", error.message);
    }

    Ok(())
}