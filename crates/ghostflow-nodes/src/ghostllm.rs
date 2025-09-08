use async_trait::async_trait;
use ghostflow_core::{GhostFlowError, Node, Result};
use ghostflow_schema::{
    DataType, ExecutionContext, NodeCategory, NodeDefinition, NodeParameter, NodePort,
};
use ghostflow_schema::node::ParameterType;
use ghostllm_sys::{GhostLLM, GhostConfig, GhostLLMError};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info};

/// Configuration for the GhostLLM node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhostLLMNodeConfig {
    pub model_path: String,
    pub default_temperature: f32,
    pub default_max_tokens: u32,
}

impl Default for GhostLLMNodeConfig {
    fn default() -> Self {
        Self {
            model_path: std::env::var("GHOSTLLM_MODEL_PATH")
                .unwrap_or_else(|_| "/models/default.gguf".to_string()),
            default_temperature: 0.7,
            default_max_tokens: 2048,
        }
    }
}

/// GhostLLM node for GPU-accelerated AI inference
pub struct GhostLLMNode {
    llm: Arc<Mutex<Option<GhostLLM>>>,
    config: GhostLLMNodeConfig,
}

impl GhostLLMNode {
    pub fn new() -> Self {
        Self {
            llm: Arc::new(Mutex::new(None)),
            config: GhostLLMNodeConfig::default(),
        }
    }

    pub fn with_config(config: GhostLLMNodeConfig) -> Self {
        Self {
            llm: Arc::new(Mutex::new(None)),
            config,
        }
    }

    /// Initialize the GhostLLM instance if not already done
    async fn ensure_initialized(&self, model_path: &str) -> Result<()> {
        let mut llm_guard = self.llm.lock().await;
        
        if llm_guard.is_none() {
            info!("Initializing GhostLLM with model: {}", model_path);
            
            match GhostLLM::new(model_path) {
                Ok(llm) => {
                    *llm_guard = Some(llm);
                    info!("GhostLLM initialized successfully");
                }
                Err(GhostLLMError::InitializationFailed) => {
                    error!("Failed to initialize GhostLLM - check model path and dependencies");
                    return Err(GhostFlowError::NodeExecutionError {
                        node_id: "ghostllm".to_string(),
                        message: "Failed to initialize GhostLLM. Check model path and ensure Zig/GhostLLM dependencies are properly installed.".to_string(),
                    });
                }
                Err(e) => {
                    error!("GhostLLM initialization error: {}", e);
                    return Err(GhostFlowError::NodeExecutionError {
                        node_id: "ghostllm".to_string(),
                        message: format!("GhostLLM initialization failed: {}", e),
                    });
                }
            }
        }
        
        Ok(())
    }
}

