use ghostflow_core::{Node, NodeDefinition, NodeParameter, ParameterType, Result, Value};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicrosoftGraphEmailNode;

#[async_trait]
impl Node for MicrosoftGraphEmailNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            name: "microsoft_graph_email".to_string(),
            display_name: "Microsoft 365 Email".to_string(),
            description: "Send and manage emails via Microsoft Graph API".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                NodeParameter {
                    name: "access_token".to_string(),
                    display_name: "Access Token".to_string(),
                    description: "Microsoft Graph API access token".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "operation".to_string(),
                    display_name: "Operation".to_string(),
                    description: "Email operation to perform".to_string(),
                    parameter_type: ParameterType::Select,
                    required: true,
                    default_value: Some(Value::String("send".to_string())),
                },
                NodeParameter {
                    name: "to".to_string(),
                    display_name: "To".to_string(),
                    description: "Recipient email addresses (comma-separated)".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "subject".to_string(),
                    display_name: "Subject".to_string(),
                    description: "Email subject".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "body".to_string(),
                    display_name: "Body".to_string(),
                    description: "Email body content".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "body_type".to_string(),
                    display_name: "Body Type".to_string(),
                    description: "Email body type (text or html)".to_string(),
                    parameter_type: ParameterType::Select,
                    required: false,
                    default_value: Some(Value::String("html".to_string())),
                },
                NodeParameter {
                    name: "cc".to_string(),
                    display_name: "CC".to_string(),
                    description: "CC recipients (comma-separated)".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "importance".to_string(),
                    display_name: "Importance".to_string(),
                    description: "Email importance level".to_string(),
                    parameter_type: ParameterType::Select,
                    required: false,
                    default_value: Some(Value::String("normal".to_string())),
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
        let access_token = context.get_parameter("access_token")
            .and_then(|v| v.as_string())
            .ok_or("Access token is required")?;
        
        let operation = context.get_parameter("operation")
            .and_then(|v| v.as_string())
            .unwrap_or("send".to_string());

        let client = reqwest::Client::new();
        let base_url = "https://graph.microsoft.com/v1.0";

        let result = match operation.as_str() {
            "send" => {
                let to = context.get_parameter("to")
                    .and_then(|v| v.as_string())
                    .ok_or("Recipients are required for send operation")?;
                
                let subject = context.get_parameter("subject")
                    .and_then(|v| v.as_string())
                    .ok_or("Subject is required for send operation")?;
                
                let body_content = context.get_parameter("body")
                    .and_then(|v| v.as_string())
                    .ok_or("Body is required for send operation")?;
                
                let body_type = context.get_parameter("body_type")
                    .and_then(|v| v.as_string())
                    .unwrap_or("html".to_string());

                let importance = context.get_parameter("importance")
                    .and_then(|v| v.as_string())
                    .unwrap_or("normal".to_string());

                let to_recipients: Vec<serde_json::Value> = to.split(',')
                    .map(|email| json!({
                        "emailAddress": {
                            "address": email.trim()
                        }
                    }))
                    .collect();

                let mut message = json!({
                    "message": {
                        "subject": subject,
                        "body": {
                            "contentType": body_type,
                            "content": body_content
                        },
                        "toRecipients": to_recipients,
                        "importance": importance
                    }
                });

                if let Some(cc) = context.get_parameter("cc").and_then(|v| v.as_string()) {
                    let cc_recipients: Vec<serde_json::Value> = cc.split(',')
                        .map(|email| json!({
                            "emailAddress": {
                                "address": email.trim()
                            }
                        }))
                        .collect();
                    message["message"]["ccRecipients"] = json!(cc_recipients);
                }

                let response = client
                    .post(&format!("{}/me/sendMail", base_url))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .header("Content-Type", "application/json")
                    .json(&message)
                    .send()
                    .await?;

                json!({
                    "success": response.status().is_success(),
                    "status": response.status().as_u16()
                })
            },
            "get_inbox" => {
                let response = client
                    .get(&format!("{}/me/messages", base_url))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .query(&[("$top", "20"), ("$orderby", "receivedDateTime desc")])
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "get_folders" => {
                let response = client
                    .get(&format!("{}/me/mailFolders", base_url))
                    .header("Authorization", format!("Bearer {}", access_token))
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
pub struct MicrosoftTeamsNode;

#[async_trait]
impl Node for MicrosoftTeamsNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            name: "microsoft_teams".to_string(),
            display_name: "Microsoft Teams".to_string(),
            description: "Send messages and manage Teams channels".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                NodeParameter {
                    name: "access_token".to_string(),
                    display_name: "Access Token".to_string(),
                    description: "Microsoft Graph API access token".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "operation".to_string(),
                    display_name: "Operation".to_string(),
                    description: "Teams operation to perform".to_string(),
                    parameter_type: ParameterType::Select,
                    required: true,
                    default_value: Some(Value::String("send_message".to_string())),
                },
                NodeParameter {
                    name: "team_id".to_string(),
                    display_name: "Team ID".to_string(),
                    description: "Microsoft Teams team ID".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "channel_id".to_string(),
                    display_name: "Channel ID".to_string(),
                    description: "Teams channel ID".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "message".to_string(),
                    display_name: "Message".to_string(),
                    description: "Message content to send".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "importance".to_string(),
                    display_name: "Importance".to_string(),
                    description: "Message importance".to_string(),
                    parameter_type: ParameterType::Select,
                    required: false,
                    default_value: Some(Value::String("normal".to_string())),
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
        let access_token = context.get_parameter("access_token")
            .and_then(|v| v.as_string())
            .ok_or("Access token is required")?;
        
        let operation = context.get_parameter("operation")
            .and_then(|v| v.as_string())
            .unwrap_or("send_message".to_string());

        let client = reqwest::Client::new();
        let base_url = "https://graph.microsoft.com/v1.0";

        let result = match operation.as_str() {
            "send_message" => {
                let team_id = context.get_parameter("team_id")
                    .and_then(|v| v.as_string())
                    .ok_or("Team ID is required for send message operation")?;
                
                let channel_id = context.get_parameter("channel_id")
                    .and_then(|v| v.as_string())
                    .ok_or("Channel ID is required for send message operation")?;
                
                let message = context.get_parameter("message")
                    .and_then(|v| v.as_string())
                    .ok_or("Message is required for send message operation")?;
                
                let importance = context.get_parameter("importance")
                    .and_then(|v| v.as_string())
                    .unwrap_or("normal".to_string());

                let body = json!({
                    "body": {
                        "content": message
                    },
                    "importance": importance
                });

                let response = client
                    .post(&format!("{}/teams/{}/channels/{}/messages", base_url, team_id, channel_id))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .header("Content-Type", "application/json")
                    .json(&body)
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "list_teams" => {
                let response = client
                    .get(&format!("{}/me/joinedTeams", base_url))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "list_channels" => {
                let team_id = context.get_parameter("team_id")
                    .and_then(|v| v.as_string())
                    .ok_or("Team ID is required for list channels operation")?;

                let response = client
                    .get(&format!("{}/teams/{}/channels", base_url, team_id))
                    .header("Authorization", format!("Bearer {}", access_token))
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
pub struct MicrosoftCalendarNode;

#[async_trait]
impl Node for MicrosoftCalendarNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            name: "microsoft_calendar".to_string(),
            display_name: "Microsoft 365 Calendar".to_string(),
            description: "Manage calendar events and meetings".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                NodeParameter {
                    name: "access_token".to_string(),
                    display_name: "Access Token".to_string(),
                    description: "Microsoft Graph API access token".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "operation".to_string(),
                    display_name: "Operation".to_string(),
                    description: "Calendar operation to perform".to_string(),
                    parameter_type: ParameterType::Select,
                    required: true,
                    default_value: Some(Value::String("get_events".to_string())),
                },
                NodeParameter {
                    name: "subject".to_string(),
                    display_name: "Subject".to_string(),
                    description: "Event subject".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "start_time".to_string(),
                    display_name: "Start Time".to_string(),
                    description: "Event start time (ISO 8601)".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "end_time".to_string(),
                    display_name: "End Time".to_string(),
                    description: "Event end time (ISO 8601)".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "attendees".to_string(),
                    display_name: "Attendees".to_string(),
                    description: "Attendee emails (comma-separated)".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "location".to_string(),
                    display_name: "Location".to_string(),
                    description: "Event location".to_string(),
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
        let access_token = context.get_parameter("access_token")
            .and_then(|v| v.as_string())
            .ok_or("Access token is required")?;
        
        let operation = context.get_parameter("operation")
            .and_then(|v| v.as_string())
            .unwrap_or("get_events".to_string());

        let client = reqwest::Client::new();
        let base_url = "https://graph.microsoft.com/v1.0";

        let result = match operation.as_str() {
            "get_events" => {
                let response = client
                    .get(&format!("{}/me/events", base_url))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .query(&[("$top", "20"), ("$orderby", "start/dateTime")])
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "create_event" => {
                let subject = context.get_parameter("subject")
                    .and_then(|v| v.as_string())
                    .ok_or("Subject is required for create event")?;
                
                let start_time = context.get_parameter("start_time")
                    .and_then(|v| v.as_string())
                    .ok_or("Start time is required for create event")?;
                
                let end_time = context.get_parameter("end_time")
                    .and_then(|v| v.as_string())
                    .ok_or("End time is required for create event")?;

                let mut event = json!({
                    "subject": subject,
                    "start": {
                        "dateTime": start_time,
                        "timeZone": "UTC"
                    },
                    "end": {
                        "dateTime": end_time,
                        "timeZone": "UTC"
                    }
                });

                if let Some(location) = context.get_parameter("location").and_then(|v| v.as_string()) {
                    event["location"] = json!({
                        "displayName": location
                    });
                }

                if let Some(attendees) = context.get_parameter("attendees").and_then(|v| v.as_string()) {
                    let attendee_list: Vec<serde_json::Value> = attendees.split(',')
                        .map(|email| json!({
                            "emailAddress": {
                                "address": email.trim()
                            },
                            "type": "required"
                        }))
                        .collect();
                    event["attendees"] = json!(attendee_list);
                }

                let response = client
                    .post(&format!("{}/me/events", base_url))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .header("Content-Type", "application/json")
                    .json(&event)
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