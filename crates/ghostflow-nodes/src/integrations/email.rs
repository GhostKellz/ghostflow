use ghostflow_core::{Node, NodeDefinition, NodeParameter, ParameterType, Result, Value};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMTPEmailNode;

#[async_trait]
impl Node for SMTPEmailNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            name: "smtp_email".to_string(),
            display_name: "SMTP Email".to_string(),
            description: "Send emails via SMTP server".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                NodeParameter {
                    name: "smtp_host".to_string(),
                    display_name: "SMTP Host".to_string(),
                    description: "SMTP server hostname".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "smtp_port".to_string(),
                    display_name: "SMTP Port".to_string(),
                    description: "SMTP server port".to_string(),
                    parameter_type: ParameterType::Number,
                    required: false,
                    default_value: Some(Value::Number(587.0)),
                },
                NodeParameter {
                    name: "username".to_string(),
                    display_name: "Username".to_string(),
                    description: "SMTP authentication username".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "password".to_string(),
                    display_name: "Password".to_string(),
                    description: "SMTP authentication password or app password".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "use_tls".to_string(),
                    display_name: "Use TLS".to_string(),
                    description: "Enable TLS encryption".to_string(),
                    parameter_type: ParameterType::Boolean,
                    required: false,
                    default_value: Some(Value::Bool(true)),
                },
                NodeParameter {
                    name: "from".to_string(),
                    display_name: "From".to_string(),
                    description: "Sender email address".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "from_name".to_string(),
                    display_name: "From Name".to_string(),
                    description: "Sender display name".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "to".to_string(),
                    display_name: "To".to_string(),
                    description: "Recipient email addresses (comma-separated)".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
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
                    name: "bcc".to_string(),
                    display_name: "BCC".to_string(),
                    description: "BCC recipients (comma-separated)".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "subject".to_string(),
                    display_name: "Subject".to_string(),
                    description: "Email subject line".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "body".to_string(),
                    display_name: "Body".to_string(),
                    description: "Email body content".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "body_type".to_string(),
                    display_name: "Body Type".to_string(),
                    description: "Email body format".to_string(),
                    parameter_type: ParameterType::Select,
                    required: false,
                    default_value: Some(Value::String("html".to_string())),
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
        let smtp_host = context.get_parameter("smtp_host")
            .and_then(|v| v.as_string())
            .ok_or("SMTP host is required")?;
        
        let smtp_port = context.get_parameter("smtp_port")
            .and_then(|v| v.as_number())
            .unwrap_or(587.0) as u16;
        
        let username = context.get_parameter("username")
            .and_then(|v| v.as_string())
            .ok_or("Username is required")?;
        
        let password = context.get_parameter("password")
            .and_then(|v| v.as_string())
            .ok_or("Password is required")?;
        
        let use_tls = context.get_parameter("use_tls")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        
        let from = context.get_parameter("from")
            .and_then(|v| v.as_string())
            .ok_or("From address is required")?;
        
        let from_name = context.get_parameter("from_name")
            .and_then(|v| v.as_string());
        
        let to = context.get_parameter("to")
            .and_then(|v| v.as_string())
            .ok_or("To address is required")?;
        
        let subject = context.get_parameter("subject")
            .and_then(|v| v.as_string())
            .ok_or("Subject is required")?;
        
        let body = context.get_parameter("body")
            .and_then(|v| v.as_string())
            .ok_or("Body is required")?;
        
        let body_type = context.get_parameter("body_type")
            .and_then(|v| v.as_string())
            .unwrap_or("html".to_string());

        // Build email message
        let mut email_builder = lettre::Message::builder()
            .from(if let Some(name) = from_name {
                format!("{} <{}>", name, from).parse().unwrap()
            } else {
                from.parse().unwrap()
            });

        // Add recipients
        for recipient in to.split(',') {
            email_builder = email_builder.to(recipient.trim().parse().unwrap());
        }

        if let Some(cc) = context.get_parameter("cc").and_then(|v| v.as_string()) {
            for recipient in cc.split(',') {
                email_builder = email_builder.cc(recipient.trim().parse().unwrap());
            }
        }

        if let Some(bcc) = context.get_parameter("bcc").and_then(|v| v.as_string()) {
            for recipient in bcc.split(',') {
                email_builder = email_builder.bcc(recipient.trim().parse().unwrap());
            }
        }

        let email = email_builder
            .subject(subject)
            .body(body)
            .unwrap();

        // Create SMTP transport
        use lettre::{SmtpTransport, Transport, transport::smtp::authentication::Credentials};

        let creds = Credentials::new(username, password);
        
        let mailer = if use_tls {
            SmtpTransport::relay(&smtp_host)
                .unwrap()
                .port(smtp_port)
                .credentials(creds)
                .build()
        } else {
            SmtpTransport::builder_dangerous(&smtp_host)
                .port(smtp_port)
                .credentials(creds)
                .build()
        };

        // Send email
        let send_result = mailer.send(&email);
        
        let result = match send_result {
            Ok(response) => json!({
                "success": true,
                "message_id": response.message_id(),
                "status": "sent"
            }),
            Err(e) => json!({
                "success": false,
                "error": e.to_string(),
                "status": "failed"
            }),
        };

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Object(result));
        Ok(outputs)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendGridNode;

