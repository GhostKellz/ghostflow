use async_trait::async_trait;
use ghostflow_core::{GhostFlowError, Node, Result};
use ghostflow_schema::{
    DataType, ExecutionContext, NodeCategory, NodeDefinition, NodeParameter, NodePort,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::process::Stdio;
use tokio::process::Command;
use tracing::{error, info};

pub struct JarvisNode;

impl JarvisNode {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JarvisCommand {
    pub command: String,
    pub args: Vec<String>,
    pub env: Option<std::collections::HashMap<String, String>>,
    pub working_dir: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JarvisResponse {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub execution_time_ms: u64,
}

#[async_trait]
impl Node for JarvisNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            id: "jarvis_command".to_string(),
            name: "Jarvis Command".to_string(),
            description: "Execute Jarvis CLI commands for Rust-based automation".to_string(),
            category: NodeCategory::Action,
            version: "1.0.0".to_string(),
            inputs: vec![NodePort {
                name: "input".to_string(),
                display_name: "Input Data".to_string(),
                description: Some("Input data to pass to Jarvis command".to_string()),
                data_type: DataType::Any,
                required: false,
            }],
            outputs: vec![NodePort {
                name: "result".to_string(),
                display_name: "Command Result".to_string(),
                description: Some("Output from Jarvis command execution".to_string()),
                data_type: DataType::Object,
                required: true,
            }],
            parameters: vec![
                NodeParameter {
                    name: "command".to_string(),
                    display_name: "Command".to_string(),
                    description: Some("Jarvis command to execute".to_string()),
                    param_type: ghostflow_schema::node::ParameterType::String,
                    default_value: Some(Value::String("jarvis".to_string())),
                    required: true,
                    options: None,
                    validation: None,
                },
                NodeParameter {
                    name: "args".to_string(),
                    display_name: "Arguments".to_string(),
                    description: Some("Command arguments (comma-separated)".to_string()),
                    param_type: ghostflow_schema::node::ParameterType::String,
                    default_value: None,
                    required: false,
                    options: None,
                    validation: None,
                },
                NodeParameter {
                    name: "working_dir".to_string(),
                    display_name: "Working Directory".to_string(),
                    description: Some("Directory to execute command in".to_string()),
                    param_type: ghostflow_schema::node::ParameterType::String,
                    default_value: None,
                    required: false,
                    options: None,
                    validation: None,
                },
                NodeParameter {
                    name: "timeout_seconds".to_string(),
                    display_name: "Timeout (seconds)".to_string(),
                    description: Some("Command timeout in seconds".to_string()),
                    param_type: ghostflow_schema::node::ParameterType::Number,
                    default_value: Some(Value::Number(serde_json::Number::from(60))),
                    required: false,
                    options: None,
                    validation: None,
                },
            ],
            icon: Some("terminal".to_string()),
            color: Some("#ef4444".to_string()), // Red for Rust
        }
    }

    async fn validate(&self, context: &ExecutionContext) -> Result<()> {
        let params = &context.input;
        
        if let Some(command) = params.get("command") {
            if command.as_str().is_none() {
                return Err(GhostFlowError::ValidationError {
                    message: "Command must be a string".to_string(),
                });
            }
        } else {
            return Err(GhostFlowError::ValidationError {
                message: "Command is required".to_string(),
            });
        }

        Ok(())
    }

    async fn execute(&self, context: ExecutionContext) -> Result<serde_json::Value> {
        let params = &context.input;
        let start_time = std::time::Instant::now();
        
        let command = params
            .get("command")
            .and_then(|v| v.as_str())
            .unwrap_or("jarvis");

        let args_str = params
            .get("args")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        let args: Vec<String> = if args_str.is_empty() {
            vec![]
        } else {
            args_str.split(',').map(|s| s.trim().to_string()).collect()
        };

        let working_dir = params
            .get("working_dir")
            .and_then(|v| v.as_str());

        let timeout_seconds = params
            .get("timeout_seconds")
            .and_then(|v| v.as_u64())
            .unwrap_or(60);

        info!("Executing Jarvis command: {} {:?}", command, args);

        // Build the command
        let mut cmd = Command::new(command);
        cmd.args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if let Some(dir) = working_dir {
            cmd.current_dir(dir);
        }

        // Pass input data as JSON to stdin if available
        let input_json = params.get("input").cloned().unwrap_or(Value::Null);
        
        // Execute with timeout
        let output = tokio::time::timeout(
            std::time::Duration::from_secs(timeout_seconds),
            cmd.output()
        ).await
        .map_err(|_| GhostFlowError::TimeoutError {
            timeout_ms: timeout_seconds * 1000,
        })?
        .map_err(|e| {
            error!("Failed to execute Jarvis command: {}", e);
            GhostFlowError::NodeExecutionError {
                node_id: context.node_id.clone(),
                message: format!("Command execution failed: {}", e),
            }
        })?;

        let execution_time_ms = start_time.elapsed().as_millis() as u64;

        let response = JarvisResponse {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
            execution_time_ms,
        };

        info!("Jarvis command completed with exit code: {}", response.exit_code);

        // Parse stdout as JSON if possible, otherwise return as string
        let result_data = if !response.stdout.is_empty() {
            serde_json::from_str::<Value>(&response.stdout)
                .unwrap_or_else(|_| Value::String(response.stdout.clone()))
        } else {
            Value::Null
        };

        Ok(serde_json::json!({
            "success": response.exit_code == 0,
            "data": result_data,
            "stdout": response.stdout,
            "stderr": response.stderr,
            "exit_code": response.exit_code,
            "execution_time_ms": response.execution_time_ms,
            "command": {
                "executable": command,
                "args": args,
                "working_dir": working_dir,
            }
        }))
    }

    fn supports_retry(&self) -> bool {
        true
    }

    fn is_deterministic(&self) -> bool {
        false // External command execution is non-deterministic
    }
}

