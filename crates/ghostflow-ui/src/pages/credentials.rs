use leptos::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialSummary {
    pub id: String,
    pub name: String,
    pub credential_type: String,
    pub description: Option<String>,
    pub created_at: String,
    pub last_used: Option<String>,
    pub is_encrypted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialTemplate {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub icon: String,
    pub fields: Vec<CredentialField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialField {
    pub name: String,
    pub display_name: String,
    pub field_type: String,
    pub required: bool,
    pub description: Option<String>,
    pub placeholder: Option<String>,
}

#[component]
pub fn CredentialsPage() -> impl IntoView {
    let (credentials, set_credentials) = create_signal(Vec::<CredentialSummary>::new());
    let (loading, set_loading) = create_signal(true);
    let (show_create_modal, set_show_create_modal) = create_signal(false);
    let (selected_template, set_selected_template) = create_signal(None::<CredentialTemplate>);
    let (search_term, set_search_term) = create_signal(String::new());
    
    // Load credentials on component mount
    create_effect(move |_| {
        spawn_local(async move {
            // Simulate API call
            gloo_timers::future::TimeoutFuture::new(800).await;
            
            let sample_credentials = vec![
                CredentialSummary {
                    id: "cred_001".to_string(),
                    name: "Cloudflare Production".to_string(),
                    credential_type: "cloudflare".to_string(),
                    description: Some("Production Cloudflare API credentials".to_string()),
                    created_at: "2024-01-15".to_string(),
                    last_used: Some("2 hours ago".to_string()),
                    is_encrypted: true,
                },
                CredentialSummary {
                    id: "cred_002".to_string(),
                    name: "Discord Bot Token".to_string(),
                    credential_type: "discord".to_string(),
                    description: Some("Bot token for alerts and notifications".to_string()),
                    created_at: "2024-01-14".to_string(),
                    last_used: Some("5 minutes ago".to_string()),
                    is_encrypted: true,
                },
                CredentialSummary {
                    id: "cred_003".to_string(),
                    name: "Microsoft 365 OAuth".to_string(),
                    credential_type: "microsoft_graph".to_string(),
                    description: Some("OAuth2 credentials for Microsoft Graph API".to_string()),
                    created_at: "2024-01-12".to_string(),
                    last_used: Some("1 day ago".to_string()),
                    is_encrypted: true,
                },
                CredentialSummary {
                    id: "cred_004".to_string(),
                    name: "PostgreSQL Database".to_string(),
                    credential_type: "postgresql".to_string(),
                    description: Some("Production database connection".to_string()),
                    created_at: "2024-01-10".to_string(),
                    last_used: Some("3 hours ago".to_string()),
                    is_encrypted: true,
                },
                CredentialSummary {
                    id: "cred_005".to_string(),
                    name: "SendGrid SMTP".to_string(),
                    credential_type: "sendgrid".to_string(),
                    description: Some("Email service for notifications".to_string()),
                    created_at: "2024-01-08".to_string(),
                    last_used: Some("6 hours ago".to_string()),
                    is_encrypted: true,
                },
            ];
            
            set_credentials.set(sample_credentials);
            set_loading.set(false);
        });
    });
    
    let filtered_credentials = move || {
        let credentials = credentials.get();
        let search = search_term.get().to_lowercase();
        
        if search.is_empty() {
            credentials
        } else {
            credentials.into_iter()
                .filter(|cred| {
                    cred.name.to_lowercase().contains(&search) ||
                    cred.credential_type.to_lowercase().contains(&search) ||
                    cred.description.as_ref().map_or(false, |d| d.to_lowercase().contains(&search))
                })
                .collect()
        }
    };
    
    let credential_templates = vec![
        CredentialTemplate {
            id: "cloudflare".to_string(),
            name: "cloudflare".to_string(),
            display_name: "Cloudflare".to_string(),
            description: "Cloudflare API credentials for DNS and security management".to_string(),
            icon: "☁️".to_string(),
            fields: vec![
                CredentialField {
                    name: "api_token".to_string(),
                    display_name: "API Token".to_string(),
                    field_type: "password".to_string(),
                    required: true,
                    description: Some("Cloudflare API token with required permissions".to_string()),
                    placeholder: Some("Enter your Cloudflare API token".to_string()),
                },
                CredentialField {
                    name: "zone_id".to_string(),
                    display_name: "Zone ID (Optional)".to_string(),
                    field_type: "string".to_string(),
                    required: false,
                    description: Some("Default Zone ID for operations".to_string()),
                    placeholder: Some("Enter Zone ID".to_string()),
                },
            ],
        },
        CredentialTemplate {
            id: "discord".to_string(),
            name: "discord".to_string(),
            display_name: "Discord".to_string(),
            description: "Discord bot and webhook credentials".to_string(),
            icon: "💬".to_string(),
            fields: vec![
                CredentialField {
                    name: "bot_token".to_string(),
                    display_name: "Bot Token".to_string(),
                    field_type: "password".to_string(),
                    required: false,
                    description: Some("Discord bot token for advanced operations".to_string()),
                    placeholder: Some("Enter bot token".to_string()),
                },
                CredentialField {
                    name: "webhook_url".to_string(),
                    display_name: "Webhook URL".to_string(),
                    field_type: "string".to_string(),
                    required: false,
                    description: Some("Discord webhook URL for simple messages".to_string()),
                    placeholder: Some("https://discord.com/api/webhooks/...".to_string()),
                },
            ],
        },
        CredentialTemplate {
            id: "postgresql".to_string(),
            name: "postgresql".to_string(),
            display_name: "PostgreSQL".to_string(),
            description: "PostgreSQL database connection credentials".to_string(),
            icon: "🐘".to_string(),
            fields: vec![
                CredentialField {
                    name: "host".to_string(),
                    display_name: "Host".to_string(),
                    field_type: "string".to_string(),
                    required: true,
                    description: Some("Database host or IP address".to_string()),
                    placeholder: Some("localhost".to_string()),
                },
                CredentialField {
                    name: "port".to_string(),
                    display_name: "Port".to_string(),
                    field_type: "number".to_string(),
                    required: false,
                    description: Some("Database port number".to_string()),
                    placeholder: Some("5432".to_string()),
                },
                CredentialField {
                    name: "database".to_string(),
                    display_name: "Database Name".to_string(),
                    field_type: "string".to_string(),
                    required: true,
                    description: Some("Name of the database".to_string()),
                    placeholder: Some("mydb".to_string()),
                },
                CredentialField {
                    name: "username".to_string(),
                    display_name: "Username".to_string(),
                    field_type: "string".to_string(),
                    required: true,
                    description: Some("Database username".to_string()),
                    placeholder: Some("postgres".to_string()),
                },
                CredentialField {
                    name: "password".to_string(),
                    display_name: "Password".to_string(),
                    field_type: "password".to_string(),
                    required: true,
                    description: Some("Database password".to_string()),
                    placeholder: Some("Enter password".to_string()),
                },
            ],
        },
    ];
    
    let get_credential_icon = |cred_type: &str| match cred_type {
        "cloudflare" => "☁️",
        "discord" => "💬",
        "microsoft_graph" => "📧",
        "postgresql" => "🐘",
        "mysql" => "🐬",
        "mongodb" => "🍃",
        "sendgrid" => "📨",
        "slack" => "💼",
        "azure" => "☁️",
        "proxmox" => "🖥️",
        "wazuh" => "🔒",
        _ => "🔑",
    };

    view! {
        <div class="credentials-container">
            <div class="page-header">
                <div class="header-left">
                    <h1>"Credentials"</h1>
                    <p class="subtitle">"Securely manage your service credentials and API keys"</p>
                </div>
                
                <div class="header-actions">
                    <button 
                        class="btn btn-primary"
                        on:click=move |_| set_show_create_modal.set(true)
                    >
                        "➕ New Credential"
                    </button>
                </div>
            </div>
            
            <div class="credentials-toolbar">
                <div class="search-bar">
                    <input
                        type="text"
                        placeholder="Search credentials..."
                        class="search-input"
                        on:input=move |ev| set_search_term.set(event_target_value(&ev))
                    />
                </div>
                
                <div class="toolbar-info">
                    <span class="credential-count">
                        {move || format!("{} credentials", filtered_credentials().len())}
                    </span>
                </div>
            </div>
            
            <div class="credentials-content">
                <Show
                    when=move || !loading.get()
                    fallback=|| view! {
                        <div class="loading-state">
                            <div class="spinner"></div>
                            <p>"Loading credentials..."</p>
                        </div>
                    }
                >
                    <div class="credentials-grid">
                        {move || {
                            let credentials = filtered_credentials();
                            if credentials.is_empty() {
                                view! {
                                    <div class="empty-state">
                                        <div class="empty-icon">"🔑"</div>
                                        <h3>"No credentials found"</h3>
                                        <p>"Create your first credential to get started with integrations"</p>
                                        <button 
                                            class="btn btn-primary"
                                            on:click=move |_| set_show_create_modal.set(true)
                                        >
                                            "Create Credential"
                                        </button>
                                    </div>
                                }.into_view()
                            } else {
                                credentials.into_iter().map(|credential| {
                                    let cred_id = credential.id.clone();
                                    view! {
                                        <div class="credential-card">
                                            <div class="card-header">
                                                <div class="credential-info">
                                                    <div class="credential-icon">
                                                        {get_credential_icon(&credential.credential_type)}
                                                    </div>
                                                    <div class="credential-details">
                                                        <h3 class="credential-name">{&credential.name}</h3>
                                                        <span class="credential-type">
                                                            {credential.credential_type.replace('_', " ").to_uppercase()}
                                                        </span>
                                                    </div>
                                                </div>
                                                
                                                <div class="credential-status">
                                                    <span class="status-indicator encrypted" title="Encrypted">
                                                        "🔒"
                                                    </span>
                                                </div>
                                            </div>
                                            
                                            <div class="card-body">
                                                {move || credential.description.as_ref().map(|desc| {
                                                    view! {
                                                        <p class="credential-description">{desc}</p>
                                                    }
                                                })}
                                                
                                                <div class="credential-meta">
                                                    <div class="meta-item">
                                                        <span class="meta-label">"Created:"</span>
                                                        <span class="meta-value">{&credential.created_at}</span>
                                                    </div>
                                                    
                                                    {move || credential.last_used.as_ref().map(|last_used| {
                                                        view! {
                                                            <div class="meta-item">
                                                                <span class="meta-label">"Last used:"</span>
                                                                <span class="meta-value">{last_used}</span>
                                                            </div>
                                                        }
                                                    })}
                                                </div>
                                            </div>
                                            
                                            <div class="card-footer">
                                                <div class="card-actions">
                                                    <button class="btn btn-sm btn-outline" title="Test Connection">
                                                        "🔍 Test"
                                                    </button>
                                                    <button class="btn btn-sm btn-outline" title="Edit">
                                                        "✏️ Edit"
                                                    </button>
                                                    <button class="btn btn-sm btn-outline btn-danger" title="Delete">
                                                        "🗑️ Delete"
                                                    </button>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Vec<_>>().into_view()
                            }
                        }}
                    </div>
                </Show>
            </div>
            
            // Create Credential Modal
            <Show when=move || show_create_modal.get()>
                <div class="modal-overlay" on:click=move |_| set_show_create_modal.set(false)>
                    <div class="modal-content credential-modal" on:click=|e| e.stop_propagation()>
                        <div class="modal-header">
                            <h3>"Create New Credential"</h3>
                            <button class="close-btn" on:click=move |_| set_show_create_modal.set(false)>
                                "×"
                            </button>
                        </div>
                        
                        <div class="modal-body">
                            <Show
                                when=move || selected_template.get().is_none()
                                fallback=move || {
                                    let template = selected_template.get().unwrap();
                                    view! {
                                        <CredentialForm 
                                            template=template
                                            on_cancel=move || {
                                                set_selected_template.set(None);
                                                set_show_create_modal.set(false);
                                            }
                                            on_save=move |_| {
                                                set_selected_template.set(None);
                                                set_show_create_modal.set(false);
                                                // TODO: Refresh credentials list
                                            }
                                        />
                                    }.into_view()
                                }
                            >
                                <div class="credential-templates">
                                    <h4>"Choose a credential type:"</h4>
                                    <div class="template-grid">
                                        {credential_templates.iter().map(|template| {
                                            let template_clone = template.clone();
                                            view! {
                                                <div 
                                                    class="template-card"
                                                    on:click=move |_| set_selected_template.set(Some(template_clone.clone()))
                                                >
                                                    <div class="template-icon">{&template.icon}</div>
                                                    <div class="template-info">
                                                        <h5>{&template.display_name}</h5>
                                                        <p>{&template.description}</p>
                                                    </div>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                            </Show>
                        </div>
                    </div>
                </div>
            </Show>
        </div>
    }
}

#[component]
fn CredentialForm(
    template: CredentialTemplate,
    on_cancel: impl Fn() + 'static,
    on_save: impl Fn(HashMap<String, String>) + 'static,
) -> impl IntoView {
    let (form_data, set_form_data) = create_signal(HashMap::<String, String>::new());
    let (credential_name, set_credential_name) = create_signal(String::new());
    let (is_saving, set_is_saving) = create_signal(false);
    
    let update_field = move |field_name: String, value: String| {
        set_form_data.update(|data| {
            data.insert(field_name, value);
        });
    };
    
    let save_credential = move |_| {
        set_is_saving.set(true);
        
        spawn_local(async move {
            // Simulate API call
            gloo_timers::future::TimeoutFuture::new(1000).await;
            
            let mut data = form_data.get();
            data.insert("name".to_string(), credential_name.get());
            
            on_save(data);
            set_is_saving.set(false);
        });
    };

    view! {
        <div class="credential-form">
            <div class="form-header">
                <div class="template-info">
                    <div class="template-icon-large">{&template.icon}</div>
                    <div>
                        <h4>{&template.display_name}</h4>
                        <p>{&template.description}</p>
                    </div>
                </div>
            </div>
            
            <div class="form-fields">
                <div class="field-group">
                    <label class="field-label">
                        "Credential Name" <span class="required">*</span>
                    </label>
                    <input
                        type="text"
                        class="field-input"
                        placeholder="Enter a name for this credential"
                        on:input=move |ev| set_credential_name.set(event_target_value(&ev))
                    />
                    <div class="field-help">
                        "Choose a descriptive name to identify this credential"
                    </div>
                </div>
                
                {template.fields.iter().map(|field| {
                    let field_name = field.name.clone();
                    let field_clone = field.clone();
                    
                    view! {
                        <div class="field-group">
                            <label class="field-label">
                                {&field.display_name}
                                {move || if field_clone.required { 
                                    view! { <span class="required">*</span> }.into_view()
                                } else { 
                                    view! {}.into_view()
                                }}
                            </label>
                            
                            {match field.field_type.as_str() {
                                "password" => view! {
                                    <input
                                        type="password"
                                        class="field-input"
                                        placeholder=field.placeholder.as_deref().unwrap_or("")
                                        on:input=move |ev| {
                                            let field_name = field_name.clone();
                                            update_field(field_name, event_target_value(&ev));
                                        }
                                    />
                                }.into_view(),
                                "number" => view! {
                                    <input
                                        type="number"
                                        class="field-input"
                                        placeholder=field.placeholder.as_deref().unwrap_or("")
                                        on:input=move |ev| {
                                            let field_name = field_name.clone();
                                            update_field(field_name, event_target_value(&ev));
                                        }
                                    />
                                }.into_view(),
                                _ => view! {
                                    <input
                                        type="text"
                                        class="field-input"
                                        placeholder=field.placeholder.as_deref().unwrap_or("")
                                        on:input=move |ev| {
                                            let field_name = field_name.clone();
                                            update_field(field_name, event_target_value(&ev));
                                        }
                                    />
                                }.into_view(),
                            }}
                            
                            {field.description.as_ref().map(|desc| view! {
                                <div class="field-help">{desc}</div>
                            })}
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
            
            <div class="form-footer">
                <button 
                    class="btn btn-secondary"
                    on:click=move |_| on_cancel()
                    disabled=move || is_saving.get()
                >
                    "Cancel"
                </button>
                
                <button 
                    class="btn btn-primary"
                    on:click=save_credential
                    disabled=move || is_saving.get() || credential_name.get().is_empty()
                >
                    {move || if is_saving.get() { "Saving..." } else { "Save Credential" }}
                </button>
            </div>
        </div>
    }
}