#[async_trait]
impl Node for SendGridNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            name: "sendgrid_email".to_string(),
            display_name: "SendGrid Email".to_string(),
            description: "Send emails via SendGrid API".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                NodeParameter {
                    name: "api_key".to_string(),
                    display_name: "API Key".to_string(),
                    description: "SendGrid API key".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "from".to_string(),
                    display_name: "From Email".to_string(),
                    description: "Sender email address".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "from_name".to_string(),
                    display_name: "From Name".to_string(),
                    description: "Sender display name".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "to".to_string(),
                    display_name: "To".to_string(),
                    description: "Recipient email addresses (comma-separated)".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "subject".to_string(),
                    display_name: "Subject".to_string(),
                    description: "Email subject line".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "content".to_string(),
                    display_name: "Content".to_string(),
                    description: "Email content".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "content_type".to_string(),
                    display_name: "Content Type".to_string(),
                    description: "Email content type".to_string(),
                    parameter_type: ParameterType::Select,
                    required: false,
                    default_value: Some(Value::String("text/html".to_string())),
                },
                NodeParameter {
                    name: "template_id".to_string(),
                    display_name: "Template ID".to_string(),
                    description: "SendGrid template ID".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "dynamic_template_data".to_string(),
                    display_name: "Template Data".to_string(),
                    description: "Dynamic template data (JSON)".to_string(),
                    parameter_type: ParameterType::Json,
                    required: false,
                    default_value: None,
                },
            ],
            inputs: vec![],
            outputs: vec!["result".to_string(), "message_id".to_string()],
        }
    }

    async fn execute(
        &self,
        context: ghostflow_core::ExecutionContext,
    ) -> Result<HashMap<String, Value>> {
        let api_key = context.get_parameter("api_key")
            .and_then(|v| v.as_string())
            .ok_or("API key is required")?;
        
        let from_email = context.get_parameter("from")
            .and_then(|v| v.as_string())
            .ok_or("From email is required")?;
        
        let from_name = context.get_parameter("from_name")
            .and_then(|v| v.as_string());
        
        let to = context.get_parameter("to")
            .and_then(|v| v.as_string())
            .ok_or("To email is required")?;
        
        let subject = context.get_parameter("subject")
            .and_then(|v| v.as_string())
            .ok_or("Subject is required")?;
        
        let content = context.get_parameter("content")
            .and_then(|v| v.as_string())
            .ok_or("Content is required")?;
        
        let content_type = context.get_parameter("content_type")
            .and_then(|v| v.as_string())
            .unwrap_or("text/html".to_string());

        let client = reqwest::Client::new();
        
        // Build email payload
        let mut email_payload = json!({
            "personalizations": [{
                "to": to.split(',').map(|email| json!({
                    "email": email.trim()
                })).collect::<Vec<_>>(),
                "subject": subject
            }],
            "from": {
                "email": from_email,
                "name": from_name.unwrap_or(from_email.clone())
            },
            "content": [{
                "type": content_type,
                "value": content
            }]
        });

        // Handle dynamic templates
        if let Some(template_id) = context.get_parameter("template_id").and_then(|v| v.as_string()) {
            email_payload["template_id"] = json!(template_id);
            
            if let Some(template_data) = context.get_parameter("dynamic_template_data") {
                email_payload["personalizations"][0]["dynamic_template_data"] = template_data.clone();
            }
            
            // Remove content when using templates
            email_payload.as_object_mut().unwrap().remove("content");
        }

        let response = client
            .post("https://api.sendgrid.com/v3/mail/send")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&email_payload)
            .send()
            .await?;

        let status = response.status();
        let success = status.is_success();
        let response_text = response.text().await?;

        let message_id = if success {
            // Extract message ID from headers if available
            Some("sg_message_id_placeholder".to_string())
        } else {
            None
        };

        let result = json!({
            "success": success,
            "status": status.as_u16(),
            "response": response_text,
            "message_id": message_id
        });

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Object(result));
        
        if let Some(msg_id) = message_id {
            outputs.insert("message_id".to_string(), Value::String(msg_id));
        }
        
        Ok(outputs)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailgunNode;

