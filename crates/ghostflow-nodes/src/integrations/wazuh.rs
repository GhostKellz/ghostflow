use ghostflow_core::{Node, NodeDefinition, NodeParameter, ParameterType, Result, Value};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WazuhApiNode;

#[async_trait]
impl Node for WazuhApiNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            name: "wazuh_api".to_string(),
            display_name: "Wazuh SIEM".to_string(),
            description: "Interact with Wazuh security monitoring platform".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                NodeParameter {
                    name: "base_url".to_string(),
                    display_name: "Wazuh API URL".to_string(),
                    description: "Wazuh manager API base URL".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: Some(Value::String("https://wazuh-manager:55000".to_string())),
                },
                NodeParameter {
                    name: "username".to_string(),
                    display_name: "Username".to_string(),
                    description: "Wazuh API username".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "password".to_string(),
                    display_name: "Password".to_string(),
                    description: "Wazuh API password".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "operation".to_string(),
                    display_name: "Operation".to_string(),
                    description: "Wazuh operation to perform".to_string(),
                    parameter_type: ParameterType::Select,
                    required: true,
                    default_value: Some(Value::String("get_agents".to_string())),
                },
                NodeParameter {
                    name: "agent_id".to_string(),
                    display_name: "Agent ID".to_string(),
                    description: "Specific agent ID for operations".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "rule_id".to_string(),
                    display_name: "Rule ID".to_string(),
                    description: "Security rule ID".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "level".to_string(),
                    display_name: "Alert Level".to_string(),
                    description: "Minimum alert level (0-15)".to_string(),
                    parameter_type: ParameterType::Number,
                    required: false,
                    default_value: Some(Value::Number(7.0)),
                },
                NodeParameter {
                    name: "limit".to_string(),
                    display_name: "Limit".to_string(),
                    description: "Maximum number of results".to_string(),
                    parameter_type: ParameterType::Number,
                    required: false,
                    default_value: Some(Value::Number(100.0)),
                },
            ],
            inputs: vec![],
            outputs: vec!["result".to_string(), "alerts".to_string()],
        }
    }

    async fn execute(
        &self,
        context: ghostflow_core::ExecutionContext,
    ) -> Result<HashMap<String, Value>> {
        let base_url = context.get_parameter("base_url")
            .and_then(|v| v.as_string())
            .ok_or("Wazuh API URL is required")?;
        
        let username = context.get_parameter("username")
            .and_then(|v| v.as_string())
            .ok_or("Username is required")?;
        
        let password = context.get_parameter("password")
            .and_then(|v| v.as_string())
            .ok_or("Password is required")?;
        
        let operation = context.get_parameter("operation")
            .and_then(|v| v.as_string())
            .unwrap_or("get_agents".to_string());

        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true) // For self-signed certs
            .build()?;

        // Authenticate and get JWT token
        let auth_response = client
            .post(&format!("{}/security/user/authenticate", base_url))
            .basic_auth(&username, Some(&password))
            .send()
            .await?;

        let auth_data: serde_json::Value = auth_response.json().await?;
        let token = auth_data["data"]["token"]
            .as_str()
            .ok_or("Failed to get authentication token")?;

        let result = match operation.as_str() {
            "get_agents" => {
                let response = client
                    .get(&format!("{}/agents", base_url))
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "get_agent_status" => {
                let agent_id = context.get_parameter("agent_id")
                    .and_then(|v| v.as_string())
                    .ok_or("Agent ID is required for get agent status operation")?;

                let response = client
                    .get(&format!("{}/agents/{}/stats/analytic", base_url, agent_id))
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "get_alerts" => {
                let level = context.get_parameter("level")
                    .and_then(|v| v.as_number())
                    .unwrap_or(7.0) as u8;
                
                let limit = context.get_parameter("limit")
                    .and_then(|v| v.as_number())
                    .unwrap_or(100.0) as u32;

                let mut params = vec![
                    ("level", level.to_string()),
                    ("limit", limit.to_string()),
                    ("sort", "-timestamp".to_string()),
                ];

                if let Some(agent_id) = context.get_parameter("agent_id").and_then(|v| v.as_string()) {
                    params.push(("agent.id", agent_id));
                }

                if let Some(rule_id) = context.get_parameter("rule_id").and_then(|v| v.as_string()) {
                    params.push(("rule.id", rule_id));
                }

                let response = client
                    .get(&format!("{}/security/alerts", base_url))
                    .header("Authorization", format!("Bearer {}", token))
                    .query(&params)
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "restart_agent" => {
                let agent_id = context.get_parameter("agent_id")
                    .and_then(|v| v.as_string())
                    .ok_or("Agent ID is required for restart agent operation")?;

                let response = client
                    .put(&format!("{}/agents/{}/restart", base_url, agent_id))
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await?;

                json!({
                    "success": response.status().is_success(),
                    "status": response.status().as_u16(),
                    "operation": "restart_agent",
                    "agent_id": agent_id
                })
            },
            "get_rules" => {
                let response = client
                    .get(&format!("{}/rules", base_url))
                    .header("Authorization", format!("Bearer {}", token))
                    .query(&[("limit", "1000")])
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "get_decoders" => {
                let response = client
                    .get(&format!("{}/decoders", base_url))
                    .header("Authorization", format!("Bearer {}", token))
                    .query(&[("limit", "1000")])
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "get_manager_info" => {
                let response = client
                    .get(&format!("{}/manager/info", base_url))
                    .header("Authorization", format!("Bearer {}", token))
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
        outputs.insert("result".to_string(), Value::Object(result.clone()));
        
        // Extract alerts if available
        if let Some(alerts) = result.get("data").and_then(|d| d.get("affected_items")) {
            outputs.insert("alerts".to_string(), Value::Array(
                alerts.as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|alert| Value::Object(alert.clone()))
                    .collect()
            ));
        }
        
        Ok(outputs)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WazuhAlertProcessorNode;

#[async_trait]
impl Node for WazuhAlertProcessorNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            name: "wazuh_alert_processor".to_string(),
            display_name: "Wazuh Alert Processor".to_string(),
            description: "Process and analyze Wazuh security alerts with intelligent filtering".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                NodeParameter {
                    name: "filter_level".to_string(),
                    display_name: "Filter Level".to_string(),
                    description: "Minimum severity level to process".to_string(),
                    parameter_type: ParameterType::Select,
                    required: false,
                    default_value: Some(Value::String("medium".to_string())),
                },
                NodeParameter {
                    name: "categories".to_string(),
                    display_name: "Alert Categories".to_string(),
                    description: "Comma-separated list of categories to include".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "exclude_agents".to_string(),
                    display_name: "Exclude Agents".to_string(),
                    description: "Comma-separated list of agent IDs to exclude".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "time_window".to_string(),
                    display_name: "Time Window".to_string(),
                    description: "Time window for alert analysis (minutes)".to_string(),
                    parameter_type: ParameterType::Number,
                    required: false,
                    default_value: Some(Value::Number(60.0)),
                },
                NodeParameter {
                    name: "enable_correlation".to_string(),
                    display_name: "Enable Correlation".to_string(),
                    description: "Enable alert correlation and pattern detection".to_string(),
                    parameter_type: ParameterType::Boolean,
                    required: false,
                    default_value: Some(Value::Bool(true)),
                },
            ],
            inputs: vec!["alerts".to_string()],
            outputs: vec!["filtered_alerts".to_string(), "high_priority".to_string(), "correlations".to_string()],
        }
    }

    async fn execute(
        &self,
        context: ghostflow_core::ExecutionContext,
    ) -> Result<HashMap<String, Value>> {
        let filter_level = context.get_parameter("filter_level")
            .and_then(|v| v.as_string())
            .unwrap_or("medium".to_string());
        
        let enable_correlation = context.get_parameter("enable_correlation")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        
        let time_window = context.get_parameter("time_window")
            .and_then(|v| v.as_number())
            .unwrap_or(60.0) as i64;

        let alerts = context.get_input("alerts")
            .and_then(|v| v.as_array())
            .ok_or("Alerts input is required")?;

        let min_level = match filter_level.as_str() {
            "low" => 3,
            "medium" => 7,
            "high" => 10,
            "critical" => 13,
            _ => 7,
        };

        let excluded_agents: Vec<String> = context.get_parameter("exclude_agents")
            .and_then(|v| v.as_string())
            .map(|s| s.split(',').map(|a| a.trim().to_string()).collect())
            .unwrap_or_default();

        let categories: Vec<String> = context.get_parameter("categories")
            .and_then(|v| v.as_string())
            .map(|s| s.split(',').map(|c| c.trim().to_string()).collect())
            .unwrap_or_default();

        let mut filtered_alerts = Vec::new();
        let mut high_priority = Vec::new();
        let mut correlations = Vec::new();

        for alert in alerts {
            if let Some(alert_obj) = alert.as_object() {
                // Filter by level
                if let Some(level) = alert_obj.get("rule").and_then(|r| r.get("level")).and_then(|l| l.as_u64()) {
                    if (level as i32) < min_level {
                        continue;
                    }
                }

                // Filter by excluded agents
                if let Some(agent_id) = alert_obj.get("agent").and_then(|a| a.get("id")).and_then(|i| i.as_str()) {
                    if excluded_agents.contains(&agent_id.to_string()) {
                        continue;
                    }
                }

                // Filter by categories
                if !categories.is_empty() {
                    if let Some(groups) = alert_obj.get("rule").and_then(|r| r.get("groups")).and_then(|g| g.as_array()) {
                        let alert_categories: Vec<String> = groups.iter()
                            .filter_map(|g| g.as_str())
                            .map(|s| s.to_string())
                            .collect();
                        
                        if !categories.iter().any(|cat| alert_categories.contains(cat)) {
                            continue;
                        }
                    }
                }

                filtered_alerts.push(alert.clone());

                // Identify high priority alerts
                if let Some(level) = alert_obj.get("rule").and_then(|r| r.get("level")).and_then(|l| l.as_u64()) {
                    if level >= 10 {
                        high_priority.push(alert.clone());
                    }
                }
            }
        }

        // Alert correlation logic
        if enable_correlation && !filtered_alerts.is_empty() {
            let mut rule_counts: HashMap<String, u32> = HashMap::new();
            let mut agent_counts: HashMap<String, u32> = HashMap::new();
            
            for alert in &filtered_alerts {
                if let Some(alert_obj) = alert.as_object() {
                    if let Some(rule_id) = alert_obj.get("rule").and_then(|r| r.get("id")).and_then(|i| i.as_str()) {
                        *rule_counts.entry(rule_id.to_string()).or_insert(0) += 1;
                    }
                    
                    if let Some(agent_id) = alert_obj.get("agent").and_then(|a| a.get("id")).and_then(|i| i.as_str()) {
                        *agent_counts.entry(agent_id.to_string()).or_insert(0) += 1;
                    }
                }
            }

            // Detect patterns
            for (rule_id, count) in rule_counts {
                if count > 5 {
                    correlations.push(Value::Object(json!({
                        "type": "rule_pattern",
                        "rule_id": rule_id,
                        "count": count,
                        "severity": if count > 20 { "high" } else { "medium" },
                        "description": format!("Rule {} triggered {} times in the last {} minutes", rule_id, count, time_window)
                    })));
                }
            }

            for (agent_id, count) in agent_counts {
                if count > 10 {
                    correlations.push(Value::Object(json!({
                        "type": "agent_pattern",
                        "agent_id": agent_id,
                        "count": count,
                        "severity": if count > 50 { "high" } else { "medium" },
                        "description": format!("Agent {} generated {} alerts in the last {} minutes", agent_id, count, time_window)
                    })));
                }
            }
        }

        let mut outputs = HashMap::new();
        outputs.insert("filtered_alerts".to_string(), Value::Array(filtered_alerts));
        outputs.insert("high_priority".to_string(), Value::Array(high_priority));
        outputs.insert("correlations".to_string(), Value::Array(correlations));
        
        Ok(outputs)
    }
}