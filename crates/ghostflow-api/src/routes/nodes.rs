use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use crate::{AppState, ApiResult};
use ghostflow_core::{NodeDefinition, NodeParameter, ParameterType};

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeListQuery {
    pub category: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeListResponse {
    pub nodes: Vec<NodeCatalogEntry>,
    pub categories: Vec<NodeCategory>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeCatalogEntry {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub category: String,
    pub version: String,
    pub icon: Option<String>,
    pub tags: Vec<String>,
    pub input_count: u32,
    pub output_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeCategory {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub node_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeDetailResponse {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub category: String,
    pub version: String,
    pub icon: Option<String>,
    pub parameters: Vec<NodeParameterInfo>,
    pub inputs: Vec<NodePortInfo>,
    pub outputs: Vec<NodePortInfo>,
    pub examples: Vec<NodeExample>,
    pub documentation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeParameterInfo {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub parameter_type: ParameterType,
    pub required: bool,
    pub default_value: Option<serde_json::Value>,
    pub validation: Option<ParameterValidation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterValidation {
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub pattern: Option<String>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodePortInfo {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub data_type: String,
    pub required: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeExample {
    pub title: String,
    pub description: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub expected_output: Option<serde_json::Value>,
}

pub async fn list_nodes(
    Query(query): Query<NodeListQuery>,
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<NodeListResponse>> {
    // TODO: Get from actual node registry
    let all_nodes = get_sample_nodes();
    
    let filtered_nodes = if let Some(category) = query.category {
        all_nodes.into_iter()
            .filter(|node| node.category == category)
            .collect()
    } else if let Some(search) = query.search {
        let search_lower = search.to_lowercase();
        all_nodes.into_iter()
            .filter(|node| {
                node.name.to_lowercase().contains(&search_lower) ||
                node.display_name.to_lowercase().contains(&search_lower) ||
                node.description.to_lowercase().contains(&search_lower) ||
                node.tags.iter().any(|tag| tag.to_lowercase().contains(&search_lower))
            })
            .collect()
    } else {
        all_nodes
    };
    
    let categories = vec![
        NodeCategory {
            id: "basic".to_string(),
            name: "Basic Nodes".to_string(),
            description: "Fundamental workflow building blocks".to_string(),
            icon: "🔧".to_string(),
            node_count: 5,
        },
        NodeCategory {
            id: "integrations".to_string(),
            name: "Integrations".to_string(),
            description: "Connect to external services and APIs".to_string(),
            icon: "🔌".to_string(),
            node_count: 25,
        },
        NodeCategory {
            id: "ai".to_string(),
            name: "AI & ML".to_string(),
            description: "Artificial intelligence and machine learning".to_string(),
            icon: "🤖".to_string(),
            node_count: 8,
        },
        NodeCategory {
            id: "data".to_string(),
            name: "Data Processing".to_string(),
            description: "Transform and manipulate data".to_string(),
            icon: "📊".to_string(),
            node_count: 12,
        },
        NodeCategory {
            id: "infrastructure".to_string(),
            name: "Infrastructure".to_string(),
            description: "Manage servers, containers, and infrastructure".to_string(),
            icon: "🖥️".to_string(),
            node_count: 15,
        },
        NodeCategory {
            id: "security".to_string(),
            name: "Security".to_string(),
            description: "Security monitoring and incident response".to_string(),
            icon: "🔒".to_string(),
            node_count: 6,
        },
    ];
    
    let response = NodeListResponse {
        nodes: filtered_nodes,
        categories,
    };
    
    Ok(Json(response))
}

pub async fn get_node(
    Path(node_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<NodeDetailResponse>> {
    // TODO: Get from actual node registry
    let node_detail = get_sample_node_detail(&node_id)?;
    Ok(Json(node_detail))
}

fn get_sample_nodes() -> Vec<NodeCatalogEntry> {
    vec![
        // Basic Nodes
        NodeCatalogEntry {
            id: "http_request".to_string(),
            name: "http_request".to_string(),
            display_name: "HTTP Request".to_string(),
            description: "Make HTTP requests to external APIs and services".to_string(),
            category: "basic".to_string(),
            version: "1.0.0".to_string(),
            icon: Some("🌐".to_string()),
            tags: vec!["http".to_string(), "api".to_string(), "request".to_string()],
            input_count: 1,
            output_count: 2,
        },
        NodeCatalogEntry {
            id: "webhook".to_string(),
            name: "webhook".to_string(),
            display_name: "Webhook".to_string(),
            description: "Receive HTTP webhooks from external services".to_string(),
            category: "basic".to_string(),
            version: "1.0.0".to_string(),
            icon: Some("📨".to_string()),
            tags: vec!["webhook".to_string(), "trigger".to_string(), "http".to_string()],
            input_count: 0,
            output_count: 1,
        },
        NodeCatalogEntry {
            id: "if_else".to_string(),
            name: "if_else".to_string(),
            display_name: "If/Else".to_string(),
            description: "Conditional branching based on input data".to_string(),
            category: "basic".to_string(),
            version: "1.0.0".to_string(),
            icon: Some("🔀".to_string()),
            tags: vec!["conditional".to_string(), "logic".to_string(), "branch".to_string()],
            input_count: 1,
            output_count: 2,
        },
        
        // Integration Nodes
        NodeCatalogEntry {
            id: "cloudflare_dns".to_string(),
            name: "cloudflare_dns".to_string(),
            display_name: "Cloudflare DNS".to_string(),
            description: "Manage Cloudflare DNS records and settings".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            icon: Some("☁️".to_string()),
            tags: vec!["cloudflare".to_string(), "dns".to_string(), "cdn".to_string()],
            input_count: 0,
            output_count: 1,
        },
        NodeCatalogEntry {
            id: "discord_webhook".to_string(),
            name: "discord_webhook".to_string(),
            display_name: "Discord Webhook".to_string(),
            description: "Send messages to Discord via webhook".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            icon: Some("💬".to_string()),
            tags: vec!["discord".to_string(), "chat".to_string(), "notification".to_string()],
            input_count: 1,
            output_count: 1,
        },
        NodeCatalogEntry {
            id: "slack_message".to_string(),
            name: "slack_message".to_string(),
            display_name: "Slack Message".to_string(),
            description: "Send messages to Slack channels or users".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            icon: Some("💼".to_string()),
            tags: vec!["slack".to_string(), "chat".to_string(), "notification".to_string()],
            input_count: 0,
            output_count: 1,
        },
        NodeCatalogEntry {
            id: "microsoft_graph_email".to_string(),
            name: "microsoft_graph_email".to_string(),
            display_name: "Microsoft 365 Email".to_string(),
            description: "Send and manage emails via Microsoft Graph API".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            icon: Some("📧".to_string()),
            tags: vec!["microsoft".to_string(), "email".to_string(), "outlook".to_string()],
            input_count: 0,
            output_count: 1,
        },
        NodeCatalogEntry {
            id: "google_sheets".to_string(),
            name: "google_sheets".to_string(),
            display_name: "Google Sheets".to_string(),
            description: "Read from and write to Google Sheets".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            icon: Some("📊".to_string()),
            tags: vec!["google".to_string(), "sheets".to_string(), "spreadsheet".to_string()],
            input_count: 0,
            output_count: 3,
        },
        NodeCatalogEntry {
            id: "gitlab_project".to_string(),
            name: "gitlab_project".to_string(),
            display_name: "GitLab Project".to_string(),
            description: "Manage GitLab projects and repositories".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            icon: Some("🦊".to_string()),
            tags: vec!["gitlab".to_string(), "git".to_string(), "project".to_string()],
            input_count: 0,
            output_count: 1,
        },
        
        // Infrastructure Nodes
        NodeCatalogEntry {
            id: "azure_vm".to_string(),
            name: "azure_vm".to_string(),
            display_name: "Azure Virtual Machine".to_string(),
            description: "Manage Azure Virtual Machines".to_string(),
            category: "infrastructure".to_string(),
            version: "1.0.0".to_string(),
            icon: Some("☁️".to_string()),
            tags: vec!["azure".to_string(), "vm".to_string(), "cloud".to_string()],
            input_count: 0,
            output_count: 2,
        },
        NodeCatalogEntry {
            id: "proxmox_vm".to_string(),
            name: "proxmox_vm".to_string(),
            display_name: "Proxmox VM".to_string(),
            description: "Manage Proxmox Virtual Machines".to_string(),
            category: "infrastructure".to_string(),
            version: "1.0.0".to_string(),
            icon: Some("🖥️".to_string()),
            tags: vec!["proxmox".to_string(), "vm".to_string(), "virtualization".to_string()],
            input_count: 0,
            output_count: 2,
        },
        
        // Security Nodes
        NodeCatalogEntry {
            id: "wazuh_api".to_string(),
            name: "wazuh_api".to_string(),
            display_name: "Wazuh SIEM".to_string(),
            description: "Interact with Wazuh security monitoring platform".to_string(),
            category: "security".to_string(),
            version: "1.0.0".to_string(),
            icon: Some("🔒".to_string()),
            tags: vec!["wazuh".to_string(), "siem".to_string(), "security".to_string()],
            input_count: 0,
            output_count: 2,
        },
        
        // AI Nodes
        NodeCatalogEntry {
            id: "ollama_generate".to_string(),
            name: "ollama_generate".to_string(),
            display_name: "Ollama Generate".to_string(),
            description: "Generate text using local Ollama models".to_string(),
            category: "ai".to_string(),
            version: "1.0.0".to_string(),
            icon: Some("🤖".to_string()),
            tags: vec!["ollama".to_string(), "ai".to_string(), "llm".to_string(), "text".to_string()],
            input_count: 0,
            output_count: 1,
        },
    ]
}

fn get_sample_node_detail(node_id: &str) -> crate::Result<NodeDetailResponse> {
    match node_id {
        "discord_webhook" => Ok(NodeDetailResponse {
            id: "discord_webhook".to_string(),
            name: "discord_webhook".to_string(),
            display_name: "Discord Webhook".to_string(),
            description: "Send messages to Discord via webhook with rich formatting support".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            icon: Some("💬".to_string()),
            parameters: vec![
                NodeParameterInfo {
                    name: "webhook_url".to_string(),
                    display_name: "Webhook URL".to_string(),
                    description: "Discord webhook URL from channel settings".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                    validation: Some(ParameterValidation {
                        pattern: Some(r"^https://discord\.com/api/webhooks/".to_string()),
                        ..Default::default()
                    }),
                },
                NodeParameterInfo {
                    name: "content".to_string(),
                    display_name: "Message Content".to_string(),
                    description: "Text message to send".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                    validation: Some(ParameterValidation {
                        max_length: Some(2000),
                        ..Default::default()
                    }),
                },
                NodeParameterInfo {
                    name: "username".to_string(),
                    display_name: "Username".to_string(),
                    description: "Override webhook username".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: Some(serde_json::Value::String("GhostFlow".to_string())),
                    validation: Some(ParameterValidation {
                        max_length: Some(80),
                        ..Default::default()
                    }),
                },
            ],
            inputs: vec![
                NodePortInfo {
                    name: "trigger".to_string(),
                    display_name: "Trigger".to_string(),
                    description: "Trigger the webhook execution".to_string(),
                    data_type: "any".to_string(),
                    required: true,
                },
            ],
            outputs: vec![
                NodePortInfo {
                    name: "result".to_string(),
                    display_name: "Result".to_string(),
                    description: "Webhook execution result".to_string(),
                    data_type: "object".to_string(),
                    required: false,
                },
            ],
            examples: vec![
                NodeExample {
                    title: "Simple Message".to_string(),
                    description: "Send a simple text message to Discord".to_string(),
                    parameters: {
                        let mut params = HashMap::new();
                        params.insert("webhook_url".to_string(), serde_json::Value::String("https://discord.com/api/webhooks/123/abc".to_string()));
                        params.insert("content".to_string(), serde_json::Value::String("Hello from GhostFlow!".to_string()));
                        params
                    },
                    expected_output: Some(serde_json::json!({
                        "success": true,
                        "status": 200
                    })),
                },
            ],
            documentation: Some("For detailed documentation, visit: https://docs.ghostflow.dev/nodes/discord-webhook".to_string()),
        }),
        _ => Err(crate::ApiError::NotFound("Node not found".to_string())),
    }
}

impl Default for ParameterValidation {
    fn default() -> Self {
        Self {
            min_length: None,
            max_length: None,
            pattern: None,
            min_value: None,
            max_value: None,
            options: None,
        }
    }
}