#[async_trait]
impl Node for MailgunNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            name: "mailgun_email".to_string(),
            display_name: "Mailgun Email".to_string(),
            description: "Send emails via Mailgun API".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                NodeParameter {
                    name: "api_key".to_string(),
                    display_name: "API Key".to_string(),
                    description: "Mailgun API key".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "domain".to_string(),
                    display_name: "Domain".to_string(),
                    description: "Mailgun sending domain".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "region".to_string(),
                    display_name: "Region".to_string(),
                    description: "Mailgun region (us, eu)".to_string(),
                    parameter_type: ParameterType::Select,
                    required: false,
                    default_value: Some(Value::String("us".to_string())),
                },
                NodeParameter {
                    name: "from".to_string(),
                    display_name: "From".to_string(),
                    description: "Sender email address".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "to".to_string(),
                    display_name: "To".to_string(),
                    description: "Recipient email addresses (comma-separated)".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
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
                    name: "bcc".to_string(),
                    display_name: "BCC".to_string(),
                    description: "BCC recipients (comma-separated)".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "subject".to_string(),
                    display_name: "Subject".to_string(),
                    description: "Email subject line".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "text".to_string(),
                    display_name: "Text Content".to_string(),
                    description: "Plain text email content".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "html".to_string(),
                    display_name: "HTML Content".to_string(),
                    description: "HTML email content".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "tags".to_string(),
                    display_name: "Tags".to_string(),
                    description: "Email tags for tracking (comma-separated)".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
            ],
            inputs: vec![],
            outputs: vec!["result".to_string(), "message_id".to_string()],
        }
    }

    async fn execute(
        &self,
        context: ghostflow_core::ExecutionContext,
    ) -> Result<HashMap<String, Value>> {
        let api_key = context.get_parameter("api_key")
            .and_then(|v| v.as_string())
            .ok_or("API key is required")?;
        
        let domain = context.get_parameter("domain")
            .and_then(|v| v.as_string())
            .ok_or("Domain is required")?;
        
        let region = context.get_parameter("region")
            .and_then(|v| v.as_string())
            .unwrap_or("us".to_string());
        
        let from = context.get_parameter("from")
            .and_then(|v| v.as_string())
            .ok_or("From address is required")?;
        
        let to = context.get_parameter("to")
            .and_then(|v| v.as_string())
            .ok_or("To address is required")?;
        
        let subject = context.get_parameter("subject")
            .and_then(|v| v.as_string())
            .ok_or("Subject is required")?;

        let base_url = match region.as_str() {
            "eu" => "https://api.eu.mailgun.net/v3",
            _ => "https://api.mailgun.net/v3",
        };

        let client = reqwest::Client::new();
        let mut form = vec![
            ("from", from),
            ("to", to),
            ("subject", subject),
        ];

        if let Some(cc) = context.get_parameter("cc").and_then(|v| v.as_string()) {
            form.push(("cc", cc));
        }
        
        if let Some(bcc) = context.get_parameter("bcc").and_then(|v| v.as_string()) {
            form.push(("bcc", bcc));
        }
        
        if let Some(text) = context.get_parameter("text").and_then(|v| v.as_string()) {
            form.push(("text", text));
        }
        
        if let Some(html) = context.get_parameter("html").and_then(|v| v.as_string()) {
            form.push(("html", html));
        }
        
        if let Some(tags) = context.get_parameter("tags").and_then(|v| v.as_string()) {
            for tag in tags.split(',') {
                form.push(("o:tag", tag.trim().to_string()));
            }
        }

        let response = client
            .post(&format!("{}/{}/messages", base_url, domain))
            .basic_auth("api", Some(&api_key))
            .form(&form)
            .send()
            .await?;

        let status = response.status();
        let success = status.is_success();
        let response_data: serde_json::Value = response.json().await?;

        let message_id = response_data.get("id")
            .and_then(|id| id.as_str())
            .map(|s| s.to_string());

        let result = json!({
            "success": success,
            "status": status.as_u16(),
            "response": response_data,
            "message_id": message_id
        });

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Object(result));
        
        if let Some(msg_id) = message_id {
            outputs.insert("message_id".to_string(), Value::String(msg_id));
        }
        
        Ok(outputs)
    }
}