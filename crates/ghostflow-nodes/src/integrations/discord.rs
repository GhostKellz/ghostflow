use ghostflow_core::{Node, NodeDefinition, NodeParameter, ParameterType, Result, Value};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordWebhookNode;

#[async_trait]
impl Node for DiscordWebhookNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            name: "discord_webhook".to_string(),
            display_name: "Discord Webhook".to_string(),
            description: "Send messages to Discord via webhook".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                NodeParameter {
                    name: "webhook_url".to_string(),
                    display_name: "Webhook URL".to_string(),
                    description: "Discord webhook URL".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "content".to_string(),
                    display_name: "Message Content".to_string(),
                    description: "Text message to send".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "username".to_string(),
                    display_name: "Username".to_string(),
                    description: "Override webhook username".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: Some(Value::String("GhostFlow".to_string())),
                },
                NodeParameter {
                    name: "avatar_url".to_string(),
                    display_name: "Avatar URL".to_string(),
                    description: "Override webhook avatar".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "embed".to_string(),
                    display_name: "Embed".to_string(),
                    description: "Rich embed object (JSON)".to_string(),
                    parameter_type: ParameterType::Json,
                    required: false,
                    default_value: None,
                },
            ],
            inputs: vec!["trigger".to_string()],
            outputs: vec!["result".to_string()],
        }
    }

    async fn execute(
        &self,
        context: ghostflow_core::ExecutionContext,
    ) -> Result<HashMap<String, Value>> {
        let webhook_url = context.get_parameter("webhook_url")
            .and_then(|v| v.as_string())
            .ok_or("Webhook URL is required")?;
        
        let mut body = json!({});
        
        if let Some(content) = context.get_parameter("content").and_then(|v| v.as_string()) {
            body["content"] = json!(content);
        }
        
        if let Some(username) = context.get_parameter("username").and_then(|v| v.as_string()) {
            body["username"] = json!(username);
        }
        
        if let Some(avatar_url) = context.get_parameter("avatar_url").and_then(|v| v.as_string()) {
            body["avatar_url"] = json!(avatar_url);
        }
        
        if let Some(embed) = context.get_parameter("embed") {
            body["embeds"] = json!([embed]);
        }

        let client = reqwest::Client::new();
        let response = client
            .post(&webhook_url)
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        let success = status.is_success();

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Object(json!({
            "success": success,
            "status": status.as_u16()
        })));
        
        Ok(outputs)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordAlertBotNode;

