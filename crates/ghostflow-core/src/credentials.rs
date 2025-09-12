use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use async_trait::async_trait;
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    pub id: String,
    pub name: String,
    pub credential_type: CredentialType,
    pub data: HashMap<String, String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub workspace_id: String,
    pub encrypted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialType {
    ApiKey,
    OAuth2,
    BasicAuth,
    BearerToken,
    DatabaseConnection,
    SshKey,
    AwsCredentials,
    AzureCredentials,
    GoogleServiceAccount,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2Credential {
    pub client_id: String,
    pub client_secret: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub authorization_url: String,
    pub token_url: String,
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialTemplate {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub icon: Option<String>,
    pub credential_type: CredentialType,
    pub fields: Vec<CredentialField>,
    pub oauth_config: Option<OAuth2Config>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialField {
    pub name: String,
    pub display_name: String,
    pub field_type: FieldType,
    pub required: bool,
    pub description: Option<String>,
    pub default_value: Option<String>,
    pub placeholder: Option<String>,
    pub validation: Option<FieldValidation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldType {
    String,
    Password,
    Number,
    Boolean,
    Select,
    MultiSelect,
    Json,
    File,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldValidation {
    pub pattern: Option<String>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2Config {
    pub authorization_url: String,
    pub token_url: String,
    pub scopes: Vec<String>,
    pub redirect_uri: String,
    pub pkce_enabled: bool,
}

#[async_trait]
pub trait CredentialVault: Send + Sync {
    async fn store(&self, credential: Credential) -> Result<String>;
    async fn retrieve(&self, id: &str) -> Result<Option<Credential>>;
    async fn update(&self, id: &str, credential: Credential) -> Result<()>;
    async fn delete(&self, id: &str) -> Result<()>;
    async fn list(&self, workspace_id: &str) -> Result<Vec<Credential>>;
    async fn search(&self, workspace_id: &str, query: &str) -> Result<Vec<Credential>>;
    async fn encrypt(&self, data: &str) -> Result<String>;
    async fn decrypt(&self, data: &str) -> Result<String>;
    async fn refresh_oauth_token(&self, credential_id: &str) -> Result<OAuth2Credential>;
}

#[derive(Clone)]
pub struct SecureVault {
    encryption_key: Vec<u8>,
    storage_backend: StorageBackend,
}

#[derive(Clone)]
pub enum StorageBackend {
    PostgreSQL { connection_string: String },
    Redis { connection_string: String },
    FileSystem { path: String },
    Memory,
}

impl SecureVault {
    pub fn new(encryption_key: Vec<u8>, storage_backend: StorageBackend) -> Self {
        Self {
            encryption_key,
            storage_backend,
        }
    }

    fn encrypt_internal(&self, data: &str) -> Result<String> {
        use aes_gcm::{
            aead::{Aead, KeyInit, OsRng},
            Aes256Gcm, Nonce,
        };
        use rand::RngCore;

        let cipher = Aes256Gcm::new_from_slice(&self.encryption_key)
            .map_err(|e| format!("Failed to create cipher: {}", e))?;

        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, data.as_bytes())
            .map_err(|e| format!("Failed to encrypt: {}", e))?;

        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);
        
        Ok(base64::encode(result))
    }

    fn decrypt_internal(&self, data: &str) -> Result<String> {
        use aes_gcm::{
            aead::{Aead, KeyInit},
            Aes256Gcm, Nonce,
        };

        let encrypted = base64::decode(data)
            .map_err(|e| format!("Failed to decode base64: {}", e))?;

        if encrypted.len() < 12 {
            return Err("Invalid encrypted data".into());
        }

        let (nonce_bytes, ciphertext) = encrypted.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let cipher = Aes256Gcm::new_from_slice(&self.encryption_key)
            .map_err(|e| format!("Failed to create cipher: {}", e))?;

        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| format!("Failed to decrypt: {}", e))?;

        String::from_utf8(plaintext)
            .map_err(|e| format!("Failed to convert to string: {}", e).into())
    }
}

#[async_trait]
impl CredentialVault for SecureVault {
    async fn store(&self, mut credential: Credential) -> Result<String> {
        for (key, value) in credential.data.iter_mut() {
            *value = self.encrypt_internal(value)?;
        }
        credential.encrypted = true;
        
        match &self.storage_backend {
            StorageBackend::PostgreSQL { connection_string } => {
                // Implementation for PostgreSQL storage
                todo!("PostgreSQL storage implementation")
            }
            StorageBackend::Memory => {
                // Simple in-memory storage for development
                Ok(credential.id.clone())
            }
            _ => todo!("Other storage backends")
        }
    }

    async fn retrieve(&self, id: &str) -> Result<Option<Credential>> {
        match &self.storage_backend {
            StorageBackend::PostgreSQL { connection_string } => {
                // Implementation for PostgreSQL retrieval
                todo!("PostgreSQL retrieval implementation")
            }
            StorageBackend::Memory => {
                // Simple in-memory retrieval for development
                Ok(None)
            }
            _ => todo!("Other storage backends")
        }
    }

    async fn update(&self, id: &str, credential: Credential) -> Result<()> {
        self.store(credential).await?;
        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<()> {
        match &self.storage_backend {
            StorageBackend::PostgreSQL { connection_string } => {
                // Implementation for PostgreSQL deletion
                todo!("PostgreSQL deletion implementation")
            }
            StorageBackend::Memory => {
                Ok(())
            }
            _ => todo!("Other storage backends")
        }
    }

    async fn list(&self, workspace_id: &str) -> Result<Vec<Credential>> {
        match &self.storage_backend {
            StorageBackend::PostgreSQL { connection_string } => {
                // Implementation for PostgreSQL listing
                todo!("PostgreSQL listing implementation")
            }
            StorageBackend::Memory => {
                Ok(Vec::new())
            }
            _ => todo!("Other storage backends")
        }
    }

    async fn search(&self, workspace_id: &str, query: &str) -> Result<Vec<Credential>> {
        let all = self.list(workspace_id).await?;
        Ok(all.into_iter()
            .filter(|c| c.name.contains(query))
            .collect())
    }

    async fn encrypt(&self, data: &str) -> Result<String> {
        self.encrypt_internal(data)
    }

    async fn decrypt(&self, data: &str) -> Result<String> {
        self.decrypt_internal(data)
    }

    async fn refresh_oauth_token(&self, credential_id: &str) -> Result<OAuth2Credential> {
        let credential = self.retrieve(credential_id).await?
            .ok_or("Credential not found")?;
        
        // OAuth2 token refresh implementation
        todo!("OAuth2 token refresh implementation")
    }
}

pub fn get_credential_templates() -> Vec<CredentialTemplate> {
    vec![
        CredentialTemplate {
            id: "cloudflare".to_string(),
            name: "cloudflare".to_string(),
            display_name: "Cloudflare".to_string(),
            description: "Cloudflare API credentials".to_string(),
            icon: Some("cloudflare.svg".to_string()),
            credential_type: CredentialType::ApiKey,
            fields: vec![
                CredentialField {
                    name: "api_token".to_string(),
                    display_name: "API Token".to_string(),
                    field_type: FieldType::Password,
                    required: true,
                    description: Some("Cloudflare API token with required permissions".to_string()),
                    default_value: None,
                    placeholder: Some("Enter your Cloudflare API token".to_string()),
                    validation: None,
                },
                CredentialField {
                    name: "zone_id".to_string(),
                    display_name: "Zone ID (Optional)".to_string(),
                    field_type: FieldType::String,
                    required: false,
                    description: Some("Default Zone ID for operations".to_string()),
                    default_value: None,
                    placeholder: Some("Enter Zone ID".to_string()),
                    validation: None,
                },
            ],
            oauth_config: None,
        },
        CredentialTemplate {
            id: "microsoft_graph".to_string(),
            name: "microsoft_graph".to_string(),
            display_name: "Microsoft 365".to_string(),
            description: "Microsoft Graph API OAuth2 credentials".to_string(),
            icon: Some("microsoft.svg".to_string()),
            credential_type: CredentialType::OAuth2,
            fields: vec![
                CredentialField {
                    name: "tenant_id".to_string(),
                    display_name: "Tenant ID".to_string(),
                    field_type: FieldType::String,
                    required: true,
                    description: Some("Azure AD tenant ID".to_string()),
                    default_value: None,
                    placeholder: Some("xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx".to_string()),
                    validation: None,
                },
            ],
            oauth_config: Some(OAuth2Config {
                authorization_url: "https://login.microsoftonline.com/{tenant_id}/oauth2/v2.0/authorize".to_string(),
                token_url: "https://login.microsoftonline.com/{tenant_id}/oauth2/v2.0/token".to_string(),
                scopes: vec![
                    "https://graph.microsoft.com/Mail.Send".to_string(),
                    "https://graph.microsoft.com/Mail.Read".to_string(),
                    "https://graph.microsoft.com/Calendars.ReadWrite".to_string(),
                    "https://graph.microsoft.com/Teams.ReadWrite".to_string(),
                ],
                redirect_uri: "http://localhost:8080/oauth/callback".to_string(),
                pkce_enabled: true,
            }),
        },
        CredentialTemplate {
            id: "discord".to_string(),
            name: "discord".to_string(),
            display_name: "Discord".to_string(),
            description: "Discord bot and webhook credentials".to_string(),
            icon: Some("discord.svg".to_string()),
            credential_type: CredentialType::Custom("discord".to_string()),
            fields: vec![
                CredentialField {
                    name: "bot_token".to_string(),
                    display_name: "Bot Token".to_string(),
                    field_type: FieldType::Password,
                    required: false,
                    description: Some("Discord bot token for advanced operations".to_string()),
                    default_value: None,
                    placeholder: Some("Enter bot token".to_string()),
                    validation: None,
                },
                CredentialField {
                    name: "webhook_url".to_string(),
                    display_name: "Webhook URL".to_string(),
                    field_type: FieldType::String,
                    required: false,
                    description: Some("Discord webhook URL for simple messages".to_string()),
                    default_value: None,
                    placeholder: Some("https://discord.com/api/webhooks/...".to_string()),
                    validation: None,
                },
            ],
            oauth_config: None,
        },
    ]
}