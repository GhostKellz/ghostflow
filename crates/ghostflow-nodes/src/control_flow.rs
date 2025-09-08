use async_trait::async_trait;
use ghostflow_core::{GhostFlowError, Node, Result};
use ghostflow_schema::{
    DataType, ExecutionContext, NodeCategory, NodeDefinition, NodeParameter, NodePort,
};
use ghostflow_schema::node::ParameterType;
use serde_json::Value;
use tracing::info;

pub struct IfNode;

impl IfNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for IfNode {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Node for IfNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            id: "if".to_string(),
            name: "If".to_string(),
            description: "Conditional execution based on expression evaluation".to_string(),
            category: NodeCategory::ControlFlow,
            version: "1.0.0".to_string(),
            inputs: vec![NodePort {
                name: "input".to_string(),
                display_name: "Input".to_string(),
                description: Some("Input data to evaluate".to_string()),
                data_type: DataType::Any,
                required: true,
            }],
            outputs: vec![
                NodePort {
                    name: "true".to_string(),
                    display_name: "True".to_string(),
                    description: Some("Output when condition is true".to_string()),
                    data_type: DataType::Any,
                    required: false,
                },
                NodePort {
                    name: "false".to_string(),
                    display_name: "False".to_string(),
                    description: Some("Output when condition is false".to_string()),
                    data_type: DataType::Any,
                    required: false,
                },
            ],
            parameters: vec![
                NodeParameter {
                    name: "condition".to_string(),
                    display_name: "Condition".to_string(),
                    description: Some("Boolean expression to evaluate".to_string()),
                    param_type: ParameterType::String,
                    default_value: Some(Value::String("$.value > 0".to_string())),
                    required: true,
                    options: None,
                    validation: None,
                },
                NodeParameter {
                    name: "true_value".to_string(),
                    display_name: "True Value".to_string(),
                    description: Some("Value to return when condition is true".to_string()),
                    param_type: ParameterType::Object,
                    default_value: None,
                    required: false,
                    options: None,
                    validation: None,
                },
                NodeParameter {
                    name: "false_value".to_string(),
                    display_name: "False Value".to_string(),
                    description: Some("Value to return when condition is false".to_string()),
                    param_type: ParameterType::Object,
                    default_value: None,
                    required: false,
                    options: None,
                    validation: None,
                },
            ],
            icon: Some("git-branch".to_string()),
            color: Some("#7c3aed".to_string()),
        }
    }

    async fn validate(&self, context: &ExecutionContext) -> Result<()> {
        let params = &context.input;
        
        if params.get("condition").is_none() {
            return Err(GhostFlowError::ValidationError {
                message: "Condition parameter is required".to_string(),
            });
        }

        Ok(())
    }

    async fn execute(&self, context: ExecutionContext) -> Result<serde_json::Value> {
        let params = &context.input;
        
        let condition_str = params
            .get("condition")
            .and_then(|v| v.as_str())
            .ok_or_else(|| GhostFlowError::NodeExecutionError {
                node_id: context.node_id.clone(),
                message: "Missing or invalid condition parameter".to_string(),
            })?;

        // Simple condition evaluation - in a real implementation, you'd use a proper expression evaluator
        let condition_result = self.evaluate_simple_condition(condition_str, params)?;

        info!("If condition '{}' evaluated to: {}", condition_str, condition_result);

        let result = if condition_result {
            params.get("true_value").cloned().unwrap_or(Value::Bool(true))
        } else {
            params.get("false_value").cloned().unwrap_or(Value::Bool(false))
        };

        Ok(result)
    }

    fn supports_retry(&self) -> bool {
        false
    }

    fn is_deterministic(&self) -> bool {
        true
    }
}

impl IfNode {
    fn evaluate_simple_condition(&self, condition: &str, _input: &Value) -> Result<bool> {
        // Very basic condition evaluation - in a real system, use a proper expression evaluator
        // like JSONata, JMESPath, or a custom DSL
        
        match condition {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => {
                // For now, default to true for any other condition
                // TODO: Implement proper expression evaluation
                Ok(true)
            }
        }
    }
}

pub struct DelayNode;

impl DelayNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DelayNode {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Node for DelayNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            id: "delay".to_string(),
            name: "Delay".to_string(),
            description: "Wait for a specified amount of time".to_string(),
            category: NodeCategory::ControlFlow,
            version: "1.0.0".to_string(),
            inputs: vec![NodePort {
                name: "input".to_string(),
                display_name: "Input".to_string(),
                description: Some("Input data to pass through".to_string()),
                data_type: DataType::Any,
                required: false,
            }],
            outputs: vec![NodePort {
                name: "output".to_string(),
                display_name: "Output".to_string(),
                description: Some("Input data passed through after delay".to_string()),
                data_type: DataType::Any,
                required: true,
            }],
            parameters: vec![
                NodeParameter {
                    name: "duration".to_string(),
                    display_name: "Duration (seconds)".to_string(),
                    description: Some("Time to wait in seconds".to_string()),
                    param_type: ParameterType::Number,
                    default_value: Some(Value::Number(serde_json::Number::from(1))),
                    required: true,
                    options: None,
                    validation: None,
                },
            ],
            icon: Some("clock".to_string()),
            color: Some("#f59e0b".to_string()),
        }
    }

    async fn validate(&self, context: &ExecutionContext) -> Result<()> {
        let params = &context.input;
        
        if let Some(duration_value) = params.get("duration") {
            if let Some(duration) = duration_value.as_f64() {
                if duration < 0.0 || duration > 3600.0 {
                    return Err(GhostFlowError::ValidationError {
                        message: "Duration must be between 0 and 3600 seconds".to_string(),
                    });
                }
            } else {
                return Err(GhostFlowError::ValidationError {
                    message: "Duration must be a number".to_string(),
                });
            }
        } else {
            return Err(GhostFlowError::ValidationError {
                message: "Duration parameter is required".to_string(),
            });
        }

        Ok(())
    }

    async fn execute(&self, context: ExecutionContext) -> Result<serde_json::Value> {
        let params = &context.input;
        
        let duration = params
            .get("duration")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| GhostFlowError::NodeExecutionError {
                node_id: context.node_id.clone(),
                message: "Missing or invalid duration parameter".to_string(),
            })?;

        info!("Delaying execution for {} seconds", duration);

        // Sleep for the specified duration
        tokio::time::sleep(tokio::time::Duration::from_secs_f64(duration)).await;

        // Pass through the original input data
        let input_data = params.get("input").cloned().unwrap_or(Value::Null);
        
        Ok(input_data)
    }

    fn supports_retry(&self) -> bool {
        false
    }

    fn is_deterministic(&self) -> bool {
        false // Time-based, so not deterministic
    }
}