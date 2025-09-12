use ghostflow_core::{Node, NodeDefinition, NodeParameter, ParameterType, Result, Value};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureVMNode;

#[async_trait]
impl Node for AzureVMNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            name: "azure_vm".to_string(),
            display_name: "Azure Virtual Machine".to_string(),
            description: "Manage Azure Virtual Machines".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                NodeParameter {
                    name: "access_token".to_string(),
                    display_name: "Access Token".to_string(),
                    description: "Azure OAuth2 access token".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "subscription_id".to_string(),
                    display_name: "Subscription ID".to_string(),
                    description: "Azure subscription ID".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "resource_group".to_string(),
                    display_name: "Resource Group".to_string(),
                    description: "Azure resource group name".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
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
                    name: "vm_name".to_string(),
                    display_name: "VM Name".to_string(),
                    description: "Virtual machine name".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "vm_size".to_string(),
                    display_name: "VM Size".to_string(),
                    description: "Azure VM size (Standard_B1s, Standard_D2s_v3, etc.)".to_string(),
                    parameter_type: ParameterType::Select,
                    required: false,
                    default_value: Some(Value::String("Standard_B1s".to_string())),
                },
                NodeParameter {
                    name: "location".to_string(),
                    display_name: "Location".to_string(),
                    description: "Azure region".to_string(),
                    parameter_type: ParameterType::Select,
                    required: false,
                    default_value: Some(Value::String("eastus".to_string())),
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
        
        let subscription_id = context.get_parameter("subscription_id")
            .and_then(|v| v.as_string())
            .ok_or("Subscription ID is required")?;
        
        let operation = context.get_parameter("operation")
            .and_then(|v| v.as_string())
            .unwrap_or("list".to_string());

        let client = reqwest::Client::new();
        let base_url = format!("https://management.azure.com/subscriptions/{}", subscription_id);

        let result = match operation.as_str() {
            "list" => {
                let url = if let Some(rg) = context.get_parameter("resource_group").and_then(|v| v.as_string()) {
                    format!("{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines", base_url, rg)
                } else {
                    format!("{}/providers/Microsoft.Compute/virtualMachines", base_url)
                };

                let response = client
                    .get(&url)
                    .header("Authorization", format!("Bearer {}", access_token))
                    .query(&[("api-version", "2023-03-01")])
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "get" => {
                let resource_group = context.get_parameter("resource_group")
                    .and_then(|v| v.as_string())
                    .ok_or("Resource group is required for get operation")?;
                
                let vm_name = context.get_parameter("vm_name")
                    .and_then(|v| v.as_string())
                    .ok_or("VM name is required for get operation")?;

                let response = client
                    .get(&format!("{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}", 
                        base_url, resource_group, vm_name))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .query(&[("api-version", "2023-03-01")])
                    .send()
                    .await?;

                let data: serde_json::Value = response.json().await?;
                data
            },
            "start" => {
                let resource_group = context.get_parameter("resource_group")
                    .and_then(|v| v.as_string())
                    .ok_or("Resource group is required for start operation")?;
                
                let vm_name = context.get_parameter("vm_name")
                    .and_then(|v| v.as_string())
                    .ok_or("VM name is required for start operation")?;

                let response = client
                    .post(&format!("{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}/start", 
                        base_url, resource_group, vm_name))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .query(&[("api-version", "2023-03-01")])
                    .send()
                    .await?;

                json!({
                    "success": response.status().is_success(),
                    "status": response.status().as_u16(),
                    "operation": "start"
                })
            },
            "stop" => {
                let resource_group = context.get_parameter("resource_group")
                    .and_then(|v| v.as_string())
                    .ok_or("Resource group is required for stop operation")?;
                
                let vm_name = context.get_parameter("vm_name")
                    .and_then(|v| v.as_string())
                    .ok_or("VM name is required for stop operation")?;

                let response = client
                    .post(&format!("{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}/powerOff", 
                        base_url, resource_group, vm_name))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .query(&[("api-version", "2023-03-01")])
                    .send()
                    .await?;

                json!({
                    "success": response.status().is_success(),
                    "status": response.status().as_u16(),
                    "operation": "stop"
                })
            },
            "restart" => {
                let resource_group = context.get_parameter("resource_group")
                    .and_then(|v| v.as_string())
                    .ok_or("Resource group is required for restart operation")?;
                
                let vm_name = context.get_parameter("vm_name")
                    .and_then(|v| v.as_string())
                    .ok_or("VM name is required for restart operation")?;

                let response = client
                    .post(&format!("{}/resourceGroups/{}/providers/Microsoft.Compute/virtualMachines/{}/restart", 
                        base_url, resource_group, vm_name))
                    .header("Authorization", format!("Bearer {}", access_token))
                    .query(&[("api-version", "2023-03-01")])
                    .send()
                    .await?;

                json!({
                    "success": response.status().is_success(),
                    "status": response.status().as_u16(),
                    "operation": "restart"
                })
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
pub struct AzureStorageNode;

#[async_trait]
impl Node for AzureStorageNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            name: "azure_storage".to_string(),
            display_name: "Azure Blob Storage".to_string(),
            description: "Manage Azure Blob Storage containers and files".to_string(),
            category: "integrations".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                NodeParameter {
                    name: "account_name".to_string(),
                    display_name: "Storage Account Name".to_string(),
                    description: "Azure storage account name".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "account_key".to_string(),
                    display_name: "Account Key".to_string(),
                    description: "Azure storage account key".to_string(),
                    parameter_type: ParameterType::String,
                    required: true,
                    default_value: None,
                },
                NodeParameter {
                    name: "operation".to_string(),
                    display_name: "Operation".to_string(),
                    description: "Storage operation to perform".to_string(),
                    parameter_type: ParameterType::Select,
                    required: true,
                    default_value: Some(Value::String("list_containers".to_string())),
                },
                NodeParameter {
                    name: "container_name".to_string(),
                    display_name: "Container Name".to_string(),
                    description: "Blob container name".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "blob_name".to_string(),
                    display_name: "Blob Name".to_string(),
                    description: "Blob file name".to_string(),
                    parameter_type: ParameterType::String,
                    required: false,
                    default_value: None,
                },
                NodeParameter {
                    name: "content".to_string(),
                    display_name: "Content".to_string(),
                    description: "File content to upload".to_string(),
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
        let account_name = context.get_parameter("account_name")
            .and_then(|v| v.as_string())
            .ok_or("Storage account name is required")?;
        
        let account_key = context.get_parameter("account_key")
            .and_then(|v| v.as_string())
            .ok_or("Account key is required")?;
        
        let operation = context.get_parameter("operation")
            .and_then(|v| v.as_string())
            .unwrap_or("list_containers".to_string());

        let client = reqwest::Client::new();
        let base_url = format!("https://{}.blob.core.windows.net", account_name);

        let result = match operation.as_str() {
            "list_containers" => {
                let auth_header = self.generate_auth_header(&account_name, &account_key, "GET", "/", "", "")?;
                
                let response = client
                    .get(&format!("{}/?comp=list", base_url))
                    .header("Authorization", auth_header)
                    .header("x-ms-date", chrono::Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string())
                    .header("x-ms-version", "2021-04-10")
                    .send()
                    .await?;

                let text = response.text().await?;
                json!({ "containers": text })
            },
            "list_blobs" => {
                let container_name = context.get_parameter("container_name")
                    .and_then(|v| v.as_string())
                    .ok_or("Container name is required for list blobs operation")?;

                let auth_header = self.generate_auth_header(&account_name, &account_key, "GET", &format!("/{}", container_name), "restype=container&comp=list", "")?;
                
                let response = client
                    .get(&format!("{}/{}?restype=container&comp=list", base_url, container_name))
                    .header("Authorization", auth_header)
                    .header("x-ms-date", chrono::Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string())
                    .header("x-ms-version", "2021-04-10")
                    .send()
                    .await?;

                let text = response.text().await?;
                json!({ "blobs": text })
            },
            "upload_blob" => {
                let container_name = context.get_parameter("container_name")
                    .and_then(|v| v.as_string())
                    .ok_or("Container name is required for upload operation")?;
                
                let blob_name = context.get_parameter("blob_name")
                    .and_then(|v| v.as_string())
                    .ok_or("Blob name is required for upload operation")?;
                
                let content = context.get_parameter("content")
                    .and_then(|v| v.as_string())
                    .ok_or("Content is required for upload operation")?;

                let auth_header = self.generate_auth_header(&account_name, &account_key, "PUT", &format!("/{}/{}", container_name, blob_name), "", &content)?;
                
                let response = client
                    .put(&format!("{}/{}/{}", base_url, container_name, blob_name))
                    .header("Authorization", auth_header)
                    .header("x-ms-date", chrono::Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string())
                    .header("x-ms-version", "2021-04-10")
                    .header("x-ms-blob-type", "BlockBlob")
                    .header("Content-Length", content.len().to_string())
                    .body(content)
                    .send()
                    .await?;

                json!({
                    "success": response.status().is_success(),
                    "status": response.status().as_u16()
                })
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

impl AzureStorageNode {
    fn generate_auth_header(&self, account_name: &str, account_key: &str, method: &str, path: &str, query: &str, body: &str) -> Result<String> {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;
        
        let date = chrono::Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();
        let content_length = if body.is_empty() { "0" } else { &body.len().to_string() };
        
        let string_to_sign = format!(
            "{}\n\n\n{}\n\n\n\n\n\n\n\n\nx-ms-date:{}\nx-ms-version:2021-04-10\n{}{}",
            method,
            content_length,
            date,
            path,
            if query.is_empty() { String::new() } else { format!("\n{}", query) }
        );

        let decoded_key = base64::decode(account_key)
            .map_err(|e| format!("Failed to decode account key: {}", e))?;
        
        let mut mac = Hmac::<Sha256>::new_from_slice(&decoded_key)
            .map_err(|e| format!("Failed to create HMAC: {}", e))?;
        
        mac.update(string_to_sign.as_bytes());
        let signature = base64::encode(mac.finalize().into_bytes());
        
        Ok(format!("SharedKey {}:{}", account_name, signature))
    }
}