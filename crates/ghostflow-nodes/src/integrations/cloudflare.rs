use ghostflow_core::{Node, NodeDefinition, NodeParameter, ParameterType, Result, Value};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudflareDNSNode;

#[async_trait]
impl Node for CloudflareDNSNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            name: "cloudflare_dns".to_string(),
            display_name: "Cloudflare DNS".to_string(),
            description: "Manage Cloudflare DNS records".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                NodeParameter {
                    name: "api_token".to_string(),
                    display_name: "API Token".to_string(),
                    description: "Cloudflare API token with DNS edit permissions".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "zone_id".to_string(),
                    display_name: "Zone ID".to_string(),
                    description: "Cloudflare Zone ID".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "operation".to_string(),
                    display_name: "Operation".to_string(),
                    description: "DNS operation to perform".to_string(),
                    parameter_type: ParameterType::Select,
                    required: true,
                    default_value: Some(Value::String("list".to_string())),
                },
                NodeParameter {
                    name: "record_type".to_string(),
                    display_name: "Record Type".to_string(),
                    description: "DNS record type (A, AAAA, CNAME, MX, TXT, etc.)".to_string(),
                    parameter_type: ParameterType::Select,
                    required: false,
                    default_value: Some(Value::String("A".to_string())),
                },
                NodeParameter {
                    name: "name".to_string(),
                    display_name: "Record Name".to_string(),
                    description: "DNS record name (e.g., subdomain)".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "content".to_string(),
                    display_name: "Content".to_string(),
                    description: "Record content (IP address, domain, etc.)".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "proxied".to_string(),
                    display_name: "Proxied".to_string(),
                    description: "Enable Cloudflare proxy".to_string(),
                    parameter_type: ParameterType::Boolean,
                    required: false,
                    default_value: Some(Value::Bool(false)),
                },
                NodeParameter {
                    name: "ttl".to_string(),
                    display_name: "TTL".to_string(),
                    description: "Time to live in seconds (1 = auto)".to_string(),
                    parameter_type: ParameterType::Number,
                    required: false,
                    default_value: Some(Value::Number(1.0)),
                },
            ],
            inputs: vec![],
            outputs: vec!["result".to_string()],
        }
    }

    async fn execute(
        &self,
        context: ghostflow_core::ExecutionContext,
    ) -> Result<HashMap<String, Value>> {
        let api_token = context.get_parameter("api_token")
            .and_then(|v| v.as_string())
            .ok_or("API token is required")?;
        
        let zone_id = context.get_parameter("zone_id")
            .and_then(|v| v.as_string())
            .ok_or("Zone ID is required")?;
        
        let operation = context.get_parameter("operation")
            .and_then(|v| v.as_string())
            .unwrap_or("list".to_string());

        let client = reqwest::Client::new();
        let base_url = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records", zone_id);

        let result = match operation.as_str() {
            "list" => {
                let response = client
                    .get(&base_url)
                    .header("Authorization", format!("Bearer {}", api_token))
                    .header("Content-Type", "application/json")
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "create" => {
                let record_type = context.get_parameter("record_type")
                    .and_then(|v| v.as_string())
                    .unwrap_or("A".to_string());
                
                let name = context.get_parameter("name")
                    .and_then(|v| v.as_string())
                    .ok_or("Record name is required for create operation")?;
                
                let content = context.get_parameter("content")
                    .and_then(|v| v.as_string())
                    .ok_or("Content is required for create operation")?;
                
                let proxied = context.get_parameter("proxied")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                
                let ttl = context.get_parameter("ttl")
                    .and_then(|v| v.as_number())
                    .unwrap_or(1.0) as i64;

                let body = json!({
                    "type": record_type,
                    "name": name,
                    "content": content,
                    "proxied": proxied,
                    "ttl": ttl
                });

                let response = client
                    .post(&base_url)
                    .header("Authorization", format!("Bearer {}", api_token))
                    .header("Content-Type", "application/json")
                    .json(&body)
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "update" => {
                let record_id = context.get_parameter("record_id")
                    .and_then(|v| v.as_string())
                    .ok_or("Record ID is required for update operation")?;
                
                let mut body = json!({});
                
                if let Some(name) = context.get_parameter("name").and_then(|v| v.as_string()) {
                    body["name"] = json!(name);
                }
                if let Some(content) = context.get_parameter("content").and_then(|v| v.as_string()) {
                    body["content"] = json!(content);
                }
                if let Some(proxied) = context.get_parameter("proxied").and_then(|v| v.as_bool()) {
                    body["proxied"] = json!(proxied);
                }
                if let Some(ttl) = context.get_parameter("ttl").and_then(|v| v.as_number()) {
                    body["ttl"] = json!(ttl as i64);
                }

                let response = client
                    .patch(&format!("{}/{}", base_url, record_id))
                    .header("Authorization", format!("Bearer {}", api_token))
                    .header("Content-Type", "application/json")
                    .json(&body)
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "delete" => {
                let record_id = context.get_parameter("record_id")
                    .and_then(|v| v.as_string())
                    .ok_or("Record ID is required for delete operation")?;

                let response = client
                    .delete(&format!("{}/{}", base_url, record_id))
                    .header("Authorization", format!("Bearer {}", api_token))
                    .header("Content-Type", "application/json")
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            _ => {
                return Err(format!("Unknown operation: {}", operation).into());
            }
        };

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Object(result));
        Ok(outputs)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudflareWAFNode;

#[async_trait]
impl Node for CloudflareWAFNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            name: "cloudflare_waf".to_string(),
            display_name: "Cloudflare WAF".to_string(),
            description: "Manage Cloudflare WAF rules and firewall settings".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                NodeParameter {
                    name: "api_token".to_string(),
                    display_name: "API Token".to_string(),
                    description: "Cloudflare API token with WAF permissions".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "zone_id".to_string(),
                    display_name: "Zone ID".to_string(),
                    description: "Cloudflare Zone ID".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "operation".to_string(),
                    display_name: "Operation".to_string(),
                    description: "WAF operation to perform".to_string(),
                    parameter_type: ParameterType::Select,
                    required: true,
                    default_value: Some(Value::String("list_rules".to_string())),
                },
                NodeParameter {
                    name: "action".to_string(),
                    display_name: "Action".to_string(),
                    description: "Rule action (block, challenge, js_challenge, allow)".to_string(),
                    parameter_type: ParameterType::Select,
                    required: false,
                    default_value: Some(Value::String("block".to_string())),
                },
                NodeParameter {
                    name: "expression".to_string(),
                    display_name: "Expression".to_string(),
                    description: "WAF rule expression".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "description".to_string(),
                    display_name: "Description".to_string(),
                    description: "Rule description".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
            ],
            inputs: vec![],
            outputs: vec!["result".to_string()],
        }
    }

    async fn execute(
        &self,
        context: ghostflow_core::ExecutionContext,
    ) -> Result<HashMap<String, Value>> {
        let api_token = context.get_parameter("api_token")
            .and_then(|v| v.as_string())
            .ok_or("API token is required")?;
        
        let zone_id = context.get_parameter("zone_id")
            .and_then(|v| v.as_string())
            .ok_or("Zone ID is required")?;
        
        let operation = context.get_parameter("operation")
            .and_then(|v| v.as_string())
            .unwrap_or("list_rules".to_string());

        let client = reqwest::Client::new();
        let base_url = format!("https://api.cloudflare.com/client/v4/zones/{}/firewall/rules", zone_id);

        let result = match operation.as_str() {
            "list_rules" => {
                let response = client
                    .get(&base_url)
                    .header("Authorization", format!("Bearer {}", api_token))
                    .header("Content-Type", "application/json")
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "create_rule" => {
                let action = context.get_parameter("action")
                    .and_then(|v| v.as_string())
                    .unwrap_or("block".to_string());
                
                let expression = context.get_parameter("expression")
                    .and_then(|v| v.as_string())
                    .ok_or("Expression is required for create operation")?;
                
                let description = context.get_parameter("description")
                    .and_then(|v| v.as_string())
                    .unwrap_or("Created by GhostFlow".to_string());

                let filter_body = json!({
                    "expression": expression,
                    "description": description
                });

                let filter_response = client
                    .post(&format!("https://api.cloudflare.com/client/v4/zones/{}/filters", zone_id))
                    .header("Authorization", format!("Bearer {}", api_token))
                    .header("Content-Type", "application/json")
                    .json(&vec![filter_body])
                    .send()
                    .await?;

                let filter_data: serde_json::Value = filter_response.json().await?;
                let filter_id = filter_data["result"][0]["id"].as_str()
                    .ok_or("Failed to create filter")?;

                let rule_body = json!({
                    "filter": {
                        "id": filter_id
                    },
                    "action": action,
                    "description": description
                });

                let response = client
                    .post(&base_url)
                    .header("Authorization", format!("Bearer {}", api_token))
                    .header("Content-Type", "application/json")
                    .json(&vec![rule_body])
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            _ => {
                return Err(format!("Unknown operation: {}", operation).into());
            }
        };

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Object(result));
        Ok(outputs)
    }
}