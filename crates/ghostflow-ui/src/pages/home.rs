use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="home-container">
            <div class="hero-section">
                <div class="hero-content">
                    <h1 class="hero-title">
                        <span class="ghost-icon">"👻"</span>
                        "GhostFlow"
                    </h1>
                    <p class="hero-subtitle">
                        "Local-first AI Orchestration Platform"
                    </p>
                    <p class="hero-description">
                        "Build, deploy, and manage AI-powered workflows with blazing fast Rust performance. 
                        The open-source alternative to n8n, designed for developers and IT professionals."
                    </p>
                    
                    <div class="hero-actions">
                        <a href="/flows/new" class="btn btn-primary">
                            "Create First Flow"
                        </a>
                        <a href="/flows" class="btn btn-secondary">
                            "View All Flows"
                        </a>
                    </div>
                </div>
                
                <div class="hero-stats">
                    <div class="stat-card">
                        <div class="stat-number">"50+"</div>
                        <div class="stat-label">"Integration Nodes"</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-number">"100%"</div>
                        <div class="stat-label">"Rust Powered"</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-number">"Local"</div>
                        <div class="stat-label">"First Design"</div>
                    </div>
                </div>
            </div>

            <div class="features-section">
                <h2>"Key Features"</h2>
                <div class="features-grid">
                    <div class="feature-card">
                        <div class="feature-icon">"🚀"</div>
                        <h3>"High Performance"</h3>
                        <p>"Rust-powered execution engine with minimal resource usage and blazing fast workflow processing."</p>
                    </div>
                    
                    <div class="feature-card">
                        <div class="feature-icon">"🔒"</div>
                        <h3>"Security First"</h3>
                        <p>"Encrypted credential vault, air-gapped deployment, and enterprise-grade security controls."</p>
                    </div>
                    
                    <div class="feature-card">
                        <div class="feature-icon">"🤖"</div>
                        <h3>"AI Native"</h3>
                        <p>"Built-in Ollama, LiteLLM integration with local-first AI processing capabilities."</p>
                    </div>
                    
                    <div class="feature-card">
                        <div class="feature-icon">"🛠️"</div>
                        <h3>"IT Infrastructure"</h3>
                        <p>"Proxmox, Azure, Wazuh, Cloudflare integrations for complete infrastructure automation."</p>
                    </div>
                    
                    <div class="feature-card">
                        <div class="feature-icon">"💼"</div>
                        <h3>"Enterprise Ready"</h3>
                        <p>"Microsoft 365, Google Workspace, Slack, Teams integration for business workflows."</p>
                    </div>
                    
                    <div class="feature-card">
                        <div class="feature-icon">"🔌"</div>
                        <h3>"Extensible"</h3>
                        <p>"Easy node development with full type safety and comprehensive SDK support."</p>
                    </div>
                </div>
            </div>

            <div class="quick-start-section">
                <h2>"Quick Start"</h2>
                <div class="quick-start-grid">
                    <div class="quick-start-card">
                        <div class="step-number">"1"</div>
                        <h3>"Create Credentials"</h3>
                        <p>"Set up secure connections to your services"</p>
                        <a href="/credentials" class="btn btn-outline">"Manage Credentials"</a>
                    </div>
                    
                    <div class="quick-start-card">
                        <div class="step-number">"2"</div>
                        <h3>"Build Workflows"</h3>
                        <p>"Design powerful automation flows visually"</p>
                        <a href="/flows/new" class="btn btn-outline">"Create Flow"</a>
                    </div>
                    
                    <div class="quick-start-card">
                        <div class="step-number">"3"</div>
                        <h3>"Monitor Execution"</h3>
                        <p>"Track performance and debug workflows"</p>
                        <a href="/executions" class="btn btn-outline">"View Executions"</a>
                    </div>
                </div>
            </div>

            <div class="integration-showcase">
                <h2>"Popular Integrations"</h2>
                <div class="integration-logos">
                    <div class="integration-logo" title="Cloudflare">"☁️"</div>
                    <div class="integration-logo" title="Microsoft 365">"📧"</div>
                    <div class="integration-logo" title="Discord">"💬"</div>
                    <div class="integration-logo" title="Slack">"💼"</div>
                    <div class="integration-logo" title="GitLab">"🦊"</div>
                    <div class="integration-logo" title="Google Sheets">"📊"</div>
                    <div class="integration-logo" title="Azure">"☁️"</div>
                    <div class="integration-logo" title="Proxmox">"🖥️"</div>
                    <div class="integration-logo" title="Wazuh">"🔒"</div>
                    <div class="integration-logo" title="Ollama">"🤖"</div>
                </div>
                <div class="integration-count">
                    "And many more integrations available..."
                </div>
            </div>
        </div>
    }
}