impl Default for JarvisNode {
    fn default() -> Self {
        Self::new()
    }
}

// Additional Jarvis-specific nodes can be added here
pub struct JarvisTaskNode;
pub struct JarvisAgentNode;
pub struct JarvisToolNode;

// Bridge for bi-directional communication with Jarvis
pub struct JarvisBridge {
    command_path: String,
    default_timeout: std::time::Duration,
}

impl JarvisBridge {
    pub fn new(command_path: impl Into<String>) -> Self {
        Self {
            command_path: command_path.into(),
            default_timeout: std::time::Duration::from_secs(60),
        }
    }

    pub async fn execute_task(&self, task: &str, context: Value) -> Result<Value> {
        let output = Command::new(&self.command_path)
            .arg("task")
            .arg(task)
            .arg("--json")
            .arg("--context")
            .arg(serde_json::to_string(&context).unwrap_or_default())
            .output()
            .await
            .map_err(|e| GhostFlowError::InternalError {
                message: format!("Failed to execute Jarvis task: {}", e),
            })?;

        if !output.status.success() {
            return Err(GhostFlowError::InternalError {
                message: format!("Jarvis task failed: {}", String::from_utf8_lossy(&output.stderr)),
            });
        }

        serde_json::from_slice(&output.stdout)
            .map_err(|e| GhostFlowError::SerializationError(e))
    }

    pub async fn list_available_tasks(&self) -> Result<Vec<String>> {
        let output = Command::new(&self.command_path)
            .arg("list-tasks")
            .arg("--json")
            .output()
            .await
            .map_err(|e| GhostFlowError::InternalError {
                message: format!("Failed to list Jarvis tasks: {}", e),
            })?;

        let tasks: Vec<String> = serde_json::from_slice(&output.stdout)
            .map_err(|e| GhostFlowError::SerializationError(e))?;
        
        Ok(tasks)
    }
}