#[async_trait]
impl Node for DiscordAlertBotNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            name: "discord_alert_bot".to_string(),
            display_name: "Discord Alert Bot".to_string(),
            description: "Advanced Discord bot for alerts with severity levels and formatting".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                NodeParameter {
                    name: "webhook_url".to_string(),
                    display_name: "Webhook URL".to_string(),
                    description: "Discord webhook URL".to_string(),
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
                    name: "mention_role".to_string(),
                    display_name: "Mention Role ID".to_string(),
                    description: "Role ID to mention for critical alerts".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
            ],
            inputs: vec!["trigger".to_string()],
            outputs: vec!["result".to_string()],
        }
    }

    async fn execute(
        &self,
        context: ghostflow_core::ExecutionContext,
    ) -> Result<HashMap<String, Value>> {
        let webhook_url = context.get_parameter("webhook_url")
            .and_then(|v| v.as_string())
            .ok_or("Webhook URL is required")?;
        
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

        let (color, emoji) = match alert_type.as_str() {
            "critical" => (0xFF0000, "🚨"),
            "error" => (0xFF6B6B, "❌"),
            "warning" => (0xFFA500, "⚠️"),
            "success" => (0x00FF00, "✅"),
            "info" => (0x3498DB, "ℹ️"),
            _ => (0x7289DA, "📢"),
        };

        let mut fields = vec![
            json!({
                "name": "Source",
                "value": source,
                "inline": true
            }),
            json!({
                "name": "Severity",
                "value": alert_type.to_uppercase(),
                "inline": true
            }),
            json!({
                "name": "Time",
                "value": format!("<t:{}:F>", chrono::Utc::now().timestamp()),
                "inline": true
            })
        ];

        if let Some(metadata) = context.get_parameter("metadata") {
            if let Value::Object(obj) = metadata {
                for (key, value) in obj.as_object().unwrap().iter() {
                    fields.push(json!({
                        "name": key,
                        "value": value.to_string(),
                        "inline": true
                    }));
                }
            }
        }

        let embed = json!({
            "title": format!("{} {}", emoji, title),
            "description": message,
            "color": color,
            "fields": fields,
            "footer": {
                "text": "GhostFlow Alert System"
            },
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        let mut content = String::new();
        if alert_type == "critical" {
            if let Some(role_id) = context.get_parameter("mention_role").and_then(|v| v.as_string()) {
                content = format!("<@&{}>", role_id);
            }
        }

        let body = if content.is_empty() {
            json!({
                "embeds": [embed],
                "username": "GhostFlow Alerts"
            })
        } else {
            json!({
                "content": content,
                "embeds": [embed],
                "username": "GhostFlow Alerts"
            })
        };

        let client = reqwest::Client::new();
        let response = client
            .post(&webhook_url)
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        let success = status.is_success();

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Object(json!({
            "success": success,
            "status": status.as_u16(),
            "alert_sent": success
        })));
        
        Ok(outputs)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordChatBotNode;

#[async_trait]
impl Node for DiscordChatBotNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            name: "discord_chat_bot".to_string(),
            display_name: "Discord Chat Bot".to_string(),
            description: "Interactive Discord bot with conversation context and AI integration".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                NodeParameter {
                    name: "bot_token".to_string(),
                    display_name: "Bot Token".to_string(),
                    description: "Discord bot token".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "channel_id".to_string(),
                    display_name: "Channel ID".to_string(),
                    description: "Discord channel ID to send message to".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "message".to_string(),
                    display_name: "Message".to_string(),
                    description: "Message to send or process".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "operation".to_string(),
                    display_name: "Operation".to_string(),
                    description: "Bot operation to perform".to_string(),
                    parameter_type: ParameterType::Select,
                    required: true,
                    default_value: Some(Value::String("send_message".to_string())),
                },
                NodeParameter {
                    name: "reply_to".to_string(),
                    display_name: "Reply To Message ID".to_string(),
                    description: "Message ID to reply to".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "ai_enabled".to_string(),
                    display_name: "Enable AI Responses".to_string(),
                    description: "Process messages with AI for intelligent responses".to_string(),
                    parameter_type: ParameterType::Boolean,
                    required: false,
                    default_value: Some(Value::Bool(false)),
                },
                NodeParameter {
                    name: "context".to_string(),
                    display_name: "Conversation Context".to_string(),
                    description: "Previous conversation context for AI".to_string(),
                    parameter_type: ParameterType::Json,
                    required: false,
                    default_value: None,
                },
            ],
            inputs: vec!["trigger".to_string(), "ai_response".to_string()],
            outputs: vec!["result".to_string(), "message_id".to_string()],
        }
    }

    async fn execute(
        &self,
        context: ghostflow_core::ExecutionContext,
    ) -> Result<HashMap<String, Value>> {
        let bot_token = context.get_parameter("bot_token")
            .and_then(|v| v.as_string())
            .ok_or("Bot token is required")?;
        
        let channel_id = context.get_parameter("channel_id")
            .and_then(|v| v.as_string())
            .ok_or("Channel ID is required")?;
        
        let message = context.get_parameter("message")
            .and_then(|v| v.as_string())
            .ok_or("Message is required")?;
        
        let operation = context.get_parameter("operation")
            .and_then(|v| v.as_string())
            .unwrap_or("send_message".to_string());

        let client = reqwest::Client::new();
        let base_url = "https://discord.com/api/v10";

        let result = match operation.as_str() {
            "send_message" => {
                let mut body = json!({
                    "content": message
                });

                if let Some(reply_to) = context.get_parameter("reply_to").and_then(|v| v.as_string()) {
                    body["message_reference"] = json!({
                        "message_id": reply_to
                    });
                }

                let response = client
                    .post(&format!("{}/channels/{}/messages", base_url, channel_id))
                    .header("Authorization", format!("Bot {}", bot_token))
                    .header("Content-Type", "application/json")
                    .json(&body)
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "get_messages" => {
                let response = client
                    .get(&format!("{}/channels/{}/messages", base_url, channel_id))
                    .header("Authorization", format!("Bot {}", bot_token))
                    .query(&[("limit", "50")])
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "create_thread" => {
                let body = json!({
                    "name": message,
                    "auto_archive_duration": 1440,
                    "type": 11
                });

                let response = client
                    .post(&format!("{}/channels/{}/threads", base_url, channel_id))
                    .header("Authorization", format!("Bot {}", bot_token))
                    .header("Content-Type", "application/json")
                    .json(&body)
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
        
        if let Some(message_id) = result.get("id").and_then(|v| v.as_str()) {
            outputs.insert("message_id".to_string(), Value::String(message_id.to_string()));
        }
        
        Ok(outputs)
    }
}