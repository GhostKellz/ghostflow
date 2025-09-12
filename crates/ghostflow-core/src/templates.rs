use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowTemplate {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub category: TemplateCategory,
    pub tags: Vec<String>,
    pub version: String,
    pub author: String,
    pub icon: Option<String>,
    pub screenshot: Option<String>,
    pub difficulty: TemplateDifficulty,
    pub estimated_time: String,
    pub use_cases: Vec<String>,
    pub prerequisites: Vec<String>,
    pub template_data: TemplateData,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub downloads: u64,
    pub rating: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateData {
    pub nodes: Vec<TemplateNode>,
    pub edges: Vec<TemplateEdge>,
    pub triggers: Vec<TemplateTrigger>,
    pub variables: Vec<TemplateVariable>,
    pub schedule: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateNode {
    pub id: String,
    pub node_type: String,
    pub position: Position,
    pub parameters: HashMap<String, TemplateParameter>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateEdge {
    pub id: String,
    pub source_node: String,
    pub source_output: String,
    pub target_node: String,
    pub target_input: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateTrigger {
    pub trigger_type: String,
    pub configuration: HashMap<String, TemplateParameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub variable_type: VariableType,
    pub default_value: Option<serde_json::Value>,
    pub required: bool,
    pub placeholder: Option<String>,
    pub validation: Option<VariableValidation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VariableType {
    String,
    Number,
    Boolean,
    Email,
    Url,
    Json,
    Secret,
    Select,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableValidation {
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub pattern: Option<String>,
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TemplateParameter {
    Static(serde_json::Value),
    Variable(String), // Variable name
    Expression(String), // Expression to evaluate
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateCategory {
    Alerts,
    Automation,
    DataProcessing,
    Integration,
    Infrastructure,
    Security,
    Monitoring,
    Communication,
    Productivity,
    Development,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TemplateDifficulty {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateInstallation {
    pub template_id: String,
    pub user_variables: HashMap<String, serde_json::Value>,
    pub flow_name: String,
    pub description: Option<String>,
}

pub fn get_builtin_templates() -> Vec<FlowTemplate> {
    vec![
        FlowTemplate {
            id: "discord_security_alerts".to_string(),
            name: "discord_security_alerts".to_string(),
            display_name: "Discord Security Alerts".to_string(),
            description: "Monitor Wazuh security events and send critical alerts to Discord with intelligent filtering and escalation".to_string(),
            category: TemplateCategory::Security,
            tags: vec!["discord".to_string(), "wazuh".to_string(), "security".to_string(), "alerts".to_string()],
            version: "1.0.0".to_string(),
            author: "GhostFlow Team".to_string(),
            icon: Some("🔒".to_string()),
            screenshot: None,
            difficulty: TemplateDifficulty::Beginner,
            estimated_time: "5 minutes".to_string(),
            use_cases: vec![
                "Security incident notifications".to_string(),
                "SOC team alerts".to_string(),
                "Threat detection responses".to_string(),
            ],
            prerequisites: vec![
                "Discord webhook URL".to_string(),
                "Wazuh SIEM access".to_string(),
            ],
            template_data: TemplateData {
                nodes: vec![
                    TemplateNode {
                        id: "wazuh_monitor".to_string(),
                        node_type: "wazuh_api".to_string(),
                        position: Position { x: 100.0, y: 100.0 },
                        parameters: {
                            let mut params = HashMap::new();
                            params.insert("base_url".to_string(), TemplateParameter::Variable("wazuh_url".to_string()));
                            params.insert("username".to_string(), TemplateParameter::Variable("wazuh_username".to_string()));
                            params.insert("password".to_string(), TemplateParameter::Variable("wazuh_password".to_string()));
                            params.insert("operation".to_string(), TemplateParameter::Static(serde_json::Value::String("get_alerts".to_string())));
                            params.insert("level".to_string(), TemplateParameter::Variable("alert_level".to_string()));
                            params
                        },
                        description: Some("Monitor Wazuh for security alerts".to_string()),
                    },
                    TemplateNode {
                        id: "alert_filter".to_string(),
                        node_type: "wazuh_alert_processor".to_string(),
                        position: Position { x: 400.0, y: 100.0 },
                        parameters: {
                            let mut params = HashMap::new();
                            params.insert("filter_level".to_string(), TemplateParameter::Variable("filter_level".to_string()));
                            params.insert("enable_correlation".to_string(), TemplateParameter::Static(serde_json::Value::Bool(true)));
                            params
                        },
                        description: Some("Filter and correlate security alerts".to_string()),
                    },
                    TemplateNode {
                        id: "discord_alert".to_string(),
                        node_type: "discord_alert_bot".to_string(),
                        position: Position { x: 700.0, y: 100.0 },
                        parameters: {
                            let mut params = HashMap::new();
                            params.insert("webhook_url".to_string(), TemplateParameter::Variable("discord_webhook".to_string()));
                            params.insert("mention_role".to_string(), TemplateParameter::Variable("discord_role_id".to_string()));
                            params
                        },
                        description: Some("Send formatted alerts to Discord".to_string()),
                    },
                ],
                edges: vec![
                    TemplateEdge {
                        id: "edge_1".to_string(),
                        source_node: "wazuh_monitor".to_string(),
                        source_output: "alerts".to_string(),
                        target_node: "alert_filter".to_string(),
                        target_input: "alerts".to_string(),
                    },
                    TemplateEdge {
                        id: "edge_2".to_string(),
                        source_node: "alert_filter".to_string(),
                        source_output: "high_priority".to_string(),
                        target_node: "discord_alert".to_string(),
                        target_input: "trigger".to_string(),
                    },
                ],
                triggers: vec![
                    TemplateTrigger {
                        trigger_type: "schedule".to_string(),
                        configuration: {
                            let mut config = HashMap::new();
                            config.insert("cron".to_string(), TemplateParameter::Variable("schedule".to_string()));
                            config
                        },
                    },
                ],
                variables: vec![
                    TemplateVariable {
                        name: "wazuh_url".to_string(),
                        display_name: "Wazuh Server URL".to_string(),
                        description: "URL of your Wazuh manager API".to_string(),
                        variable_type: VariableType::Url,
                        default_value: Some(serde_json::Value::String("https://wazuh-manager:55000".to_string())),
                        required: true,
                        placeholder: Some("https://wazuh-manager:55000".to_string()),
                        validation: None,
                    },
                    TemplateVariable {
                        name: "wazuh_username".to_string(),
                        display_name: "Wazuh Username".to_string(),
                        description: "Wazuh API username".to_string(),
                        variable_type: VariableType::String,
                        default_value: None,
                        required: true,
                        placeholder: Some("Enter username".to_string()),
                        validation: None,
                    },
                    TemplateVariable {
                        name: "wazuh_password".to_string(),
                        display_name: "Wazuh Password".to_string(),
                        description: "Wazuh API password".to_string(),
                        variable_type: VariableType::Secret,
                        default_value: None,
                        required: true,
                        placeholder: Some("Enter password".to_string()),
                        validation: None,
                    },
                    TemplateVariable {
                        name: "discord_webhook".to_string(),
                        display_name: "Discord Webhook URL".to_string(),
                        description: "Discord channel webhook URL for alerts".to_string(),
                        variable_type: VariableType::Url,
                        default_value: None,
                        required: true,
                        placeholder: Some("https://discord.com/api/webhooks/...".to_string()),
                        validation: Some(VariableValidation {
                            pattern: Some(r"^https://discord\.com/api/webhooks/".to_string()),
                            ..Default::default()
                        }),
                    },
                    TemplateVariable {
                        name: "discord_role_id".to_string(),
                        display_name: "Discord Role ID (Optional)".to_string(),
                        description: "Role ID to mention for critical alerts".to_string(),
                        variable_type: VariableType::String,
                        default_value: None,
                        required: false,
                        placeholder: Some("123456789012345678".to_string()),
                        validation: None,
                    },
                    TemplateVariable {
                        name: "alert_level".to_string(),
                        display_name: "Minimum Alert Level".to_string(),
                        description: "Minimum severity level to process (0-15)".to_string(),
                        variable_type: VariableType::Select,
                        default_value: Some(serde_json::Value::String("7".to_string())),
                        required: false,
                        placeholder: None,
                        validation: Some(VariableValidation {
                            options: Some(vec!["3".to_string(), "7".to_string(), "10".to_string(), "13".to_string()]),
                            ..Default::default()
                        }),
                    },
                    TemplateVariable {
                        name: "filter_level".to_string(),
                        display_name: "Filter Level".to_string(),
                        description: "Alert filtering level".to_string(),
                        variable_type: VariableType::Select,
                        default_value: Some(serde_json::Value::String("medium".to_string())),
                        required: false,
                        placeholder: None,
                        validation: Some(VariableValidation {
                            options: Some(vec!["low".to_string(), "medium".to_string(), "high".to_string(), "critical".to_string()]),
                            ..Default::default()
                        }),
                    },
                    TemplateVariable {
                        name: "schedule".to_string(),
                        display_name: "Check Schedule".to_string(),
                        description: "How often to check for alerts (cron format)".to_string(),
                        variable_type: VariableType::String,
                        default_value: Some(serde_json::Value::String("0 */5 * * * *".to_string())),
                        required: false,
                        placeholder: Some("0 */5 * * * * (every 5 minutes)".to_string()),
                        validation: None,
                    },
                ],
                schedule: Some("0 */5 * * * *".to_string()),
            },
            created_at: Utc::now() - chrono::Duration::days(7),
            updated_at: Utc::now() - chrono::Duration::days(2),
            downloads: 1250,
            rating: Some(4.8),
        },
        
        FlowTemplate {
            id: "proxmox_vm_monitoring".to_string(),
            name: "proxmox_vm_monitoring".to_string(),
            display_name: "Proxmox VM Monitoring".to_string(),
            description: "Monitor Proxmox virtual machine resources and send alerts when thresholds are exceeded".to_string(),
            category: TemplateCategory::Infrastructure,
            tags: vec!["proxmox".to_string(), "monitoring".to_string(), "vm".to_string(), "infrastructure".to_string()],
            version: "1.0.0".to_string(),
            author: "GhostFlow Team".to_string(),
            icon: Some("🖥️".to_string()),
            screenshot: None,
            difficulty: TemplateDifficulty::Intermediate,
            estimated_time: "10 minutes".to_string(),
            use_cases: vec![
                "VM resource monitoring".to_string(),
                "Infrastructure alerting".to_string(),
                "Capacity planning".to_string(),
            ],
            prerequisites: vec![
                "Proxmox server access".to_string(),
                "Notification channel (Slack/Discord/Email)".to_string(),
            ],
            template_data: TemplateData {
                nodes: vec![
                    TemplateNode {
                        id: "vm_status".to_string(),
                        node_type: "proxmox_vm".to_string(),
                        position: Position { x: 100.0, y: 100.0 },
                        parameters: {
                            let mut params = HashMap::new();
                            params.insert("host".to_string(), TemplateParameter::Variable("proxmox_host".to_string()));
                            params.insert("username".to_string(), TemplateParameter::Variable("proxmox_username".to_string()));
                            params.insert("password".to_string(), TemplateParameter::Variable("proxmox_password".to_string()));
                            params.insert("operation".to_string(), TemplateParameter::Static(serde_json::Value::String("list".to_string())));
                            params
                        },
                        description: Some("Get VM status from Proxmox".to_string()),
                    },
                    TemplateNode {
                        id: "resource_check".to_string(),
                        node_type: "if_else".to_string(),
                        position: Position { x: 400.0, y: 100.0 },
                        parameters: {
                            let mut params = HashMap::new();
                            params.insert("condition".to_string(), TemplateParameter::Expression("cpu_usage > 80 OR memory_usage > 90".to_string()));
                            params
                        },
                        description: Some("Check if resources exceed thresholds".to_string()),
                    },
                    TemplateNode {
                        id: "send_alert".to_string(),
                        node_type: "slack_alert".to_string(),
                        position: Position { x: 700.0, y: 100.0 },
                        parameters: {
                            let mut params = HashMap::new();
                            params.insert("bot_token".to_string(), TemplateParameter::Variable("slack_token".to_string()));
                            params.insert("channel".to_string(), TemplateParameter::Variable("slack_channel".to_string()));
                            params.insert("alert_type".to_string(), TemplateParameter::Static(serde_json::Value::String("warning".to_string())));
                            params
                        },
                        description: Some("Send alert to Slack".to_string()),
                    },
                ],
                edges: vec![
                    TemplateEdge {
                        id: "edge_1".to_string(),
                        source_node: "vm_status".to_string(),
                        source_output: "result".to_string(),
                        target_node: "resource_check".to_string(),
                        target_input: "input".to_string(),
                    },
                    TemplateEdge {
                        id: "edge_2".to_string(),
                        source_node: "resource_check".to_string(),
                        source_output: "true".to_string(),
                        target_node: "send_alert".to_string(),
                        target_input: "trigger".to_string(),
                    },
                ],
                triggers: vec![
                    TemplateTrigger {
                        trigger_type: "schedule".to_string(),
                        configuration: {
                            let mut config = HashMap::new();
                            config.insert("cron".to_string(), TemplateParameter::Variable("check_interval".to_string()));
                            config
                        },
                    },
                ],
                variables: vec![
                    TemplateVariable {
                        name: "proxmox_host".to_string(),
                        display_name: "Proxmox Host".to_string(),
                        description: "Proxmox server hostname or IP address".to_string(),
                        variable_type: VariableType::String,
                        default_value: None,
                        required: true,
                        placeholder: Some("proxmox.example.com".to_string()),
                        validation: None,
                    },
                    TemplateVariable {
                        name: "proxmox_username".to_string(),
                        display_name: "Proxmox Username".to_string(),
                        description: "Proxmox authentication username".to_string(),
                        variable_type: VariableType::String,
                        default_value: None,
                        required: true,
                        placeholder: Some("user@pam".to_string()),
                        validation: None,
                    },
                    TemplateVariable {
                        name: "proxmox_password".to_string(),
                        display_name: "Proxmox Password".to_string(),
                        description: "Proxmox authentication password".to_string(),
                        variable_type: VariableType::Secret,
                        default_value: None,
                        required: true,
                        placeholder: Some("Enter password".to_string()),
                        validation: None,
                    },
                    TemplateVariable {
                        name: "slack_token".to_string(),
                        display_name: "Slack Bot Token".to_string(),
                        description: "Slack bot token for sending alerts".to_string(),
                        variable_type: VariableType::Secret,
                        default_value: None,
                        required: true,
                        placeholder: Some("xoxb-...".to_string()),
                        validation: Some(VariableValidation {
                            pattern: Some(r"^xoxb-".to_string()),
                            ..Default::default()
                        }),
                    },
                    TemplateVariable {
                        name: "slack_channel".to_string(),
                        display_name: "Slack Channel".to_string(),
                        description: "Slack channel for alerts".to_string(),
                        variable_type: VariableType::String,
                        default_value: Some(serde_json::Value::String("#alerts".to_string())),
                        required: true,
                        placeholder: Some("#infrastructure-alerts".to_string()),
                        validation: None,
                    },
                    TemplateVariable {
                        name: "check_interval".to_string(),
                        display_name: "Check Interval".to_string(),
                        description: "How often to check VM resources".to_string(),
                        variable_type: VariableType::Select,
                        default_value: Some(serde_json::Value::String("0 */10 * * * *".to_string())),
                        required: false,
                        placeholder: None,
                        validation: Some(VariableValidation {
                            options: Some(vec![
                                "0 */5 * * * *".to_string(),   // Every 5 minutes
                                "0 */10 * * * *".to_string(),  // Every 10 minutes
                                "0 */30 * * * *".to_string(),  // Every 30 minutes
                                "0 0 * * * *".to_string(),     // Every hour
                            ]),
                            ..Default::default()
                        }),
                    },
                ],
                schedule: Some("0 */10 * * * *".to_string()),
            },
            created_at: Utc::now() - chrono::Duration::days(14),
            updated_at: Utc::now() - chrono::Duration::days(5),
            downloads: 856,
            rating: Some(4.6),
        },

        FlowTemplate {
            id: "microsoft_teams_daily_report".to_string(),
            name: "microsoft_teams_daily_report".to_string(),
            display_name: "Microsoft Teams Daily Report".to_string(),
            description: "Generate and send daily system reports to Microsoft Teams with data from multiple sources".to_string(),
            category: TemplateCategory::Communication,
            tags: vec!["microsoft".to_string(), "teams".to_string(), "reporting".to_string(), "daily".to_string()],
            version: "1.0.0".to_string(),
            author: "GhostFlow Team".to_string(),
            icon: Some("📊".to_string()),
            screenshot: None,
            difficulty: TemplateDifficulty::Beginner,
            estimated_time: "7 minutes".to_string(),
            use_cases: vec![
                "Daily status reports".to_string(),
                "Team communication".to_string(),
                "Automated reporting".to_string(),
            ],
            prerequisites: vec![
                "Microsoft Teams access".to_string(),
                "Microsoft Graph API credentials".to_string(),
            ],
            template_data: TemplateData {
                nodes: vec![
                    TemplateNode {
                        id: "generate_report".to_string(),
                        node_type: "template".to_string(),
                        position: Position { x: 100.0, y: 100.0 },
                        parameters: {
                            let mut params = HashMap::new();
                            params.insert("template".to_string(), TemplateParameter::Variable("report_template".to_string()));
                            params
                        },
                        description: Some("Generate report content".to_string()),
                    },
                    TemplateNode {
                        id: "send_teams".to_string(),
                        node_type: "microsoft_teams".to_string(),
                        position: Position { x: 400.0, y: 100.0 },
                        parameters: {
                            let mut params = HashMap::new();
                            params.insert("access_token".to_string(), TemplateParameter::Variable("teams_token".to_string()));
                            params.insert("team_id".to_string(), TemplateParameter::Variable("team_id".to_string()));
                            params.insert("channel_id".to_string(), TemplateParameter::Variable("channel_id".to_string()));
                            params.insert("operation".to_string(), TemplateParameter::Static(serde_json::Value::String("send_message".to_string())));
                            params
                        },
                        description: Some("Send report to Teams".to_string()),
                    },
                ],
                edges: vec![
                    TemplateEdge {
                        id: "edge_1".to_string(),
                        source_node: "generate_report".to_string(),
                        source_output: "result".to_string(),
                        target_node: "send_teams".to_string(),
                        target_input: "message".to_string(),
                    },
                ],
                triggers: vec![
                    TemplateTrigger {
                        trigger_type: "schedule".to_string(),
                        configuration: {
                            let mut config = HashMap::new();
                            config.insert("cron".to_string(), TemplateParameter::Variable("report_schedule".to_string()));
                            config
                        },
                    },
                ],
                variables: vec![
                    TemplateVariable {
                        name: "teams_token".to_string(),
                        display_name: "Microsoft Graph Token".to_string(),
                        description: "Access token for Microsoft Graph API".to_string(),
                        variable_type: VariableType::Secret,
                        default_value: None,
                        required: true,
                        placeholder: Some("Enter access token".to_string()),
                        validation: None,
                    },
                    TemplateVariable {
                        name: "team_id".to_string(),
                        display_name: "Teams Team ID".to_string(),
                        description: "Microsoft Teams team ID".to_string(),
                        variable_type: VariableType::String,
                        default_value: None,
                        required: true,
                        placeholder: Some("Enter team ID".to_string()),
                        validation: None,
                    },
                    TemplateVariable {
                        name: "channel_id".to_string(),
                        display_name: "Teams Channel ID".to_string(),
                        description: "Microsoft Teams channel ID".to_string(),
                        variable_type: VariableType::String,
                        default_value: None,
                        required: true,
                        placeholder: Some("Enter channel ID".to_string()),
                        validation: None,
                    },
                    TemplateVariable {
                        name: "report_template".to_string(),
                        display_name: "Report Template".to_string(),
                        description: "Template for the daily report".to_string(),
                        variable_type: VariableType::String,
                        default_value: Some(serde_json::Value::String("# Daily System Report\n\n**Date:** {{date}}\n**Status:** All systems operational\n**Uptime:** {{uptime}}\n\nHave a great day! 🚀".to_string())),
                        required: false,
                        placeholder: Some("Enter report template with {{variables}}".to_string()),
                        validation: None,
                    },
                    TemplateVariable {
                        name: "report_schedule".to_string(),
                        display_name: "Report Schedule".to_string(),
                        description: "When to send the daily report".to_string(),
                        variable_type: VariableType::String,
                        default_value: Some(serde_json::Value::String("0 0 9 * * MON-FRI".to_string())),
                        required: false,
                        placeholder: Some("0 0 9 * * MON-FRI (9 AM weekdays)".to_string()),
                        validation: None,
                    },
                ],
                schedule: Some("0 0 9 * * MON-FRI".to_string()),
            },
            created_at: Utc::now() - chrono::Duration::days(21),
            updated_at: Utc::now() - chrono::Duration::days(10),
            downloads: 2103,
            rating: Some(4.9),
        },
    ]
}

impl Default for VariableValidation {
    fn default() -> Self {
        Self {
            min_length: None,
            max_length: None,
            pattern: None,
            options: None,
        }
    }
}