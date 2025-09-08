use async_trait::async_trait;
use ghostflow_core::{GhostFlowError, Node, Result};
use ghostflow_schema::{
    DataType, ExecutionContext, NodeCategory, NodeDefinition, NodeParameter, NodePort,
};
use ghostflow_schema::node::ParameterType;
use serde_json::Value;
use tracing::info;

pub struct WebhookTriggerNode;

impl WebhookTriggerNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WebhookTriggerNode {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Node for WebhookTriggerNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            id: "webhook_trigger".to_string(),
            name: "Webhook Trigger".to_string(),
            description: "Receives HTTP webhook requests to trigger flow execution".to_string(),
            category: NodeCategory::Trigger,
            version: "1.0.0".to_string(),
            inputs: vec![], // Triggers don't have inputs
            outputs: vec![NodePort {
                name: "webhook_data".to_string(),
                display_name: "Webhook Data".to_string(),
                description: Some("Data received from the webhook request".to_string()),
                data_type: DataType::Object,
                required: true,
            }],
            parameters: vec![
                NodeParameter {
                    name: "path".to_string(),
                    display_name: "Webhook Path".to_string(),
                    description: Some("URL path for the webhook endpoint".to_string()),
                    param_type: ParameterType::String,
                    default_value: Some(Value::String("/webhook".to_string())),
                    required: true,
                    options: None,
                    validation: None,
                },
                NodeParameter {
                    name: "methods".to_string(),
                    display_name: "HTTP Methods".to_string(),
                    description: Some("Allowed HTTP methods".to_string()),
                    param_type: ParameterType::MultiSelect,
                    default_value: Some(Value::Array(vec![Value::String("POST".to_string())])),
                    required: false,
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
                    name: "authentication".to_string(),
                    display_name: "Authentication".to_string(),
                    description: Some("Webhook authentication method".to_string()),
                    param_type: ParameterType::Select,
                    default_value: Some(Value::String("none".to_string())),
                    required: false,
                    options: Some(vec![
                        serde_json::from_str(r#"{"value": "none", "label": "None"}"#).unwrap(),
                        serde_json::from_str(r#"{"value": "header", "label": "Header Token"}"#).unwrap(),
                        serde_json::from_str(r#"{"value": "hmac", "label": "HMAC Signature"}"#).unwrap(),
                    ]),
                    validation: None,
                },
                NodeParameter {
                    name: "secret".to_string(),
                    display_name: "Webhook Secret".to_string(),
                    description: Some("Secret for HMAC validation or header authentication".to_string()),
                    param_type: ParameterType::Secret,
                    default_value: None,
                    required: false,
                    options: None,
                    validation: None,
                },
            ],
            icon: Some("webhook".to_string()),
            color: Some("#f97316".to_string()),
        }
    }

    async fn validate(&self, context: &ExecutionContext) -> Result<()> {
        let params = &context.input;
        
        // Validate path
        if let Some(path_value) = params.get("path") {
            if let Some(path_str) = path_value.as_str() {
                if path_str.is_empty() || !path_str.starts_with('/') {
                    return Err(GhostFlowError::ValidationError {
                        message: "Webhook path must start with '/'".to_string(),
                    });
                }
            } else {
                return Err(GhostFlowError::ValidationError {
                    message: "Webhook path must be a string".to_string(),
                });
            }
        } else {
            return Err(GhostFlowError::ValidationError {
                message: "Webhook path is required".to_string(),
            });
        }

        Ok(())
    }

    async fn execute(&self, context: ExecutionContext) -> Result<serde_json::Value> {
        // For webhook triggers, the execution context should already contain
        // the webhook data from the HTTP request
        
        let webhook_data = context.input.clone();
        
        info!("Processing webhook trigger with data");

        // Return the webhook data as-is for downstream nodes
        Ok(webhook_data)
    }

    fn supports_retry(&self) -> bool {
        false // Triggers typically don't retry
    }

    fn is_deterministic(&self) -> bool {
        false // Webhook data can vary
    }
}