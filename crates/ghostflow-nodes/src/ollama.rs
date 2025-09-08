use async_trait::async_trait;
use ghostflow_core::{GhostFlowError, Node, Result};
use ghostflow_schema::{
    DataType, ExecutionContext, NodeCategory, NodeDefinition, NodeParameter, NodePort,
};
use ghostflow_schema::node::ParameterType;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{error, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<i32>,
    stream: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OllamaResponse {
    model: String,
    response: String,
    done: bool,
    context: Option<Vec<i32>>,
}

pub struct OllamaNode {
    client: Client,
    base_url: String,
}

impl OllamaNode {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: std::env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://localhost:11434".to_string()),
        }
    }

    pub fn with_base_url(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }
}

impl Default for OllamaNode {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Node for OllamaNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            id: "ollama_generate".to_string(),
            name: "Ollama Generate".to_string(),
            description: "Generate text using local Ollama models".to_string(),
            category: NodeCategory::Ai,
            version: "1.0.0".to_string(),
            inputs: vec![NodePort {
                name: "prompt".to_string(),
                display_name: "Prompt".to_string(),
                description: Some("Input prompt for the model".to_string()),
                data_type: DataType::String,
                required: true,
            }],
            outputs: vec![NodePort {
                name: "response".to_string(),
                display_name: "Response".to_string(),
                description: Some("Model generated response".to_string()),
                data_type: DataType::Object,
                required: true,
            }],
            parameters: vec![
                NodeParameter {
                    name: "model".to_string(),
                    display_name: "Model".to_string(),
                    description: Some("Ollama model to use (e.g., llama2, mistral, codellama)".to_string()),
                    param_type: ParameterType::String,
                    default_value: Some(Value::String("llama2".to_string())),
                    required: true,
                    options: None,
                    validation: None,
                },
                NodeParameter {
                    name: "system".to_string(),
                    display_name: "System Prompt".to_string(),
                    description: Some("System prompt to set model behavior".to_string()),
                    param_type: ParameterType::String,
                    default_value: None,
                    required: false,
                    options: None,
                    validation: None,
                },
                NodeParameter {
                    name: "temperature".to_string(),
                    display_name: "Temperature".to_string(),
                    description: Some("Sampling temperature (0.0 to 2.0)".to_string()),
                    param_type: ParameterType::Number,
                    default_value: Some(Value::Number(serde_json::Number::from_f64(0.7).unwrap())),
                    required: false,
                    options: None,
                    validation: None,
                },
                NodeParameter {
                    name: "max_tokens".to_string(),
                    display_name: "Max Tokens".to_string(),
                    description: Some("Maximum tokens to generate".to_string()),
                    param_type: ParameterType::Number,
                    default_value: Some(Value::Number(serde_json::Number::from(512))),
                    required: false,
                    options: None,
                    validation: None,
                },
            ],
            icon: Some("cpu".to_string()),
            color: Some("#8b5cf6".to_string()), // Purple for AI
        }
    }

    async fn validate(&self, context: &ExecutionContext) -> Result<()> {
        let params = &context.input;
        
        if params.get("model").and_then(|v| v.as_str()).is_none() {
            return Err(GhostFlowError::ValidationError {
                message: "Model parameter is required".to_string(),
            });
        }

        if let Some(temp) = params.get("temperature").and_then(|v| v.as_f64()) {
            if temp < 0.0 || temp > 2.0 {
                return Err(GhostFlowError::ValidationError {
                    message: "Temperature must be between 0.0 and 2.0".to_string(),
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

        let model = params
            .get("model")
            .and_then(|v| v.as_str())
            .unwrap_or("llama2");

        let system = params
            .get("system")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let temperature = params
            .get("temperature")
            .and_then(|v| v.as_f64())
            .map(|t| t as f32);

        let max_tokens = params
            .get("max_tokens")
            .and_then(|v| v.as_i64())
            .map(|t| t as i32);

        info!("Generating text with Ollama model: {}", model);

        let request = OllamaRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            system,
            temperature,
            max_tokens,
            stream: false,
        };

        let response = self.client
            .post(format!("{}/api/generate", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                error!("Ollama request failed: {}", e);
                GhostFlowError::NetworkError(e.to_string())
            })?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(GhostFlowError::NodeExecutionError {
                node_id: context.node_id,
                message: format!("Ollama API error: {}", error_text),
            });
        }

        let ollama_response: OllamaResponse = response.json().await
            .map_err(|e| GhostFlowError::SerializationError(e))?;

        Ok(serde_json::json!({
            "model": ollama_response.model,
            "response": ollama_response.response,
            "prompt": prompt,
            "metadata": {
                "temperature": temperature,
                "max_tokens": max_tokens,
                "done": ollama_response.done,
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

pub struct OllamaEmbeddingsNode {
    client: Client,
    base_url: String,
}

impl OllamaEmbeddingsNode {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: std::env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://localhost:11434".to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct EmbeddingsRequest {
    model: String,
    prompt: String,
}

#[derive(Debug, Deserialize)]
struct EmbeddingsResponse {
    embedding: Vec<f32>,
}

#[async_trait]
impl Node for OllamaEmbeddingsNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            id: "ollama_embeddings".to_string(),
            name: "Ollama Embeddings".to_string(),
            description: "Generate text embeddings using Ollama models".to_string(),
            category: NodeCategory::Ai,
            version: "1.0.0".to_string(),
            inputs: vec![NodePort {
                name: "text".to_string(),
                display_name: "Text".to_string(),
                description: Some("Text to generate embeddings for".to_string()),
                data_type: DataType::String,
                required: true,
            }],
            outputs: vec![NodePort {
                name: "embeddings".to_string(),
                display_name: "Embeddings".to_string(),
                description: Some("Vector embeddings".to_string()),
                data_type: DataType::Array,
                required: true,
            }],
            parameters: vec![
                NodeParameter {
                    name: "model".to_string(),
                    display_name: "Model".to_string(),
                    description: Some("Embedding model (e.g., nomic-embed-text)".to_string()),
                    param_type: ParameterType::String,
                    default_value: Some(Value::String("nomic-embed-text".to_string())),
                    required: true,
                    options: None,
                    validation: None,
                },
            ],
            icon: Some("layers".to_string()),
            color: Some("#8b5cf6".to_string()),
        }
    }

    async fn validate(&self, context: &ExecutionContext) -> Result<()> {
        if context.input.get("text").and_then(|v| v.as_str()).is_none() {
            return Err(GhostFlowError::ValidationError {
                message: "Text input is required".to_string(),
            });
        }
        Ok(())
    }

    async fn execute(&self, context: ExecutionContext) -> Result<serde_json::Value> {
        let text = context.input
            .get("text")
            .and_then(|v| v.as_str())
            .ok_or_else(|| GhostFlowError::NodeExecutionError {
                node_id: context.node_id.clone(),
                message: "Missing text input".to_string(),
            })?;

        let model = context.input
            .get("model")
            .and_then(|v| v.as_str())
            .unwrap_or("nomic-embed-text");

        let request = EmbeddingsRequest {
            model: model.to_string(),
            prompt: text.to_string(),
        };

        let response = self.client
            .post(format!("{}/api/embeddings", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| GhostFlowError::NetworkError(e.to_string()))?;

        let embeddings: EmbeddingsResponse = response.json().await
            .map_err(|e| GhostFlowError::SerializationError(e))?;

        Ok(serde_json::json!({
            "embeddings": embeddings.embedding,
            "model": model,
            "dimension": embeddings.embedding.len(),
        }))
    }
}

impl Default for OllamaEmbeddingsNode {
    fn default() -> Self {
        Self::new()
    }
}