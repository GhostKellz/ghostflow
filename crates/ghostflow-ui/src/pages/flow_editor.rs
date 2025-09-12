use leptos::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowNode {
    pub id: String,
    pub node_type: String,
    pub display_name: String,
    pub position: Position,
    pub parameters: HashMap<String, serde_json::Value>,
    pub selected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowEdge {
    pub id: String,
    pub source_node: String,
    pub source_output: String,
    pub target_node: String,
    pub target_input: String,
}

#[component]
pub fn FlowEditor() -> impl IntoView {
    let params = leptos_router::use_params_map();
    let flow_id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());
    
    let (nodes, set_nodes) = create_signal(Vec::<FlowNode>::new());
    let (edges, set_edges) = create_signal(Vec::<FlowEdge>::new());
    let (selected_node, set_selected_node) = create_signal(None::<String>);
    let (is_executing, set_is_executing) = create_signal(false);
    let (execution_logs, set_execution_logs) = create_signal(Vec::<String>::new());
    let (show_node_palette, set_show_node_palette) = create_signal(false);
    
    // Available node types
    let node_types = vec![
        ("http_request", "HTTP Request", "🌐"),
        ("webhook", "Webhook", "📨"),
        ("if_else", "If/Else", "🔀"),
        ("delay", "Delay", "⏰"),
        ("template", "Template", "📝"),
        ("cloudflare_dns", "Cloudflare DNS", "☁️"),
        ("discord_webhook", "Discord Webhook", "💬"),
        ("slack_message", "Slack Message", "💼"),
        ("microsoft_graph_email", "Microsoft Email", "📧"),
        ("google_sheets", "Google Sheets", "📊"),
        ("gitlab_project", "GitLab Project", "🦊"),
        ("azure_vm", "Azure VM", "☁️"),
        ("proxmox_vm", "Proxmox VM", "🖥️"),
        ("wazuh_api", "Wazuh SIEM", "🔒"),
        ("ollama_generate", "Ollama Generate", "🤖"),
    ];
    
    let add_node = move |node_type: &str, display_name: &str| {
        let new_node = FlowNode {
            id: format!("node_{}", uuid::Uuid::new_v4().to_string()[..8].to_string()),
            node_type: node_type.to_string(),
            display_name: display_name.to_string(),
            position: Position { x: 100.0, y: 100.0 },
            parameters: HashMap::new(),
            selected: false,
        };
        
        set_nodes.update(|nodes| nodes.push(new_node));
        set_show_node_palette.set(false);
    };
    
    let execute_flow = move |_| {
        set_is_executing.set(true);
        set_execution_logs.set(vec!["Starting flow execution...".to_string()]);
        
        // Simulate execution
        spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(2000).await;
            set_execution_logs.update(|logs| logs.push("Flow executed successfully!".to_string()));
            set_is_executing.set(false);
        });
    };
    
    let save_flow = move |_| {
        // TODO: Implement API call to save flow
        log::info!("Saving flow...");
    };

    view! {
        <div class="flow-editor">
            <div class="editor-toolbar">
                <div class="toolbar-left">
                    <h2>"Flow Editor"</h2>
                    <span class="flow-id">{move || format!("ID: {}", flow_id())}</span>
                </div>
                
                <div class="toolbar-right">
                    <button 
                        class="btn btn-secondary"
                        on:click=move |_| set_show_node_palette.set(!show_node_palette.get())
                    >
                        "➕ Add Node"
                    </button>
                    
                    <button 
                        class="btn btn-primary"
                        on:click=save_flow
                    >
                        "💾 Save"
                    </button>
                    
                    <button 
                        class="btn btn-success"
                        on:click=execute_flow
                        disabled=move || is_executing.get()
                    >
                        {move || if is_executing.get() { "⏳ Executing..." } else { "▶️ Execute" }}
                    </button>
                </div>
            </div>
            
            <div class="editor-content">
                <div class="editor-main">
                    <div class="flow-canvas">
                        <svg class="canvas-svg" width="100%" height="100%">
                            // Render edges
                            {move || {
                                edges.get().into_iter().map(|edge| {
                                    view! {
                                        <line
                                            class="edge"
                                            x1="100" y1="100"
                                            x2="300" y2="200"
                                            stroke="#666"
                                            stroke-width="2"
                                            marker-end="url(#arrowhead)"
                                        />
                                    }
                                }).collect::<Vec<_>>()
                            }}
                            
                            // Arrow marker definition
                            <defs>
                                <marker id="arrowhead" markerWidth="10" markerHeight="7" 
                                        refX="9" refY="3.5" orient="auto">
                                    <polygon points="0 0, 10 3.5, 0 7" fill="#666" />
                                </marker>
                            </defs>
                        </svg>
                        
                        // Render nodes
                        {move || {
                            nodes.get().into_iter().map(|node| {
                                let node_id = node.id.clone();
                                let is_selected = move || selected_node.get() == Some(node_id.clone());
                                
                                view! {
                                    <div
                                        class="flow-node"
                                        class:selected=is_selected
                                        style=format!("transform: translate({}px, {}px)", node.position.x, node.position.y)
                                        on:click=move |_| set_selected_node.set(Some(node_id.clone()))
                                    >
                                        <div class="node-header">
                                            <span class="node-icon">
                                                {node_types.iter().find(|(t, _, _)| *t == node.node_type)
                                                    .map(|(_, _, icon)| *icon).unwrap_or("⚙️")}
                                            </span>
                                            <span class="node-title">{&node.display_name}</span>
                                        </div>
                                        
                                        <div class="node-ports">
                                            <div class="input-ports">
                                                <div class="port input-port" title="Input"></div>
                                            </div>
                                            <div class="output-ports">
                                                <div class="port output-port" title="Output"></div>
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Vec<_>>()
                        }}
                        
                        // Canvas background
                        <div class="canvas-grid"></div>
                    </div>
                </div>
                
                <div class="editor-sidebar">
                    <div class="sidebar-section">
                        <h3>"Properties"</h3>
                        {move || {
                            if let Some(node_id) = selected_node.get() {
                                if let Some(node) = nodes.get().iter().find(|n| n.id == node_id) {
                                    view! {
                                        <div class="node-properties">
                                            <div class="property-group">
                                                <label>"Node Type"</label>
                                                <input type="text" value=&node.node_type readonly />
                                            </div>
                                            
                                            <div class="property-group">
                                                <label>"Display Name"</label>
                                                <input type="text" value=&node.display_name />
                                            </div>
                                            
                                            <div class="property-group">
                                                <label>"Parameters"</label>
                                                <div class="parameters-editor">
                                                    "Parameters will be dynamically loaded based on node type"
                                                </div>
                                            </div>
                                        </div>
                                    }.into_view()
                                } else {
                                    view! { <div>"Node not found"</div> }.into_view()
                                }
                            } else {
                                view! { <div class="no-selection">"Select a node to edit properties"</div> }.into_view()
                            }
                        }}
                    </div>
                    
                    <div class="sidebar-section">
                        <h3>"Execution Log"</h3>
                        <div class="execution-log">
                            {move || {
                                execution_logs.get().into_iter().map(|log| {
                                    view! {
                                        <div class="log-entry">{log}</div>
                                    }
                                }).collect::<Vec<_>>()
                            }}
                        </div>
                    </div>
                </div>
            </div>
            
            // Node Palette Modal
            <Show when=move || show_node_palette.get()>
                <div class="modal-overlay" on:click=move |_| set_show_node_palette.set(false)>
                    <div class="modal-content node-palette" on:click=|e| e.stop_propagation()>
                        <div class="modal-header">
                            <h3>"Add Node"</h3>
                            <button class="close-btn" on:click=move |_| set_show_node_palette.set(false)>
                                "×"
                            </button>
                        </div>
                        
                        <div class="modal-body">
                            <div class="node-categories">
                                <div class="category">
                                    <h4>"Basic Nodes"</h4>
                                    <div class="node-grid">
                                        {node_types.iter().take(5).map(|(node_type, display_name, icon)| {
                                            let node_type = node_type.to_string();
                                            let display_name = display_name.to_string();
                                            view! {
                                                <div 
                                                    class="node-palette-item"
                                                    on:click=move |_| add_node(&node_type, &display_name)
                                                >
                                                    <div class="palette-icon">{icon}</div>
                                                    <div class="palette-name">{display_name}</div>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                                
                                <div class="category">
                                    <h4>"Integrations"</h4>
                                    <div class="node-grid">
                                        {node_types.iter().skip(5).map(|(node_type, display_name, icon)| {
                                            let node_type = node_type.to_string();
                                            let display_name = display_name.to_string();
                                            view! {
                                                <div 
                                                    class="node-palette-item"
                                                    on:click=move |_| add_node(&node_type, &display_name)
                                                >
                                                    <div class="palette-icon">{icon}</div>
                                                    <div class="palette-name">{display_name}</div>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </Show>
        </div>
    }
}