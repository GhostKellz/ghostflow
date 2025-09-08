use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod components;
pub mod pages;

use crate::pages::{FlowEditor, FlowList, Home};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/ghostflow-ui.css"/>
        <Title text="GhostFlow - AI Orchestration Platform"/>
        
        <Router>
            <nav class="navbar">
                <div class="container">
                    <a href="/" class="brand">
                        <span class="logo">👻</span>
                        <span class="name">"GhostFlow"</span>
                    </a>
                    <div class="nav-links">
                        <A href="/flows">"Flows"</A>
                        <A href="/nodes">"Nodes"</A>
                        <A href="/executions">"Executions"</A>
                        <A href="/settings">"Settings"</A>
                    </div>
                </div>
            </nav>
            
            <main class="main-content">
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/flows" view=FlowList/>
                    <Route path="/flows/:id" view=FlowEditor/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="not-found">
            <h1>"404"</h1>
            <p>"Page not found"</p>
            <A href="/">"Go home"</A>
        </div>
    }
}