# GhostFlow Documentation 📖

Complete documentation for GhostFlow - the local-first AI orchestration platform.

## 📚 Table of Contents

- [Getting Started](#getting-started)
- [Architecture](#architecture)
- [API Reference](#api-reference)
- [Node Development](#node-development)
- [Deployment](#deployment)
- [Examples](#examples)
- [Troubleshooting](#troubleshooting)

---

## 🚀 Getting Started

### Prerequisites

- **Docker & Docker Compose** (recommended)
- **Rust 1.75+** (for native development)
- **PostgreSQL 16+** (if running natively)
- **Ollama** (optional, for AI nodes)

### Quick Start with Docker

```bash
# Clone and start
git clone https://github.com/ghostkellz/ghostflow
cd ghostflow
./scripts/start.sh dev

# Access services
# UI: http://localhost:8080
# API: http://localhost:3000
# Database UI: http://localhost:8081
```

### Native Development Setup

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install sqlx-cli for database migrations
cargo install sqlx-cli

# Set up database
createdb ghostflow
export DATABASE_URL="postgresql://localhost/ghostflow"
sqlx migrate run

# Build and run
cargo build --release
cargo run --bin ghostflow-server
```

---

## 🏗️ Architecture

### System Overview

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Web UI        │    │   API Server    │    │   Database      │
│   (Leptos)      │◄──►│   (Axum)        │◄──►│   (PostgreSQL)  │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                              │                         
                              ▼                         
                    ┌─────────────────┐    ┌─────────────────┐
                    │  Flow Engine    │◄──►│   Node Registry │
                    │  (Execution)    │    │   (Plugins)     │
                    └─────────────────┘    └─────────────────┘
                              │
                              ▼
                    ┌─────────────────┐    ┌─────────────────┐
                    │   Integrations  │    │   Storage       │
                    │ (Ollama/Jarvis) │    │   (MinIO/S3)    │
                    └─────────────────┘    └─────────────────┘
```

### Core Components

**1. Flow Engine (`ghostflow-engine`)**
- Topological graph execution
- Parallel node processing
- Retry and error handling
- Flow scheduling and triggers

**2. Node System (`ghostflow-nodes`)**
- Plugin architecture
- Type-safe interfaces
- Built-in node library
- Custom node support

**3. API Layer (`ghostflow-api`)**
- REST endpoints
- WebSocket real-time updates
- Authentication (planned)
- Rate limiting (planned)

**4. Web UI (`ghostflow-ui`)**
- Visual flow editor
- Real-time execution monitoring
- Node configuration
- Flow management

---

## 🔌 API Reference

### Authentication

Currently, GhostFlow runs without authentication. Production deployment will include JWT-based auth.

### Flow Management

#### Create Flow
```http
POST /api/flows
Content-Type: application/json

{
  "name": "My Flow",
  "description": "Flow description",
  "nodes": {
    "node1": {
      "id": "node1",
      "node_type": "http_request",
      "name": "API Call",
      "parameters": {
        "url": "https://api.example.com/data",
        "method": "GET"
      },
      "position": { "x": 100, "y": 100 }
    }
  },
  "edges": [],
  "triggers": [
    {
      "id": "manual",
      "trigger_type": { "type": "manual" },
      "enabled": true
    }
  ]
}
```

#### Execute Flow
```http
POST /api/flows/:id/execute
Content-Type: application/json

{
  "input": {
    "message": "Hello World"
  }
}
```

#### List Flows
```http
GET /api/flows
```

Response:
```json
{
  "flows": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "name": "My Flow",
      "description": "Flow description",
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z"
    }
  ]
}
```

### Execution Management

#### List Executions
```http
GET /api/executions?flow_id=550e8400-e29b-41d4-a716-446655440000
```

#### Get Execution Details
```http
GET /api/executions/:id
```

### Node Catalog

#### List Available Nodes
```http
GET /api/nodes
```

Response:
```json
{
  "nodes": [
    {
      "id": "http_request",
      "name": "HTTP Request",
      "description": "Make HTTP requests to external APIs",
      "category": "action",
      "parameters": [...]
    }
  ]
}
```

### WebSocket API

Connect to `/ws` for real-time updates:

```javascript
const ws = new WebSocket('ws://localhost:3000/ws');

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('Execution update:', data);
};
```

Message types:
- `execution_started`
- `node_completed`
- `execution_completed`
- `execution_failed`

---

## 🔧 Node Development

### Creating a Custom Node

1. **Define the Node Structure**

```rust
use async_trait::async_trait;
use ghostflow_core::{Node, Result};
use ghostflow_schema::*;

pub struct MyCustomNode;

#[async_trait]
impl Node for MyCustomNode {
    fn definition(&self) -> NodeDefinition {
        NodeDefinition {
            id: "my_custom_node".to_string(),
            name: "My Custom Node".to_string(),
            description: "Does something amazing".to_string(),
            category: NodeCategory::Action,
            version: "1.0.0".to_string(),
            inputs: vec![
                NodePort {
                    name: "input".to_string(),
                    display_name: "Input Data".to_string(),
                    description: Some("Input data to process".to_string()),
                    data_type: DataType::Object,
                    required: true,
                }
            ],
            outputs: vec![
                NodePort {
                    name: "result".to_string(),
                    display_name: "Result".to_string(),
                    description: Some("Processed result".to_string()),
                    data_type: DataType::Object,
                    required: true,
                }
            ],
            parameters: vec![
                NodeParameter {
                    name: "config_value".to_string(),
                    display_name: "Configuration".to_string(),
                    description: Some("Node configuration".to_string()),
                    param_type: ParameterType::String,
                    default_value: Some(serde_json::Value::String("default".to_string())),
                    required: false,
                    options: None,
                    validation: None,
                }
            ],
            icon: Some("star".to_string()),
            color: Some("#10b981".to_string()),
        }
    }

    async fn validate(&self, context: &ExecutionContext) -> Result<()> {
        // Validate inputs and parameters
        if context.input.get("input").is_none() {
            return Err(GhostFlowError::ValidationError {
                message: "Input is required".to_string(),
            });
        }
        Ok(())
    }

    async fn execute(&self, context: ExecutionContext) -> Result<serde_json::Value> {
        let input_data = context.input.get("input").unwrap();
        let config = context.input
            .get("config_value")
            .and_then(|v| v.as_str())
            .unwrap_or("default");

        // Your node logic here
        let result = serde_json::json!({
            "processed": true,
            "input": input_data,
            "config": config,
            "timestamp": chrono::Utc::now()
        });

        Ok(result)
    }

    fn supports_retry(&self) -> bool {
        true
    }

    fn is_deterministic(&self) -> bool {
        true
    }
}
```

2. **Register the Node**

```rust
// In your main application
let mut registry = BasicNodeRegistry::new();
registry.register_node("my_custom_node".to_string(), Arc::new(MyCustomNode::new()))?;
```

3. **Test Your Node**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use ghostflow_schema::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_my_custom_node() {
        let node = MyCustomNode;
        let context = ExecutionContext {
            execution_id: Uuid::new_v4(),
            flow_id: Uuid::new_v4(),
            node_id: "test".to_string(),
            input: serde_json::json!({
                "input": {"test": "data"},
                "config_value": "test_config"
            }),
            variables: std::collections::HashMap::new(),
            secrets: std::collections::HashMap::new(),
            artifacts: std::collections::HashMap::new(),
        };

        let result = node.execute(context).await.unwrap();
        assert!(result["processed"].as_bool().unwrap());
    }
}
```

### Node Categories

- **Trigger** - Start flow execution (webhooks, schedules)
- **Action** - Perform operations (HTTP, database, file operations)
- **Transform** - Modify data (templates, filters, mappers)
- **ControlFlow** - Flow logic (conditions, loops, delays)
- **Integration** - External services (APIs, databases, messaging)
- **Ai** - AI/ML operations (LLMs, embeddings, predictions)
- **Data** - Data operations (storage, queries, transformations)
- **Utility** - Helper functions (logging, debugging, utilities)

---

## 🚀 Deployment

### Docker Compose (Recommended)

```yaml
# docker-compose.yml
version: '3.8'

services:
  ghostflow:
    build: .
    ports:
      - "3000:3000"
      - "8080:8080"
    environment:
      DATABASE_URL: postgresql://ghostflow:ghostflow@postgres/ghostflow
      RUST_LOG: info
    depends_on:
      - postgres

  postgres:
    image: postgres:16-alpine
    environment:
      POSTGRES_USER: ghostflow
      POSTGRES_PASSWORD: ghostflow
      POSTGRES_DB: ghostflow
    volumes:
      - postgres_data:/var/lib/postgresql/data

volumes:
  postgres_data:
```

### Kubernetes Deployment

```yaml
# k8s/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: ghostflow
spec:
  replicas: 3
  selector:
    matchLabels:
      app: ghostflow
  template:
    metadata:
      labels:
        app: ghostflow
    spec:
      containers:
      - name: ghostflow
        image: ghostflow:latest
        ports:
        - containerPort: 3000
        - containerPort: 8080
        env:
        - name: DATABASE_URL
          value: "postgresql://ghostflow:password@postgres:5432/ghostflow"
        - name: RUST_LOG
          value: "info"
---
apiVersion: v1
kind: Service
metadata:
  name: ghostflow-service
spec:
  selector:
    app: ghostflow
  ports:
  - name: api
    port: 3000
    targetPort: 3000
  - name: ui
    port: 8080
    targetPort: 8080
  type: LoadBalancer
```

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | `postgresql://localhost/ghostflow` |
| `RUST_LOG` | Log level | `info` |
| `OLLAMA_HOST` | Ollama server URL | `http://localhost:11434` |
| `MINIO_ENDPOINT` | MinIO/S3 endpoint | `http://localhost:9000` |
| `MINIO_ACCESS_KEY` | MinIO access key | `ghostflow` |
| `MINIO_SECRET_KEY` | MinIO secret key | `ghostflow123` |
| `JWT_SECRET` | JWT signing secret | `your-secret-key` |

---

## 💡 Examples

### Simple HTTP to Template Flow

```yaml
# simple-http-flow.yaml
name: "HTTP to Template"
description: "Fetch data and format it"
version: "1.0.0"

nodes:
  fetch_data:
    node_type: "http_request"
    name: "Fetch API Data"
    parameters:
      url: "https://jsonplaceholder.typicode.com/users/1"
      method: "GET"
    position: { x: 100, y: 100 }

  format_output:
    node_type: "template"
    name: "Format Output"
    parameters:
      template: "Hello {{name}}, your email is {{email}}"
    position: { x: 300, y: 100 }

edges:
  - id: "fetch_to_format"
    source_node: "fetch_data"
    target_node: "format_output"

triggers:
  - id: "manual"
    trigger_type: { type: "manual" }
    enabled: true
```

### AI Text Generation with Ollama

```yaml
name: "AI Content Generator"
description: "Generate content using local AI"

nodes:
  generate_content:
    node_type: "ollama_generate"
    name: "Generate Text"
    parameters:
      model: "llama2"
      system: "You are a helpful content writer"
      temperature: 0.7
      max_tokens: 500
    position: { x: 100, y: 100 }

  format_response:
    node_type: "template"
    name: "Format Response"
    parameters:
      template: |
        Generated Content:
        {{response}}
        
        Model: {{model}}
        Generated at: {{timestamp}}
    position: { x: 300, y: 100 }

edges:
  - source_node: "generate_content"
    target_node: "format_response"
```

### Jarvis Integration Flow

```yaml
name: "Jarvis Automation"
description: "Execute Jarvis commands"

nodes:
  run_jarvis:
    node_type: "jarvis_command"
    name: "Run Jarvis Task"
    parameters:
      command: "jarvis"
      args: "task,analyze,--format,json"
      timeout_seconds: 60
    position: { x: 100, y: 100 }

  process_results:
    node_type: "template"
    name: "Process Results"
    parameters:
      template: |
        Task completed successfully: {{success}}
        Exit code: {{exit_code}}
        Output: {{stdout}}
    position: { x: 300, y: 100 }
```

---

## 🔍 Troubleshooting

### Common Issues

**1. Database Connection Failed**
```
Error: Failed to connect to database
```
Solution:
- Ensure PostgreSQL is running
- Check DATABASE_URL environment variable
- Verify database exists: `createdb ghostflow`

**2. Compilation Errors**
```
Error: failed to compile `ghostflow-engine`
```
Solution:
- Update Rust: `rustup update`
- Clean build: `cargo clean && cargo build`
- Check Rust version: `rustc --version` (need 1.75+)

**3. Ollama Node Errors**
```
Error: Network error: Connection refused
```
Solution:
- Install Ollama: `curl -fsSL https://ollama.ai/install.sh | sh`
- Start Ollama: `ollama serve`
- Pull model: `ollama pull llama2`

**4. Docker Build Issues**
```
Error: failed to solve: executor failed running
```
Solution:
- Increase Docker memory limit
- Clean Docker: `docker system prune -a`
- Check Dockerfile syntax

### Performance Tuning

**Database Optimization**
```sql
-- Add indexes for better performance
CREATE INDEX idx_flows_created_at ON flows(created_at);
CREATE INDEX idx_executions_flow_id ON flow_executions(flow_id);
CREATE INDEX idx_executions_status ON flow_executions(status);
```

**Memory Usage**
```bash
# Monitor memory usage
docker stats ghostflow

# Adjust Rust memory allocator
export MALLOC_ARENA_MAX=2
```

### Debug Mode

Enable detailed logging:
```bash
export RUST_LOG=debug
export RUST_BACKTRACE=1
cargo run --bin ghostflow-server
```

### Health Checks

```bash
# API health
curl http://localhost:3000/health

# Database connection
psql $DATABASE_URL -c "SELECT 1;"

# Ollama status
curl http://localhost:11434/api/tags
```

---

## 🤝 Contributing

See the main repository for contribution guidelines. Key areas:

- **Node Development** - Create new node types
- **UI/UX** - Improve the web interface
- **Documentation** - Help improve these docs
- **Testing** - Add test coverage
- **Performance** - Optimize execution engine

---

## 📞 Support

- **GitHub Issues** - Bug reports and feature requests
- **Discussions** - Community help and questions
- **Discord** - Real-time chat and support
- **Email** - security@ghostflow.dev for security issues

---

*This documentation is actively maintained. Last updated: 2025-01-08*