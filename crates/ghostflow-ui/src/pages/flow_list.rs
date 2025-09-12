use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowSummary {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: FlowStatus,
    pub last_execution: Option<String>,
    pub created_at: String,
    pub node_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlowStatus {
    Active,
    Paused,
    Draft,
    Error,
}

#[component]
pub fn FlowList() -> impl IntoView {
    let (flows, set_flows) = create_signal(Vec::<FlowSummary>::new());
    let (loading, set_loading) = create_signal(true);
    let (search_term, set_search_term) = create_signal(String::new());
    let (selected_status, set_selected_status) = create_signal("all".to_string());
    
    // Load flows on component mount
    create_effect(move |_| {
        spawn_local(async move {
            // Simulate API call
            gloo_timers::future::TimeoutFuture::new(1000).await;
            
            let sample_flows = vec![
                FlowSummary {
                    id: "flow_001".to_string(),
                    name: "Discord Alert System".to_string(),
                    description: "Send security alerts to Discord channels with severity filtering".to_string(),
                    status: FlowStatus::Active,
                    last_execution: Some("2 minutes ago".to_string()),
                    created_at: "2024-01-15".to_string(),
                    node_count: 5,
                },
                FlowSummary {
                    id: "flow_002".to_string(),
                    name: "Microsoft Teams Notifications".to_string(),
                    description: "Automated notifications for project updates via Microsoft Teams".to_string(),
                    status: FlowStatus::Active,
                    last_execution: Some("1 hour ago".to_string()),
                    created_at: "2024-01-14".to_string(),
                    node_count: 3,
                },
                FlowSummary {
                    id: "flow_003".to_string(),
                    name: "Proxmox VM Monitoring".to_string(),
                    description: "Monitor VM status and send alerts when resources are low".to_string(),
                    status: FlowStatus::Paused,
                    last_execution: Some("1 day ago".to_string()),
                    created_at: "2024-01-10".to_string(),
                    node_count: 8,
                },
                FlowSummary {
                    id: "flow_004".to_string(),
                    name: "Wazuh Security Correlation".to_string(),
                    description: "Correlate security events and trigger incident response workflows".to_string(),
                    status: FlowStatus::Active,
                    last_execution: Some("5 minutes ago".to_string()),
                    created_at: "2024-01-12".to_string(),
                    node_count: 12,
                },
                FlowSummary {
                    id: "flow_005".to_string(),
                    name: "Google Sheets Data Sync".to_string(),
                    description: "Sync data between multiple Google Sheets and generate reports".to_string(),
                    status: FlowStatus::Draft,
                    last_execution: None,
                    created_at: "2024-01-16".to_string(),
                    node_count: 4,
                },
            ];
            
            set_flows.set(sample_flows);
            set_loading.set(false);
        });
    });
    
    let filtered_flows = move || {
        let flows = flows.get();
        let search = search_term.get().to_lowercase();
        let status_filter = selected_status.get();
        
        flows.into_iter()
            .filter(|flow| {
                let matches_search = search.is_empty() || 
                    flow.name.to_lowercase().contains(&search) ||
                    flow.description.to_lowercase().contains(&search);
                    
                let matches_status = status_filter == "all" || 
                    matches!((status_filter.as_str(), &flow.status), 
                        ("active", FlowStatus::Active) |
                        ("paused", FlowStatus::Paused) |
                        ("draft", FlowStatus::Draft) |
                        ("error", FlowStatus::Error)
                    );
                    
                matches_search && matches_status
            })
            .collect::<Vec<_>>()
    };
    
    let get_status_class = |status: &FlowStatus| match status {
        FlowStatus::Active => "status-active",
        FlowStatus::Paused => "status-paused",
        FlowStatus::Draft => "status-draft",
        FlowStatus::Error => "status-error",
    };
    
    let get_status_text = |status: &FlowStatus| match status {
        FlowStatus::Active => "Active",
        FlowStatus::Paused => "Paused",
        FlowStatus::Draft => "Draft",
        FlowStatus::Error => "Error",
    };

    view! {
        <div class="flow-list-container">
            <div class="page-header">
                <div class="header-left">
                    <h1>"Workflows"</h1>
                    <p class="subtitle">"Manage and monitor your automation workflows"</p>
                </div>
                
                <div class="header-actions">
                    <a href="/flows/new" class="btn btn-primary">
                        "➕ New Flow"
                    </a>
                </div>
            </div>
            
            <div class="filters-section">
                <div class="search-bar">
                    <input
                        type="text"
                        placeholder="Search flows..."
                        class="search-input"
                        on:input=move |ev| set_search_term.set(event_target_value(&ev))
                    />
                </div>
                
                <div class="filter-controls">
                    <select 
                        class="status-filter"
                        on:change=move |ev| set_selected_status.set(event_target_value(&ev))
                    >
                        <option value="all">"All Status"</option>
                        <option value="active">"Active"</option>
                        <option value="paused">"Paused"</option>
                        <option value="draft">"Draft"</option>
                        <option value="error">"Error"</option>
                    </select>
                </div>
            </div>
            
            <div class="flows-content">
                <Show
                    when=move || !loading.get()
                    fallback=|| view! {
                        <div class="loading-state">
                            <div class="spinner"></div>
                            <p>"Loading workflows..."</p>
                        </div>
                    }
                >
                    <div class="flows-grid">
                        {move || {
                            let flows = filtered_flows();
                            if flows.is_empty() {
                                view! {
                                    <div class="empty-state">
                                        <div class="empty-icon">"📋"</div>
                                        <h3>"No flows found"</h3>
                                        <p>"Create your first workflow to get started"</p>
                                        <a href="/flows/new" class="btn btn-primary">
                                            "Create Flow"
                                        </a>
                                    </div>
                                }.into_view()
                            } else {
                                flows.into_iter().map(|flow| {
                                    let flow_id = flow.id.clone();
                                    view! {
                                        <div class="flow-card">
                                            <div class="card-header">
                                                <div class="flow-info">
                                                    <h3 class="flow-name">
                                                        <a href=format!("/flows/{}", flow_id)>{&flow.name}</a>
                                                    </h3>
                                                    <span class={format!("status-badge {}", get_status_class(&flow.status))}>
                                                        {get_status_text(&flow.status)}
                                                    </span>
                                                </div>
                                                
                                                <div class="flow-actions">
                                                    <button class="btn-icon" title="Edit">
                                                        "✏️"
                                                    </button>
                                                    <button class="btn-icon" title="Execute">
                                                        "▶️"
                                                    </button>
                                                    <button class="btn-icon" title="More">
                                                        "⋮"
                                                    </button>
                                                </div>
                                            </div>
                                            
                                            <div class="card-body">
                                                <p class="flow-description">{&flow.description}</p>
                                                
                                                <div class="flow-stats">
                                                    <div class="stat">
                                                        <span class="stat-icon">"🔗"</span>
                                                        <span class="stat-value">{flow.node_count}</span>
                                                        <span class="stat-label">"nodes"</span>
                                                    </div>
                                                    
                                                    <div class="stat">
                                                        <span class="stat-icon">"📅"</span>
                                                        <span class="stat-value">{&flow.created_at}</span>
                                                        <span class="stat-label">"created"</span>
                                                    </div>
                                                </div>
                                            </div>
                                            
                                            <div class="card-footer">
                                                <div class="last-execution">
                                                    {move || match &flow.last_execution {
                                                        Some(time) => view! {
                                                            <span>"Last run: " {time}</span>
                                                        }.into_view(),
                                                        None => view! {
                                                            <span class="never-run">"Never executed"</span>
                                                        }.into_view(),
                                                    }}
                                                </div>
                                                
                                                <div class="card-actions">
                                                    <a href=format!("/flows/{}", flow_id) class="btn btn-sm btn-outline">
                                                        "View Details"
                                                    </a>
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
            
            // Flow statistics summary
            <div class="stats-summary">
                <div class="summary-card">
                    <div class="summary-number">{move || flows.get().len()}</div>
                    <div class="summary-label">"Total Flows"</div>
                </div>
                
                <div class="summary-card">
                    <div class="summary-number">
                        {move || flows.get().iter().filter(|f| matches!(f.status, FlowStatus::Active)).count()}
                    </div>
                    <div class="summary-label">"Active"</div>
                </div>
                
                <div class="summary-card">
                    <div class="summary-number">
                        {move || flows.get().iter().map(|f| f.node_count).sum::<u32>()}
                    </div>
                    <div class="summary-label">"Total Nodes"</div>
                </div>
                
                <div class="summary-card">
                    <div class="summary-number">
                        {move || flows.get().iter().filter(|f| f.last_execution.is_some()).count()}
                    </div>
                    <div class="summary-label">"Recently Executed"</div>
                </div>
            </div>
        </div>
    }
}