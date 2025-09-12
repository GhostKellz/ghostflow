use ghostflow_core::{Node, NodeDefinition, NodeParameter, ParameterType, Result, Value};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackMessageNode;

#[async_trait]
impl Node for SlackMessageNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            name: "slack_message".to_string(),
            display_name: "Slack Message".to_string(),
            description: "Send messages to Slack channels or users".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                NodeParameter {
                    name: "bot_token".to_string(),
                    display_name: "Bot Token".to_string(),
                    description: "Slack bot token (xoxb-...)".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "channel".to_string(),
                    display_name: "Channel".to_string(),
                    description: "Channel ID, name (#general), or user ID".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "text".to_string(),
                    display_name: "Message Text".to_string(),
                    description: "Message content".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "blocks".to_string(),
                    display_name: "Blocks".to_string(),
                    description: "Slack Block Kit blocks (JSON)".to_string(),
                    parameter_type: ParameterType::Json,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "attachments".to_string(),
                    display_name: "Attachments".to_string(),
                    description: "Message attachments (JSON)".to_string(),
                    parameter_type: ParameterType::Json,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "thread_ts".to_string(),
                    display_name: "Thread Timestamp".to_string(),
                    description: "Thread timestamp to reply to".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "username".to_string(),
                    display_name: "Username".to_string(),
                    description: "Override bot username".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "icon_emoji".to_string(),
                    display_name: "Icon Emoji".to_string(),
                    description: "Override bot icon with emoji".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
            ],
            inputs: vec![],
            outputs: vec!["result".to_string(), "message_ts".to_string()],
        }
    }

    async fn execute(
        &self,
        context: ghostflow_core::ExecutionContext,
    ) -> Result<HashMap<String, Value>> {
        let bot_token = context.get_parameter("bot_token")
            .and_then(|v| v.as_string())
            .ok_or("Bot token is required")?;
        
        let channel = context.get_parameter("channel")
            .and_then(|v| v.as_string())
            .ok_or("Channel is required")?;

        let client = reqwest::Client::new();
        let mut body = json!({
            "channel": channel
        });

        if let Some(text) = context.get_parameter("text").and_then(|v| v.as_string()) {
            body["text"] = json!(text);
        }

        if let Some(blocks) = context.get_parameter("blocks") {
            body["blocks"] = blocks.clone();
        }

        if let Some(attachments) = context.get_parameter("attachments") {
            body["attachments"] = attachments.clone();
        }

        if let Some(thread_ts) = context.get_parameter("thread_ts").and_then(|v| v.as_string()) {
            body["thread_ts"] = json!(thread_ts);
        }

        if let Some(username) = context.get_parameter("username").and_then(|v| v.as_string()) {
            body["username"] = json!(username);
        }

        if let Some(icon_emoji) = context.get_parameter("icon_emoji").and_then(|v| v.as_string()) {
            body["icon_emoji"] = json!(icon_emoji);
        }

        let response = client
            .post("https://slack.com/api/chat.postMessage")
            .header("Authorization", format!("Bearer {}", bot_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;
        
        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Object(result.clone()));
        
        if let Some(message_ts) = result.get("ts").and_then(|ts| ts.as_str()) {
            outputs.insert("message_ts".to_string(), Value::String(message_ts.to_string()));
        }
        
        Ok(outputs)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackAlertNode;

#[async_trait]
impl Node for SlackAlertNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            name: "slack_alert".to_string(),
            display_name: "Slack Alert".to_string(),
            description: "Send formatted alerts to Slack with severity levels".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                NodeParameter {
                    name: "bot_token".to_string(),
                    display_name: "Bot Token".to_string(),
                    description: "Slack bot token (xoxb-...)".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "channel".to_string(),
                    display_name: "Channel".to_string(),
                    description: "Channel ID or name for alerts".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "alert_type".to_string(),
                    display_name: "Alert Type".to_string(),
                    description: "Type of alert".to_string(),
                    parameter_type: ParameterType::Select,
                    required: true,
                    default_value: Some(Value::String("info".to_string())),
                },
                NodeParameter {
                    name: "title".to_string(),
                    display_name: "Alert Title".to_string(),
                    description: "Title of the alert".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "message".to_string(),
                    display_name: "Alert Message".to_string(),
                    description: "Detailed alert message".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "source".to_string(),
                    display_name: "Alert Source".to_string(),
                    description: "System or service that triggered the alert".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: Some(Value::String("GhostFlow".to_string())),
                },
                NodeParameter {
                    name: "metadata".to_string(),
                    display_name: "Metadata".to_string(),
                    description: "Additional metadata (JSON)".to_string(),
                    parameter_type: ParameterType::Json,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "mention_channel".to_string(),
                    display_name: "Mention Channel".to_string(),
                    description: "Mention @channel for critical alerts".to_string(),
                    parameter_type: ParameterType::Boolean,
                    required: false,
                    default_value: Some(Value::Bool(false)),
                },
            ],
            inputs: vec!["trigger".to_string()],
            outputs: vec!["result".to_string(), "message_ts".to_string()],
        }
    }

    async fn execute(
        &self,
        context: ghostflow_core::ExecutionContext,
    ) -> Result<HashMap<String, Value>> {
        let bot_token = context.get_parameter("bot_token")
            .and_then(|v| v.as_string())
            .ok_or("Bot token is required")?;
        
        let channel = context.get_parameter("channel")
            .and_then(|v| v.as_string())
            .ok_or("Channel is required")?;
        
        let alert_type = context.get_parameter("alert_type")
            .and_then(|v| v.as_string())
            .unwrap_or("info".to_string());
        
        let title = context.get_parameter("title")
            .and_then(|v| v.as_string())
            .ok_or("Alert title is required")?;
        
        let message = context.get_parameter("message")
            .and_then(|v| v.as_string())
            .ok_or("Alert message is required")?;
        
        let source = context.get_parameter("source")
            .and_then(|v| v.as_string())
            .unwrap_or("GhostFlow".to_string());
        
        let mention_channel = context.get_parameter("mention_channel")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let (color, emoji) = match alert_type.as_str() {
            "critical" => ("#FF0000", ":rotating_light:"),
            "error" => ("#FF6B6B", ":x:"),
            "warning" => ("#FFA500", ":warning:"),
            "success" => ("#00FF00", ":white_check_mark:"),
            "info" => ("#3498DB", ":information_source:"),
            _ => ("#7289DA", ":bell:"),
        };

        let mut fields = vec![
            json!({
                "title": "Source",
                "value": source,
                "short": true
            }),
            json!({
                "title": "Severity",
                "value": alert_type.to_uppercase(),
                "short": true
            }),
            json!({
                "title": "Time",
                "value": format!("<!date^{}^{{date_short_pretty}} {{time}}|{}>", 
                    chrono::Utc::now().timestamp(),
                    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")),
                "short": true
            })
        ];

        if let Some(metadata) = context.get_parameter("metadata") {
            if let Value::Object(obj) = metadata {
                for (key, value) in obj.as_object().unwrap().iter() {
                    fields.push(json!({
                        "title": key,
                        "value": value.to_string(),
                        "short": true
                    }));
                }
            }
        }

        let attachment = json!({
            "color": color,
            "title": format!("{} {}", emoji, title),
            "text": message,
            "fields": fields,
            "footer": "GhostFlow Alert System",
            "footer_icon": "https://github.com/ghostflow.png",
            "ts": chrono::Utc::now().timestamp()
        });

        let mut text = String::new();
        if alert_type == "critical" && mention_channel {
            text = "<!channel>".to_string();
        }

        let body = json!({
            "channel": channel,
            "text": if text.is_empty() { format!("{} Alert from {}", emoji, source) } else { text },
            "attachments": [attachment],
            "username": "GhostFlow Alerts"
        });

        let client = reqwest::Client::new();
        let response = client
            .post("https://slack.com/api/chat.postMessage")
            .header("Authorization", format!("Bearer {}", bot_token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;
        
        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Object(result.clone()));
        
        if let Some(message_ts) = result.get("ts").and_then(|ts| ts.as_str()) {
            outputs.insert("message_ts".to_string(), Value::String(message_ts.to_string()));
        }
        
        Ok(outputs)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackChannelNode;

#[async_trait]
impl Node for SlackChannelNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            name: "slack_channel".to_string(),
            display_name: "Slack Channel Management".to_string(),
            description: "Manage Slack channels and members".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                NodeParameter {
                    name: "bot_token".to_string(),
                    display_name: "Bot Token".to_string(),
                    description: "Slack bot token (xoxb-...)".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "operation".to_string(),
                    display_name: "Operation".to_string(),
                    description: "Channel operation to perform".to_string(),
                    parameter_type: ParameterType::Select,
                    required: true,
                    default_value: Some(Value::String("list_channels".to_string())),
                },
                NodeParameter {
                    name: "channel_id".to_string(),
                    display_name: "Channel ID".to_string(),
                    description: "Slack channel ID".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "channel_name".to_string(),
                    display_name: "Channel Name".to_string(),
                    description: "Channel name (without #)".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "purpose".to_string(),
                    display_name: "Purpose".to_string(),
                    description: "Channel purpose/description".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "is_private".to_string(),
                    display_name: "Private Channel".to_string(),
                    description: "Create as private channel".to_string(),
                    parameter_type: ParameterType::Boolean,
                    required: false,
                    default_value: Some(Value::Bool(false)),
                },
            ],
            inputs: vec![],
            outputs: vec!["result".to_string(), "channel_info".to_string()],
        }
    }

    async fn execute(
        &self,
        context: ghostflow_core::ExecutionContext,
    ) -> Result<HashMap<String, Value>> {
        let bot_token = context.get_parameter("bot_token")
            .and_then(|v| v.as_string())
            .ok_or("Bot token is required")?;
        
        let operation = context.get_parameter("operation")
            .and_then(|v| v.as_string())
            .unwrap_or("list_channels".to_string());

        let client = reqwest::Client::new();

        let result = match operation.as_str() {
            "list_channels" => {
                let response = client
                    .get("https://slack.com/api/conversations.list")
                    .header("Authorization", format!("Bearer {}", bot_token))
                    .query(&[("types", "public_channel,private_channel")])
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "create_channel" => {
                let channel_name = context.get_parameter("channel_name")
                    .and_then(|v| v.as_string())
                    .ok_or("Channel name is required for create operation")?;
                
                let is_private = context.get_parameter("is_private")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                let body = json!({
                    "name": channel_name,
                    "is_private": is_private
                });

                let response = client
                    .post("https://slack.com/api/conversations.create")
                    .header("Authorization", format!("Bearer {}", bot_token))
                    .header("Content-Type", "application/json")
                    .json(&body)
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "get_channel_info" => {
                let channel_id = context.get_parameter("channel_id")
                    .and_then(|v| v.as_string())
                    .ok_or("Channel ID is required for get info operation")?;

                let response = client
                    .get("https://slack.com/api/conversations.info")
                    .header("Authorization", format!("Bearer {}", bot_token))
                    .query(&[("channel", &channel_id)])
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "archive_channel" => {
                let channel_id = context.get_parameter("channel_id")
                    .and_then(|v| v.as_string())
                    .ok_or("Channel ID is required for archive operation")?;

                let response = client
                    .post("https://slack.com/api/conversations.archive")
                    .header("Authorization", format!("Bearer {}", bot_token))
                    .header("Content-Type", "application/json")
                    .json(&json!({
                        "channel": channel_id
                    }))
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
        
        if let Some(channel_info) = result.get("channel") {
            outputs.insert("channel_info".to_string(), channel_info.clone().into());
        }
        
        Ok(outputs)
    }
}