impl Default for GhostLLMNode {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Node for GhostLLMNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            id: "ghostllm_generate".to_string(),
            name: "GhostLLM Generate".to_string(),
            description: "GPU-accelerated AI text generation using GhostLLM (4x faster performance)".to_string(),
            category: NodeCategory::Ai,
            version: "1.0.0".to_string(),
            inputs: vec![NodePort {
                name: "prompt".to_string(),
                display_name: "Prompt".to_string(),
                description: Some("Input prompt for the AI model".to_string()),
                data_type: DataType::String,
                required: true,
            }],
            outputs: vec![NodePort {
                name: "response".to_string(),
                display_name: "Response".to_string(),
                description: Some("AI generated response".to_string()),
                data_type: DataType::Object,
                required: true,
            }],
            parameters: vec![
                NodeParameter {
                    name: "model_path".to_string(),
                    display_name: "Model Path".to_string(),
                    description: Some("Path to the GGUF model file (e.g., /models/llama2.gguf)".to_string()),
                    param_type: ParameterType::String,
                    default_value: Some(Value::String(self.config.model_path.clone())),
                    required: true,
                    options: None,
                    validation: None,
                },
                NodeParameter {
                    name: "temperature".to_string(),
                    display_name: "Temperature".to_string(),
                    description: Some("Sampling temperature - higher values = more creative (0.0 to 2.0)".to_string()),
                    param_type: ParameterType::Number,
                    default_value: Some(Value::Number(
                        serde_json::Number::from_f64(self.config.default_temperature as f64).unwrap()
                    )),
                    required: false,
                    options: None,
                    validation: None,
                },
                NodeParameter {
                    name: "max_tokens".to_string(),
                    display_name: "Max Tokens".to_string(),
                    description: Some("Maximum number of tokens to generate (1 to 32768)".to_string()),
                    param_type: ParameterType::Number,
                    default_value: Some(Value::Number(serde_json::Number::from(self.config.default_max_tokens))),
                    required: false,
                    options: None,
                    validation: None,
                },
                NodeParameter {
                    name: "streaming".to_string(),
                    display_name: "Enable Streaming".to_string(),
                    description: Some("Enable streaming output for real-time token generation".to_string()),
                    param_type: ParameterType::Boolean,
                    default_value: Some(Value::Bool(false)),
                    required: false,
                    options: None,
                    validation: None,
                },
            ],
            icon: Some("zap".to_string()), // Lightning bolt for speed
            color: Some("#10b981".to_string()), // Green for GhostLLM
        }
    }

    async fn validate(&self, context: &ExecutionContext) -> Result<()> {
        let params = &context.input;
        
        // Validate prompt
        if params.get("prompt").and_then(|v| v.as_str()).map(|s| s.is_empty()).unwrap_or(true) {
            return Err(GhostFlowError::ValidationError {
                message: "Prompt parameter is required and cannot be empty".to_string(),
            });
        }

        // Validate model path
        if let Some(model_path) = params.get("model_path").and_then(|v| v.as_str()) {
            if model_path.is_empty() {
                return Err(GhostFlowError::ValidationError {
                    message: "Model path cannot be empty".to_string(),
                });
            }
        }

        // Validate temperature
        if let Some(temp) = params.get("temperature").and_then(|v| v.as_f64()) {
            if temp < 0.0 || temp > 2.0 {
                return Err(GhostFlowError::ValidationError {
                    message: "Temperature must be between 0.0 and 2.0".to_string(),
                });
            }
        }

        // Validate max_tokens
        if let Some(max_tokens) = params.get("max_tokens").and_then(|v| v.as_u64()) {
            if max_tokens == 0 || max_tokens > 32768 {
                return Err(GhostFlowError::ValidationError {
                    message: "Max tokens must be between 1 and 32768".to_string(),
                });
            }
        }

        Ok(())
    }

    async fn execute(&self, context: ExecutionContext) -> Result<serde_json::Value> {
        let params = &context.input;
        
        let prompt = params
            .get("prompt")
            .and_then(|v| v.as_str())
            .ok_or_else(|| GhostFlowError::NodeExecutionError {
                node_id: context.node_id.clone(),
                message: "Missing prompt parameter".to_string(),
            })?;

        let model_path = params
            .get("model_path")
            .and_then(|v| v.as_str())
            .unwrap_or(&self.config.model_path);

        let temperature = params
            .get("temperature")
            .and_then(|v| v.as_f64())
            .unwrap_or(self.config.default_temperature as f64) as f32;

        let max_tokens = params
            .get("max_tokens")
            .and_then(|v| v.as_u64())
            .unwrap_or(self.config.default_max_tokens as u64) as u32;

        let enable_streaming = params
            .get("streaming")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        // Ensure GhostLLM is initialized
        self.ensure_initialized(model_path).await?;

        info!(
            "Generating text with GhostLLM - temperature: {}, max_tokens: {}, streaming: {}",
            temperature, max_tokens, enable_streaming
        );

        let llm_guard = self.llm.lock().await;
        let _llm = llm_guard.as_ref().ok_or_else(|| GhostFlowError::NodeExecutionError {
            node_id: context.node_id.clone(),
            message: "GhostLLM not initialized".to_string(),
        })?;

        // Update configuration
        let config = GhostConfig {
            max_tokens,
            temperature,
        };

        // Create a new LLM instance with updated config for this request
        // This is a workaround since we can't easily modify the existing instance
        let request_llm = GhostLLM::with_config(model_path, config)
            .map_err(|e| GhostFlowError::NodeExecutionError {
                node_id: context.node_id.clone(),
                message: format!("Failed to configure GhostLLM: {}", e),
            })?;

        let start_time = std::time::Instant::now();

        let response = if enable_streaming {
            // Use streaming generation
            let mut tokens = Vec::new();
            
            request_llm.generate_stream(prompt, move |token| {
                tokens.push(token.to_string());
                // In a real implementation, you might want to send these tokens
                // to a WebSocket or other streaming endpoint
            }).map_err(|e| {
                error!("GhostLLM generation failed: {}", e);
                GhostFlowError::NodeExecutionError {
                    node_id: context.node_id.clone(),
                    message: format!("Text generation failed: {}", e),
                }
            })?
        } else {
            // Standard generation
            request_llm.generate(prompt).map_err(|e| {
                error!("GhostLLM generation failed: {}", e);
                GhostFlowError::NodeExecutionError {
                    node_id: context.node_id.clone(),
                    message: format!("Text generation failed: {}", e),
                }
            })?
        };

        let generation_time = start_time.elapsed();

        info!(
            "GhostLLM generation completed in {:.2}s - {} tokens",
            generation_time.as_secs_f64(),
            response.tokens_used
        );

        Ok(serde_json::json!({
            "text": response.text,
            "tokens_used": response.tokens_used,
            "prompt": prompt,
            "metadata": {
                "model_path": model_path,
                "temperature": temperature,
                "max_tokens": max_tokens,
                "streaming_enabled": enable_streaming,
                "generation_time_ms": generation_time.as_millis(),
                "tokens_per_second": if generation_time.as_secs_f64() > 0.0 {
                    response.tokens_used as f64 / generation_time.as_secs_f64()
                } else {
                    0.0
                },
                "engine": "GhostLLM"
            }
        }))
    }

    fn supports_retry(&self) -> bool {
        true
    }

    fn is_deterministic(&self) -> bool {
        false // LLM outputs are non-deterministic
    }
}