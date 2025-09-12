use ghostflow_core::{Node, NodeDefinition, NodeParameter, ParameterType, Result, Value};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxmoxVMNode;

#[async_trait]
impl Node for ProxmoxVMNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            name: "proxmox_vm".to_string(),
            display_name: "Proxmox VM".to_string(),
            description: "Manage Proxmox Virtual Machines".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                NodeParameter {
                    name: "host".to_string(),
                    display_name: "Proxmox Host".to_string(),
                    description: "Proxmox server hostname or IP".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "port".to_string(),
                    display_name: "Port".to_string(),
                    description: "Proxmox API port".to_string(),
                    parameter_type: ParameterType::Number,
                    required: false,
                    default_value: Some(Value::Number(8006.0)),
                },
                NodeParameter {
                    name: "username".to_string(),
                    display_name: "Username".to_string(),
                    description: "Proxmox username (user@pam or user@pve)".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "password".to_string(),
                    display_name: "Password".to_string(),
                    description: "Proxmox password or API token".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "operation".to_string(),
                    display_name: "Operation".to_string(),
                    description: "VM operation to perform".to_string(),
                    parameter_type: ParameterType::Select,
                    required: true,
                    default_value: Some(Value::String("list".to_string())),
                },
                NodeParameter {
                    name: "node".to_string(),
                    display_name: "Node".to_string(),
                    description: "Proxmox node name".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "vmid".to_string(),
                    display_name: "VM ID".to_string(),
                    description: "Virtual machine ID".to_string(),
                    parameter_type: ParameterType::Number,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "vm_name".to_string(),
                    display_name: "VM Name".to_string(),
                    description: "Virtual machine name".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
            ],
            inputs: vec![],
            outputs: vec!["result".to_string(), "vm_status".to_string()],
        }
    }

    async fn execute(
        &self,
        context: ghostflow_core::ExecutionContext,
    ) -> Result<HashMap<String, Value>> {
        let host = context.get_parameter("host")
            .and_then(|v| v.as_string())
            .ok_or("Proxmox host is required")?;
        
        let port = context.get_parameter("port")
            .and_then(|v| v.as_number())
            .unwrap_or(8006.0) as u16;
        
        let username = context.get_parameter("username")
            .and_then(|v| v.as_string())
            .ok_or("Username is required")?;
        
        let password = context.get_parameter("password")
            .and_then(|v| v.as_string())
            .ok_or("Password is required")?;
        
        let operation = context.get_parameter("operation")
            .and_then(|v| v.as_string())
            .unwrap_or("list".to_string());

        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true) // Proxmox often uses self-signed certs
            .build()?;

        let base_url = format!("https://{}:{}/api2/json", host, port);

        // Authenticate and get ticket
        let auth_response = client
            .post(&format!("{}/access/ticket", base_url))
            .form(&[
                ("username", username.as_str()),
                ("password", password.as_str()),
            ])
            .send()
            .await?;

        let auth_data: serde_json::Value = auth_response.json().await?;
        let ticket = auth_data["data"]["ticket"]
            .as_str()
            .ok_or("Failed to get authentication ticket")?;
        let csrf_token = auth_data["data"]["CSRFPreventionToken"]
            .as_str()
            .ok_or("Failed to get CSRF token")?;

        let result = match operation.as_str() {
            "list" => {
                let url = if let Some(node) = context.get_parameter("node").and_then(|v| v.as_string()) {
                    format!("{}/nodes/{}/qemu", base_url, node)
                } else {
                    // List all VMs across all nodes
                    let nodes_response = client
                        .get(&format!("{}/nodes", base_url))
                        .header("Cookie", format!("PVEAuthCookie={}", ticket))
                        .send()
                        .await?;

                    let nodes_data: serde_json::Value = nodes_response.json().await?;
                    let mut all_vms = Vec::new();

                    if let Some(nodes) = nodes_data["data"].as_array() {
                        for node in nodes {
                            if let Some(node_name) = node["node"].as_str() {
                                let vms_response = client
                                    .get(&format!("{}/nodes/{}/qemu", base_url, node_name))
                                    .header("Cookie", format!("PVEAuthCookie={}", ticket))
                                    .send()
                                    .await?;

                                if let Ok(vms_data) = vms_response.json::<serde_json::Value>().await {
                                    if let Some(vms) = vms_data["data"].as_array() {
                                        for vm in vms {
                                            all_vms.push(vm.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }

                    return Ok({
                        let mut outputs = HashMap::new();
                        outputs.insert("result".to_string(), Value::Object(json!({
                            "data": all_vms
                        })));
                        outputs
                    });
                };

                let response = client
                    .get(&url)
                    .header("Cookie", format!("PVEAuthCookie={}", ticket))
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "get" => {
                let node = context.get_parameter("node")
                    .and_then(|v| v.as_string())
                    .ok_or("Node is required for get operation")?;
                
                let vmid = context.get_parameter("vmid")
                    .and_then(|v| v.as_number())
                    .ok_or("VM ID is required for get operation")? as u32;

                let response = client
                    .get(&format!("{}/nodes/{}/qemu/{}/status/current", base_url, node, vmid))
                    .header("Cookie", format!("PVEAuthCookie={}", ticket))
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "start" => {
                let node = context.get_parameter("node")
                    .and_then(|v| v.as_string())
                    .ok_or("Node is required for start operation")?;
                
                let vmid = context.get_parameter("vmid")
                    .and_then(|v| v.as_number())
                    .ok_or("VM ID is required for start operation")? as u32;

                let response = client
                    .post(&format!("{}/nodes/{}/qemu/{}/status/start", base_url, node, vmid))
                    .header("Cookie", format!("PVEAuthCookie={}", ticket))
                    .header("CSRFPreventionToken", csrf_token)
                    .send()
                    .await?;

                json!({
                    "success": response.status().is_success(),
                    "status": response.status().as_u16(),
                    "operation": "start",
                    "vmid": vmid,
                    "node": node
                })
            },
            "stop" => {
                let node = context.get_parameter("node")
                    .and_then(|v| v.as_string())
                    .ok_or("Node is required for stop operation")?;
                
                let vmid = context.get_parameter("vmid")
                    .and_then(|v| v.as_number())
                    .ok_or("VM ID is required for stop operation")? as u32;

                let response = client
                    .post(&format!("{}/nodes/{}/qemu/{}/status/stop", base_url, node, vmid))
                    .header("Cookie", format!("PVEAuthCookie={}", ticket))
                    .header("CSRFPreventionToken", csrf_token)
                    .send()
                    .await?;

                json!({
                    "success": response.status().is_success(),
                    "status": response.status().as_u16(),
                    "operation": "stop",
                    "vmid": vmid,
                    "node": node
                })
            },
            "restart" => {
                let node = context.get_parameter("node")
                    .and_then(|v| v.as_string())
                    .ok_or("Node is required for restart operation")?;
                
                let vmid = context.get_parameter("vmid")
                    .and_then(|v| v.as_number())
                    .ok_or("VM ID is required for restart operation")? as u32;

                let response = client
                    .post(&format!("{}/nodes/{}/qemu/{}/status/reboot", base_url, node, vmid))
                    .header("Cookie", format!("PVEAuthCookie={}", ticket))
                    .header("CSRFPreventionToken", csrf_token)
                    .send()
                    .await?;

                json!({
                    "success": response.status().is_success(),
                    "status": response.status().as_u16(),
                    "operation": "restart",
                    "vmid": vmid,
                    "node": node
                })
            },
            "clone" => {
                let node = context.get_parameter("node")
                    .and_then(|v| v.as_string())
                    .ok_or("Node is required for clone operation")?;
                
                let vmid = context.get_parameter("vmid")
                    .and_then(|v| v.as_number())
                    .ok_or("VM ID is required for clone operation")? as u32;
                
                let new_vmid = context.get_parameter("new_vmid")
                    .and_then(|v| v.as_number())
                    .ok_or("New VM ID is required for clone operation")? as u32;

                let mut params = vec![
                    ("newid", new_vmid.to_string()),
                ];

                if let Some(name) = context.get_parameter("vm_name").and_then(|v| v.as_string()) {
                    params.push(("name", name));
                }

                let response = client
                    .post(&format!("{}/nodes/{}/qemu/{}/clone", base_url, node, vmid))
                    .header("Cookie", format!("PVEAuthCookie={}", ticket))
                    .header("CSRFPreventionToken", csrf_token)
                    .form(&params)
                    .send()
                    .await?;

                json!({
                    "success": response.status().is_success(),
                    "status": response.status().as_u16(),
                    "operation": "clone",
                    "source_vmid": vmid,
                    "new_vmid": new_vmid,
                    "node": node
                })
            },
            "snapshot" => {
                let node = context.get_parameter("node")
                    .and_then(|v| v.as_string())
                    .ok_or("Node is required for snapshot operation")?;
                
                let vmid = context.get_parameter("vmid")
                    .and_then(|v| v.as_number())
                    .ok_or("VM ID is required for snapshot operation")? as u32;
                
                let snapname = context.get_parameter("snapname")
                    .and_then(|v| v.as_string())
                    .unwrap_or_else(|| format!("ghostflow-{}", chrono::Utc::now().timestamp()));

                let response = client
                    .post(&format!("{}/nodes/{}/qemu/{}/snapshot", base_url, node, vmid))
                    .header("Cookie", format!("PVEAuthCookie={}", ticket))
                    .header("CSRFPreventionToken", csrf_token)
                    .form(&[("snapname", &snapname)])
                    .send()
                    .await?;

                json!({
                    "success": response.status().is_success(),
                    "status": response.status().as_u16(),
                    "operation": "snapshot",
                    "vmid": vmid,
                    "snapshot_name": snapname,
                    "node": node
                })
            },
            _ => {
                return Err(format!("Unknown operation: {}", operation).into());
            }
        };

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Object(result.clone()));
        
        // Extract VM status if available
        if let Some(status_data) = result.get("data") {
            if let Some(status) = status_data.get("status").and_then(|s| s.as_str()) {
                outputs.insert("vm_status".to_string(), Value::String(status.to_string()));
            }
        }
        
        Ok(outputs)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxmoxContainerNode;

#[async_trait]
impl Node for ProxmoxContainerNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            name: "proxmox_container".to_string(),
            display_name: "Proxmox LXC Container".to_string(),
            description: "Manage Proxmox LXC Containers".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                NodeParameter {
                    name: "host".to_string(),
                    display_name: "Proxmox Host".to_string(),
                    description: "Proxmox server hostname or IP".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "port".to_string(),
                    display_name: "Port".to_string(),
                    description: "Proxmox API port".to_string(),
                    parameter_type: ParameterType::Number,
                    required: false,
                    default_value: Some(Value::Number(8006.0)),
                },
                NodeParameter {
                    name: "username".to_string(),
                    display_name: "Username".to_string(),
                    description: "Proxmox username (user@pam or user@pve)".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "password".to_string(),
                    display_name: "Password".to_string(),
                    description: "Proxmox password or API token".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "operation".to_string(),
                    display_name: "Operation".to_string(),
                    description: "Container operation to perform".to_string(),
                    parameter_type: ParameterType::Select,
                    required: true,
                    default_value: Some(Value::String("list".to_string())),
                },
                NodeParameter {
                    name: "node".to_string(),
                    display_name: "Node".to_string(),
                    description: "Proxmox node name".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "vmid".to_string(),
                    display_name: "Container ID".to_string(),
                    description: "LXC container ID".to_string(),
                    parameter_type: ParameterType::Number,
                    required: false,
                    default_value: None,
                },
            ],
            inputs: vec![],
            outputs: vec!["result".to_string(), "container_status".to_string()],
        }
    }

    async fn execute(
        &self,
        context: ghostflow_core::ExecutionContext,
    ) -> Result<HashMap<String, Value>> {
        let host = context.get_parameter("host")
            .and_then(|v| v.as_string())
            .ok_or("Proxmox host is required")?;
        
        let port = context.get_parameter("port")
            .and_then(|v| v.as_number())
            .unwrap_or(8006.0) as u16;
        
        let username = context.get_parameter("username")
            .and_then(|v| v.as_string())
            .ok_or("Username is required")?;
        
        let password = context.get_parameter("password")
            .and_then(|v| v.as_string())
            .ok_or("Password is required")?;
        
        let operation = context.get_parameter("operation")
            .and_then(|v| v.as_string())
            .unwrap_or("list".to_string());

        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;

        let base_url = format!("https://{}:{}/api2/json", host, port);

        // Authenticate
        let auth_response = client
            .post(&format!("{}/access/ticket", base_url))
            .form(&[
                ("username", username.as_str()),
                ("password", password.as_str()),
            ])
            .send()
            .await?;

        let auth_data: serde_json::Value = auth_response.json().await?;
        let ticket = auth_data["data"]["ticket"]
            .as_str()
            .ok_or("Failed to get authentication ticket")?;
        let csrf_token = auth_data["data"]["CSRFPreventionToken"]
            .as_str()
            .ok_or("Failed to get CSRF token")?;

        let result = match operation.as_str() {
            "list" => {
                let url = if let Some(node) = context.get_parameter("node").and_then(|v| v.as_string()) {
                    format!("{}/nodes/{}/lxc", base_url, node)
                } else {
                    // List all containers across all nodes
                    let nodes_response = client
                        .get(&format!("{}/nodes", base_url))
                        .header("Cookie", format!("PVEAuthCookie={}", ticket))
                        .send()
                        .await?;

                    let nodes_data: serde_json::Value = nodes_response.json().await?;
                    let mut all_containers = Vec::new();

                    if let Some(nodes) = nodes_data["data"].as_array() {
                        for node in nodes {
                            if let Some(node_name) = node["node"].as_str() {
                                let containers_response = client
                                    .get(&format!("{}/nodes/{}/lxc", base_url, node_name))
                                    .header("Cookie", format!("PVEAuthCookie={}", ticket))
                                    .send()
                                    .await?;

                                if let Ok(containers_data) = containers_response.json::<serde_json::Value>().await {
                                    if let Some(containers) = containers_data["data"].as_array() {
                                        for container in containers {
                                            all_containers.push(container.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }

                    return Ok({
                        let mut outputs = HashMap::new();
                        outputs.insert("result".to_string(), Value::Object(json!({
                            "data": all_containers
                        })));
                        outputs
                    });
                };

                let response = client
                    .get(&url)
                    .header("Cookie", format!("PVEAuthCookie={}", ticket))
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "start" | "stop" | "restart" => {
                let node = context.get_parameter("node")
                    .and_then(|v| v.as_string())
                    .ok_or("Node is required for container operations")?;
                
                let vmid = context.get_parameter("vmid")
                    .and_then(|v| v.as_number())
                    .ok_or("Container ID is required for container operations")? as u32;

                let action = match operation.as_str() {
                    "restart" => "reboot",
                    op => op,
                };

                let response = client
                    .post(&format!("{}/nodes/{}/lxc/{}/status/{}", base_url, node, vmid, action))
                    .header("Cookie", format!("PVEAuthCookie={}", ticket))
                    .header("CSRFPreventionToken", csrf_token)
                    .send()
                    .await?;

                json!({
                    "success": response.status().is_success(),
                    "status": response.status().as_u16(),
                    "operation": operation,
                    "vmid": vmid,
                    "node": node
                })
            },
            _ => {
                return Err(format!("Unknown operation: {}", operation).into());
            }
        };

        let mut outputs = HashMap::new();
        outputs.insert("result".to_string(), Value::Object(result.clone()));
        
        if let Some(status_data) = result.get("data") {
            if let Some(status) = status_data.get("status").and_then(|s| s.as_str()) {
                outputs.insert("container_status".to_string(), Value::String(status.to_string()));
            }
        }
        
        Ok(outputs)
    }
}