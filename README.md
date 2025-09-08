# GhostFlow 👻

<div align="center">
  
**Open-Source AI Orchestration Platform**  
*The DeepSeek to n8n's OpenAI - Local-first, Developer-focused, Rust-powered*

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)
[![Status](https://img.shields.io/badge/status-alpha-red)](TODO.md)
[![Docker](https://img.shields.io/badge/docker-ready-blue?logo=docker)](docker-compose.yml)

</div>

---

## 🎯 What is GhostFlow?

GhostFlow is a **local-first AI orchestration platform** that lets you build, deploy, and manage AI-powered workflows. Think n8n meets LangChain, but faster, type-safe, and fully under your control.

### Key Features

- 🚀 **Rust Performance** - Blazing fast execution with minimal resource usage
- 🏠 **Local-First** - Run entirely on your hardware, no cloud required
- 🤖 **AI Native** - Built-in Ollama, LiteLLM, and Jarvis integration
- 🔌 **Extensible** - Easy node development with full type safety
- 🐳 **Docker Ready** - One-command deployment with docker-compose
- 🎨 **Visual Editor** - Leptos-powered web UI (100% Rust)
- 🔒 **Secure** - Air-gapped friendly, zero-trust architecture

## 🚀 Quick Start

### Using Docker (Recommended)

```bash
# Clone the repository
git clone https://github.com/ghostkellz/ghostflow
cd ghostflow

# Start GhostFlow in development mode
./scripts/start.sh dev

# Access the platform
# UI: http://localhost:8080
# API: http://localhost:3000
```

### Manual Setup

```bash
# Prerequisites: Rust 1.75+, PostgreSQL, Ollama (optional)

# Build the project
cargo build --release

# Run migrations
sqlx migrate run --database-url postgresql://ghostflow:ghostflow@localhost/ghostflow

# Start the server
cargo run --bin ghostflow-server

# In another terminal, start the UI
cargo run --bin ghostflow-ui
```

## 📦 Architecture

```
ghostflow/
├── crates/
│   ├── ghostflow-core/       # Core traits and types
│   ├── ghostflow-schema/     # Flow schemas and models
│   ├── ghostflow-engine/     # Execution engine
│   ├── ghostflow-nodes/      # Built-in nodes
│   ├── ghostflow-api/        # REST/WebSocket API
│   ├── ghostflow-ui/         # Leptos web UI
│   ├── ghostflow-jarvis/     # Jarvis CLI integration
│   ├── ghostflow-server/     # Main server binary
│   └── ghostflow-cli/        # gflow CLI tool
├── migrations/               # PostgreSQL migrations
├── docker-compose.yml        # Docker orchestration
└── Dockerfile               # Multi-stage build
```

## 🎯 Available Nodes

### Core Nodes
- **HTTP Request** - Make API calls with full request control
- **Webhook** - Receive incoming HTTP requests
- **Template** - Process templates with variable substitution
- **If/Else** - Conditional flow control
- **Delay** - Time-based flow control

### AI/LLM Nodes
- **Ollama Generate** - Local LLM text generation
- **Ollama Embeddings** - Generate vector embeddings
- **Jarvis Command** - Execute Rust CLI automation

### Coming Soon
- Database Query (PostgreSQL, MySQL, SQLite)
- Vector Database (Qdrant, Weaviate)
- Email (SMTP/IMAP)
- Slack/Discord
- OpenAI/Anthropic

## 🛠️ Development

### Creating a Custom Node

```rust
use ghostflow_core::{Node, Result};
use async_trait::async_trait;

pub struct MyCustomNode;

#[async_trait]
impl Node for MyCustomNode {
    fn definition(&self) -> NodeDefinition {
        // Define inputs, outputs, and parameters
    }
    
    async fn execute(&self, context: ExecutionContext) -> Result<Value> {
        // Your node logic here
    }
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific crate tests
cargo test -p ghostflow-engine

# Run with logging
RUST_LOG=debug cargo test
```

## 🐳 Docker Services

The docker-compose setup includes:

- **PostgreSQL** - Flow and execution storage
- **MinIO** - S3-compatible artifact storage
- **Ollama** - Local LLM runtime
- **GhostFlow** - Main application
- **Adminer** - Database UI (dev only)

## 📚 API Documentation

### REST Endpoints

```
GET    /api/flows              # List all flows
POST   /api/flows              # Create new flow
GET    /api/flows/:id          # Get flow details
PUT    /api/flows/:id          # Update flow
DELETE /api/flows/:id          # Delete flow
POST   /api/flows/:id/execute  # Execute flow

GET    /api/executions         # List executions
GET    /api/executions/:id     # Get execution details

GET    /api/nodes              # List available nodes
```

### WebSocket

Connect to `/ws` for real-time execution updates.

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Areas We Need Help

- Additional node implementations
- UI/UX improvements
- Documentation
- Testing
- Performance optimization

## 📋 Roadmap

See [TODO.md](TODO.md) for detailed roadmap and progress.

### Current Priorities
- ✅ Core execution engine
- ✅ Basic nodes
- ✅ Docker setup
- 🚧 Web UI improvements
- 🚧 More AI integrations
- 📅 Authentication & RBAC
- 📅 Distributed execution

## 🙏 Acknowledgments

Built with amazing open-source projects:
- [Rust](https://rust-lang.org) - Systems programming language
- [Leptos](https://leptos.dev) - Full-stack Rust web framework
- [Axum](https://github.com/tokio-rs/axum) - Web application framework
- [Ollama](https://ollama.ai) - Local LLM runtime
- [PostgreSQL](https://postgresql.org) - Database

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

<div align="center">
  
**Built with ❤️ by the GhostFlow Community**  
*Fast. Flexible. Fully yours.*

[Documentation](https://docs.ghostflow.dev) | [Discord](https://discord.gg/ghostflow) | [Twitter](https://twitter.com/ghostflow)

</div>