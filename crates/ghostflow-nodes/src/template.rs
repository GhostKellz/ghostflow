use async_trait::async_trait;
use ghostflow_core::{GhostFlowError, Node, Result};
use ghostflow_schema::{
    DataType, ExecutionContext, NodeCategory, NodeDefinition, NodeParameter, NodePort,
};
use ghostflow_schema::node::ParameterType;
use serde_json::Value;
use tracing::info;

pub struct TemplateNode;

impl TemplateNode {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TemplateNode {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Node for TemplateNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            id: "template".to_string(),
            name: "Template".to_string(),
            description: "Process template strings with variable substitution".to_string(),
            category: NodeCategory::Transform,
            version: "1.0.0".to_string(),
            inputs: vec![NodePort {
                name: "data".to_string(),
                display_name: "Data".to_string(),
                description: Some("Input data for template variables".to_string()),
                data_type: DataType::Object,
                required: true,
            }],
            outputs: vec![NodePort {
                name: "result".to_string(),
                display_name: "Result".to_string(),
                description: Some("Processed template result".to_string()),
                data_type: DataType::String,
                required: true,
            }],
            parameters: vec![
                NodeParameter {
                    name: "template".to_string(),
                    display_name: "Template".to_string(),
                    description: Some("Template string with {{variable}} placeholders".to_string()),
                    param_type: ParameterType::String,
                    default_value: Some(Value::String("Hello {{name}}!".to_string())),
                    required: true,
                    options: None,
                    validation: None,
                },
                NodeParameter {
                    name: "output_format".to_string(),
                    display_name: "Output Format".to_string(),
                    description: Some("Format of the output".to_string()),
                    param_type: ParameterType::Select,
                    default_value: Some(Value::String("string".to_string())),
                    required: false,
                    options: Some(vec![
                        serde_json::from_str(r#"{"value": "string", "label": "String"}"#).unwrap(),
                        serde_json::from_str(r#"{"value": "json", "label": "JSON"}"#).unwrap(),
                    ]),
                    validation: None,
                },
            ],
            icon: Some("file-text".to_string()),
            color: Some("#10b981".to_string()),
        }
    }

    async fn validate(&self, context: &ExecutionContext) -> Result<()> {
        let params = &context.input;
        
        if params.get("template").is_none() {
            return Err(GhostFlowError::ValidationError {
                message: "Template parameter is required".to_string(),
            });
        }

        Ok(())
    }

    async fn execute(&self, context: ExecutionContext) -> Result<serde_json::Value> {
        let params = &context.input;
        
        let template = params
            .get("template")
            .and_then(|v| v.as_str())
            .ok_or_else(|| GhostFlowError::NodeExecutionError {
                node_id: context.node_id.clone(),
                message: "Missing or invalid template parameter".to_string(),
            })?;

        let data = params.get("data").cloned().unwrap_or(Value::Object(serde_json::Map::new()));
        
        let output_format = params
            .get("output_format")
            .and_then(|v| v.as_str())
            .unwrap_or("string");

        info!("Processing template with {} format", output_format);

        // Process the template
        let result = self.process_template(template, &data)?;

        let output = match output_format {
            "json" => {
                // Try to parse the result as JSON
                match serde_json::from_str::<Value>(&result) {
                    Ok(json_value) => json_value,
                    Err(_) => Value::String(result), // Fallback to string if not valid JSON
                }
            }
            _ => Value::String(result),
        };

        Ok(output)
    }

    fn supports_retry(&self) -> bool {
        false
    }

    fn is_deterministic(&self) -> bool {
        true
    }
}

impl TemplateNode {
    fn process_template(&self, template: &str, data: &Value) -> Result<String> {
        let mut result = template.to_string();
        
        // Simple template processing - replace {{variable}} with values from data
        // In a real implementation, you'd use a proper template engine like Handlebars or Tera
        
        if let Some(data_obj) = data.as_object() {
            for (key, value) in data_obj {
                let placeholder = format!("{{{{{}}}}}", key);
                let replacement = self.value_to_string(value);
                result = result.replace(&placeholder, &replacement);
            }
        }
        
        // Handle nested access like {{user.name}} - very basic implementation
        // TODO: Implement proper nested object access
        
        Ok(result)
    }
    
    fn value_to_string(&self, value: &Value) -> String {
        match value {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => "null".to_string(),
            Value::Array(_) | Value::Object(_) => serde_json::to_string(value).unwrap_or_else(|_| "{}".to_string()),
        }
    }
}