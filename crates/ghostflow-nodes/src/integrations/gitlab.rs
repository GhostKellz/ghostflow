use ghostflow_core::{Node, NodeDefinition, NodeParameter, ParameterType, Result, Value};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitLabProjectNode;

#[async_trait]
impl Node for GitLabProjectNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            name: "gitlab_project".to_string(),
            display_name: "GitLab Project".to_string(),
            description: "Manage GitLab projects and repositories".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                NodeParameter {
                    name: "base_url".to_string(),
                    display_name: "GitLab URL".to_string(),
                    description: "GitLab instance URL".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: Some(Value::String("https://gitlab.com".to_string())),
                },
                NodeParameter {
                    name: "access_token".to_string(),
                    display_name: "Access Token".to_string(),
                    description: "GitLab personal access token".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "operation".to_string(),
                    display_name: "Operation".to_string(),
                    description: "GitLab operation to perform".to_string(),
                    parameter_type: ParameterType::Select,
                    required: true,
                    default_value: Some(Value::String("list_projects".to_string())),
                },
                NodeParameter {
                    name: "project_id".to_string(),
                    display_name: "Project ID".to_string(),
                    description: "GitLab project ID or path".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "branch".to_string(),
                    display_name: "Branch".to_string(),
                    description: "Git branch name".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: Some(Value::String("main".to_string())),
                },
                NodeParameter {
                    name: "commit_message".to_string(),
                    display_name: "Commit Message".to_string(),
                    description: "Commit message for file operations".to_string(),
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
        let base_url = context.get_parameter("base_url")
            .and_then(|v| v.as_string())
            .unwrap_or("https://gitlab.com".to_string());
        
        let access_token = context.get_parameter("access_token")
            .and_then(|v| v.as_string())
            .ok_or("Access token is required")?;
        
        let operation = context.get_parameter("operation")
            .and_then(|v| v.as_string())
            .unwrap_or("list_projects".to_string());

        let client = reqwest::Client::new();
        let api_base = format!("{}/api/v4", base_url);

        let result = match operation.as_str() {
            "list_projects" => {
                let response = client
                    .get(&format!("{}/projects", api_base))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .query(&[("membership", "true"), ("per_page", "50")])
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                json!({ "projects": data })
            },
            "get_project" => {
                let project_id = context.get_parameter("project_id")
                    .and_then(|v| v.as_string())
                    .ok_or("Project ID is required for get project operation")?;

                let encoded_project_id = urlencoding::encode(&project_id);
                let response = client
                    .get(&format!("{}/projects/{}", api_base, encoded_project_id))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "list_branches" => {
                let project_id = context.get_parameter("project_id")
                    .and_then(|v| v.as_string())
                    .ok_or("Project ID is required for list branches operation")?;

                let encoded_project_id = urlencoding::encode(&project_id);
                let response = client
                    .get(&format!("{}/projects/{}/repository/branches", api_base, encoded_project_id))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                json!({ "branches": data })
            },
            "list_commits" => {
                let project_id = context.get_parameter("project_id")
                    .and_then(|v| v.as_string())
                    .ok_or("Project ID is required for list commits operation")?;

                let branch = context.get_parameter("branch")
                    .and_then(|v| v.as_string())
                    .unwrap_or("main".to_string());

                let encoded_project_id = urlencoding::encode(&project_id);
                let response = client
                    .get(&format!("{}/projects/{}/repository/commits", api_base, encoded_project_id))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .query(&[("ref_name", &branch), ("per_page", "20")])
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                json!({ "commits": data })
            },
            "trigger_pipeline" => {
                let project_id = context.get_parameter("project_id")
                    .and_then(|v| v.as_string())
                    .ok_or("Project ID is required for trigger pipeline operation")?;

                let branch = context.get_parameter("branch")
                    .and_then(|v| v.as_string())
                    .unwrap_or("main".to_string());

                let encoded_project_id = urlencoding::encode(&project_id);
                let response = client
                    .post(&format!("{}/projects/{}/pipeline", api_base, encoded_project_id))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .json(&json!({
                        "ref": branch
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
        outputs.insert("result".to_string(), Value::Object(result));
        Ok(outputs)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitLabIssueNode;

#[async_trait]
impl Node for GitLabIssueNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            name: "gitlab_issue".to_string(),
            display_name: "GitLab Issues".to_string(),
            description: "Manage GitLab issues and merge requests".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                NodeParameter {
                    name: "base_url".to_string(),
                    display_name: "GitLab URL".to_string(),
                    description: "GitLab instance URL".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: Some(Value::String("https://gitlab.com".to_string())),
                },
                NodeParameter {
                    name: "access_token".to_string(),
                    display_name: "Access Token".to_string(),
                    description: "GitLab personal access token".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "operation".to_string(),
                    display_name: "Operation".to_string(),
                    description: "Issue operation to perform".to_string(),
                    parameter_type: ParameterType::Select,
                    required: true,
                    default_value: Some(Value::String("list_issues".to_string())),
                },
                NodeParameter {
                    name: "project_id".to_string(),
                    display_name: "Project ID".to_string(),
                    description: "GitLab project ID or path".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "issue_id".to_string(),
                    display_name: "Issue ID".to_string(),
                    description: "Issue internal ID".to_string(),
                    parameter_type: ParameterType::Number,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "title".to_string(),
                    display_name: "Title".to_string(),
                    description: "Issue or MR title".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "description".to_string(),
                    display_name: "Description".to_string(),
                    description: "Issue or MR description".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "labels".to_string(),
                    display_name: "Labels".to_string(),
                    description: "Comma-separated list of labels".to_string(),
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
        let base_url = context.get_parameter("base_url")
            .and_then(|v| v.as_string())
            .unwrap_or("https://gitlab.com".to_string());
        
        let access_token = context.get_parameter("access_token")
            .and_then(|v| v.as_string())
            .ok_or("Access token is required")?;
        
        let operation = context.get_parameter("operation")
            .and_then(|v| v.as_string())
            .unwrap_or("list_issues".to_string());
        
        let project_id = context.get_parameter("project_id")
            .and_then(|v| v.as_string())
            .ok_or("Project ID is required")?;

        let client = reqwest::Client::new();
        let api_base = format!("{}/api/v4", base_url);
        let encoded_project_id = urlencoding::encode(&project_id);

        let result = match operation.as_str() {
            "list_issues" => {
                let response = client
                    .get(&format!("{}/projects/{}/issues", api_base, encoded_project_id))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .query(&[("state", "opened"), ("per_page", "50")])
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                json!({ "issues": data })
            },
            "create_issue" => {
                let title = context.get_parameter("title")
                    .and_then(|v| v.as_string())
                    .ok_or("Title is required for create issue operation")?;
                
                let description = context.get_parameter("description")
                    .and_then(|v| v.as_string())
                    .unwrap_or_default();

                let mut body = json!({
                    "title": title,
                    "description": description
                });

                if let Some(labels) = context.get_parameter("labels").and_then(|v| v.as_string()) {
                    let label_list: Vec<&str> = labels.split(',').map(|l| l.trim()).collect();
                    body["labels"] = json!(label_list);
                }

                let response = client
                    .post(&format!("{}/projects/{}/issues", api_base, encoded_project_id))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .json(&body)
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "update_issue" => {
                let issue_id = context.get_parameter("issue_id")
                    .and_then(|v| v.as_number())
                    .ok_or("Issue ID is required for update operation")? as u32;

                let mut body = json!({});

                if let Some(title) = context.get_parameter("title").and_then(|v| v.as_string()) {
                    body["title"] = json!(title);
                }
                if let Some(description) = context.get_parameter("description").and_then(|v| v.as_string()) {
                    body["description"] = json!(description);
                }
                if let Some(labels) = context.get_parameter("labels").and_then(|v| v.as_string()) {
                    let label_list: Vec<&str> = labels.split(',').map(|l| l.trim()).collect();
                    body["labels"] = json!(label_list);
                }

                let response = client
                    .put(&format!("{}/projects/{}/issues/{}", api_base, encoded_project_id, issue_id))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .json(&body)
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "close_issue" => {
                let issue_id = context.get_parameter("issue_id")
                    .and_then(|v| v.as_number())
                    .ok_or("Issue ID is required for close operation")? as u32;

                let response = client
                    .put(&format!("{}/projects/{}/issues/{}", api_base, encoded_project_id, issue_id))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .json(&json!({
                        "state_event": "close"
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
        outputs.insert("result".to_string(), Value::Object(result));
        Ok(outputs)
    }
}