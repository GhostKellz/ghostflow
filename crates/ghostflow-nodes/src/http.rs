use async_trait::async_trait;
use ghostflow_core::{GhostFlowError, Node, Result};
use ghostflow_schema::{
    DataType, ExecutionContext, NodeCategory, NodeDefinition, NodeParameter, NodePort,
    ParameterValidation,
};
use ghostflow_schema::node::ParameterType;
use reqwest::{Client, Method};
use serde_json::Value;
use std::collections::HashMap;
use tracing::{error, info};

pub struct HttpRequestNode {
    client: Client,
}

impl HttpRequestNode {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl Default for HttpRequestNode {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Node for HttpRequestNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            id: "http_request".to_string(),
            name: "HTTP Request".to_string(),
            description: "Make HTTP requests to external APIs".to_string(),
            category: NodeCategory::Action,
            version: "1.0.0".to_string(),
            inputs: vec![NodePort {
                name: "trigger".to_string(),
                display_name: "Trigger".to_string(),
                description: Some("Trigger the HTTP request".to_string()),
                data_type: DataType::Any,
                required: false,
            }],
            outputs: vec![NodePort {
                name: "response".to_string(),
                display_name: "Response".to_string(),
                description: Some("HTTP response data".to_string()),
                data_type: DataType::Object,
                required: true,
            }],
            parameters: vec![
                NodeParameter {
                    name: "method".to_string(),
                    display_name: "HTTP Method".to_string(),
                    description: Some("HTTP method to use".to_string()),
                    param_type: ParameterType::Select,
                    default_value: Some(Value::String("GET".to_string())),
                    required: true,
                    options: Some(vec![
                        serde_json::from_str(r#"{"value": "GET", "label": "GET"}"#).unwrap(),
                        serde_json::from_str(r#"{"value": "POST", "label": "POST"}"#).unwrap(),
                        serde_json::from_str(r#"{"value": "PUT", "label": "PUT"}"#).unwrap(),
                        serde_json::from_str(r#"{"value": "PATCH", "label": "PATCH"}"#).unwrap(),
                        serde_json::from_str(r#"{"value": "DELETE", "label": "DELETE"}"#).unwrap(),
                    ]),
                    validation: None,
                },
                NodeParameter {
                    name: "url".to_string(),
                    display_name: "URL".to_string(),
                    description: Some("URL to make the request to".to_string()),
                    param_type: ParameterType::String,
                    default_value: None,
                    required: true,
                    options: None,
                    validation: Some(ParameterValidation {
                        min_length: Some(1),
                        max_length: None,
                        min_value: None,
                        max_value: None,
                        pattern: Some(r"^https?://.*".to_string()),
                    }),
                },
                NodeParameter {
                    name: "headers".to_string(),
                    display_name: "Headers".to_string(),
                    description: Some("HTTP headers as JSON object".to_string()),
                    param_type: ParameterType::Object,
                    default_value: Some(Value::Object(serde_json::Map::new())),
                    required: false,
                    options: None,
                    validation: None,
                },
                NodeParameter {
                    name: "body".to_string(),
                    display_name: "Request Body".to_string(),
                    description: Some("Request body data".to_string()),
                    param_type: ParameterType::Object,
                    default_value: None,
                    required: false,
                    options: None,
                    validation: None,
                },
                NodeParameter {
                    name: "timeout".to_string(),
                    display_name: "Timeout (seconds)".to_string(),
                    description: Some("Request timeout in seconds".to_string()),
                    param_type: ParameterType::Number,
                    default_value: Some(Value::Number(serde_json::Number::from(30))),
                    required: false,
                    options: None,
                    validation: Some(ParameterValidation {
                        min_length: None,
                        max_length: None,
                        min_value: Some(1.0),
                        max_value: Some(300.0),
                        pattern: None,
                    }),
                },
            ],
            icon: Some("globe".to_string()),
            color: Some("#2563eb".to_string()),
        }
    }

    async fn validate(&self, context: &ExecutionContext) -> Result<()> {
        let params = &context.input;
        
        // Validate URL
        if let Some(url_value) = params.get("url") {
            if let Some(url_str) = url_value.as_str() {
                if url_str.is_empty() {
                    return Err(GhostFlowError::ValidationError {
                        message: "URL cannot be empty".to_string(),
                    });
                }
                
                // Basic URL validation
                if !url_str.starts_with("http://") && !url_str.starts_with("https://") {
                    return Err(GhostFlowError::ValidationError {
                        message: "URL must start with http:// or https://".to_string(),
                    });
                }
            } else {
                return Err(GhostFlowError::ValidationError {
                    message: "URL must be a string".to_string(),
                });
            }
        } else {
            return Err(GhostFlowError::ValidationError {
                message: "URL is required".to_string(),
            });
        }

        // Validate method
        if let Some(method_value) = params.get("method") {
            if let Some(method_str) = method_value.as_str() {
                match method_str {
                    "GET" | "POST" | "PUT" | "PATCH" | "DELETE" | "HEAD" | "OPTIONS" => {}
                    _ => {
                        return Err(GhostFlowError::ValidationError {
                            message: format!("Unsupported HTTP method: {}", method_str),
                        });
                    }
                }
            } else {
                return Err(GhostFlowError::ValidationError {
                    message: "HTTP method must be a string".to_string(),
                });
            }
        }

        Ok(())
    }

    async fn execute(&self, context: ExecutionContext) -> Result<serde_json::Value> {
        let params = &context.input;
        
        // Extract parameters
        let url = params
            .get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| GhostFlowError::NodeExecutionError {
                node_id: context.node_id.clone(),
                message: "Missing or invalid URL parameter".to_string(),
            })?;

        let method_str = params
            .get("method")
            .and_then(|v| v.as_str())
            .unwrap_or("GET");

        let method = Method::from_bytes(method_str.as_bytes())
            .map_err(|_| GhostFlowError::NodeExecutionError {
                node_id: context.node_id.clone(),
                message: format!("Invalid HTTP method: {}", method_str),
            })?;

        let timeout = params
            .get("timeout")
            .and_then(|v| v.as_u64())
            .unwrap_or(30);

        info!("Making {} request to {}", method, url);

        // Build request
        let mut request = self.client.request(method.clone(), url);

        // Add timeout
        request = request.timeout(std::time::Duration::from_secs(timeout));

        // Add headers
        if let Some(headers_value) = params.get("headers") {
            if let Some(headers_obj) = headers_value.as_object() {
                for (key, value) in headers_obj {
                    if let Some(value_str) = value.as_str() {
                        request = request.header(key, value_str);
                    }
                }
            }
        }

        // Add body for applicable methods
        if matches!(method, Method::POST | Method::PUT | Method::PATCH) {
            if let Some(body_value) = params.get("body") {
                request = request.json(body_value);
            }
        }

        // Execute request
        let response = request.send().await.map_err(|e| {
            error!("HTTP request failed: {}", e);
            GhostFlowError::NetworkError(e.to_string())
        })?;

        let status = response.status();
        let headers: HashMap<String, String> = response
            .headers()
            .iter()
            .map(|(name, value)| {
                (
                    name.to_string(),
                    value.to_str().unwrap_or("").to_string(),
                )
            })
            .collect();

        // Get response bytes first, then try to parse
        let body_bytes = response.bytes().await.map_err(|e| {
            error!("Failed to read response body: {}", e);
            GhostFlowError::NetworkError(e.to_string())
        })?;

        // Try to parse response body as JSON, fallback to text
        let body = match serde_json::from_slice::<serde_json::Value>(&body_bytes) {
            Ok(json) => json,
            Err(_) => {
                // Fallback to text
                match String::from_utf8(body_bytes.to_vec()) {
                    Ok(text) => Value::String(text),
                    Err(_) => {
                        // If it's not valid UTF-8, return error info
                        Value::String("<binary data>".to_string())
                    }
                }
            }
        };

        let result = serde_json::json!({
            "status": status.as_u16(),
            "statusText": status.canonical_reason().unwrap_or("Unknown"),
            "headers": headers,
            "body": body
        });

        Ok(result)
    }

    fn supports_retry(&self) -> bool {
        true
    }

    fn is_deterministic(&self) -> bool {
        false // HTTP requests can have different responses
    }
}