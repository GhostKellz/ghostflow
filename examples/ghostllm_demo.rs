use ghostflow_nodes::GhostLLMNode;
use ghostflow_core::Node;
use ghostflow_schema::ExecutionContext;
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🚀 GhostLLM Integration Demo");
    println!("============================");

    // Create a GhostLLM node
    let node = GhostLLMNode::new();
    
    // Display node definition
    let definition = node.definition();
    println!("📋 Node Definition:");
    println!("   ID: {}", definition.id);
    println!("   Name: {}", definition.name);
    println!("   Description: {}", definition.description);
    println!("   Category: {:?}", definition.category);
    println!("   Inputs: {} ports", definition.inputs.len());
    println!("   Outputs: {} ports", definition.outputs.len());
    println!("   Parameters: {} options", definition.parameters.len());
    println!();

    // Test basic functionality with demo model
    let test_input = json!({
        "prompt": "Hello, world! Please respond with a friendly greeting.",
        "model_path": "/tmp/demo_model.gguf", // This will use the stub
        "temperature": 0.7,
        "max_tokens": 50,
        "streaming": false
    });

    let context = ExecutionContext {
        node_id: "test_ghostllm_001".to_string(),
        flow_id: "demo_flow".to_string(),
        execution_id: "demo_execution".to_string(),
        input: test_input,
        variables: HashMap::new(),
        metadata: HashMap::new(),
    };

    println!("🔍 Validating input parameters...");
    match node.validate(&context).await {
        Ok(()) => println!("✅ Validation passed"),
        Err(e) => {
            println!("❌ Validation failed: {}", e);
            return Ok(());
        }
    }

    println!("⚡ Executing GhostLLM generation...");
    println!("   Prompt: \"{}\"", context.input["prompt"].as_str().unwrap());
    println!("   Model: {}", context.input["model_path"].as_str().unwrap());
    println!();

    let start_time = std::time::Instant::now();
    
    match node.execute(context).await {
        Ok(response) => {
            let duration = start_time.elapsed();
            
            println!("🎉 Generation completed successfully!");
            println!("   Duration: {:.2}ms", duration.as_millis());
            println!();
            
            // Parse and display the response
            if let Some(text) = response.get("text").and_then(|v| v.as_str()) {
                println!("💬 Generated Response:");
                println!("   {}", text);
                println!();
            }
            
            if let Some(metadata) = response.get("metadata").and_then(|v| v.as_object()) {
                println!("📊 Metadata:");
                for (key, value) in metadata {
                    match key.as_str() {
                        "tokens_used" => println!("   Tokens Used: {}", value),
                        "generation_time_ms" => println!("   Generation Time: {}ms", value),
                        "tokens_per_second" => println!("   Tokens/sec: {:.1}", value.as_f64().unwrap_or(0.0)),
                        "temperature" => println!("   Temperature: {}", value),
                        "max_tokens" => println!("   Max Tokens: {}", value),
                        "engine" => println!("   Engine: {}", value.as_str().unwrap_or("unknown")),
                        _ => println!("   {}: {}", key, value),
                    }
                }
                println!();
            }
            
            println!("🔧 Technical Details:");
            println!("   Node supports retry: {}", node.supports_retry());
            println!("   Node is deterministic: {}", node.is_deterministic());
            println!("   FFI Status: Using Zig {} with C stub (install full GhostLLM for GPU acceleration)", 
                     std::env::var("ZIG_VERSION").unwrap_or_else(|_| "0.16+".to_string()));
            println!();
            
            // Display the full response JSON (formatted)
            println!("📄 Full Response JSON:");
            println!("{}", serde_json::to_string_pretty(&response)?);
        }
        Err(e) => {
            println!("❌ Generation failed: {}", e);
            println!("   This is expected if running with the C stub implementation.");
            println!("   Install full GhostLLM for actual AI inference.");
        }
    }

    println!();
    println!("✨ Demo completed!");
    println!("💡 To use real AI inference:");
    println!("   1. Install full GhostLLM from https://github.com/ghostkellz/ghostllm");
    println!("   2. Download a GGUF model file");
    println!("   3. Set GHOSTLLM_MODEL_PATH environment variable");
    println!("   4. Ensure GPU drivers are installed for acceleration");

    Ok(())
}