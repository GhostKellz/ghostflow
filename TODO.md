# GhostFlow – TODO.md

**Status: Alpha** | **Version: 0.1.0** | **Last Updated: 2025-01-08**

An open-source, local-first AI orchestration platform. The DeepSeek to n8n's OpenAI.

---

## 🎯 Current Status

### ✅ Completed (Phase 1: Foundation)
- [x] Rust workspace structure with 6 crates
- [x] Core flow schema v0 (flows, nodes, edges, triggers)
- [x] Execution engine with topological graph traversal
- [x] Node registry and plugin architecture
- [x] PostgreSQL schema and migrations
- [x] Basic nodes: HTTP Request, If/Else, Delay, Template, Webhook
- [x] CLI foundation (`gflow`)
- [x] Basic test coverage
- [x] Working end-to-end flow execution

### 🚧 In Progress
- [ ] Web UI (flow editor)
- [ ] GhostLLM integration
- [ ] Authentication system

---

## 📋 Immediate Priorities (Next 2 Weeks)

### P0: Critical Path
- [ ] **Web UI MVP**
  - [ ] Set up SvelteKit project in `ui/` directory
  - [ ] Create flow canvas with node drag-and-drop
  - [ ] Implement node connection system
  - [ ] Add property panel for node configuration
  - [ ] Connect to backend API endpoints

- [ ] **API Layer**
  - [ ] REST endpoints for flow CRUD operations
  - [ ] WebSocket for real-time execution updates
  - [ ] Flow validation endpoint
  - [ ] Node catalog endpoint

- [ ] **Docker Setup**
  - [ ] Multi-stage Dockerfile for Rust services
  - [ ] Docker Compose with Postgres, MinIO
  - [ ] Development vs production configs
  - [ ] Health checks and restart policies

### P1: Core Features
- [ ] **GhostLLM Integration**
  - [ ] Ollama node (chat completions, embeddings)
  - [ ] LiteLLM router node
  - [ ] Model management endpoints
  - [ ] Streaming response support

- [ ] **Storage & Persistence**
  - [ ] Implement FlowStorage trait with Postgres
  - [ ] ExecutionStorage implementation
  - [ ] Artifact storage with MinIO/S3
  - [ ] Flow import/export (YAML/JSON)

- [ ] **Authentication & Security**
  - [ ] Basic auth with JWT tokens
  - [ ] API key management
  - [ ] Secrets vault abstraction
  - [ ] CORS configuration

### P2: Developer Experience
- [ ] **CLI Enhancements**
  - [ ] `gflow init` - scaffold new project
  - [ ] `gflow run <flow.yaml>` - execute flows locally
  - [ ] `gflow validate` - validate flow definitions
  - [ ] `gflow deploy` - deploy to server

- [ ] **Documentation**
  - [ ] API documentation (OpenAPI/Swagger)
  - [ ] Node development guide
  - [ ] Deployment guide
  - [ ] Example flow library

---

## 🗓️ 30-Day Roadmap

### Week 1-2: UI & API
- Complete Web UI MVP with flow editor
- Full REST API implementation
- WebSocket real-time updates
- Docker development environment

### Week 3: AI Integration
- Ollama integration (local models)
- LiteLLM support (unified interface)
- Function calling/tool use
- RAG pipeline example

### Week 4: Production Ready
- Authentication & RBAC
- Monitoring & logging
- Backup/restore
- Performance optimization
- Beta release

---

## 🚀 Feature Backlog

### Nodes to Implement
**Data Sources**
- [ ] Database Query (Postgres, MySQL, SQLite)
- [ ] Redis operations
- [ ] File operations (read/write/watch)
- [ ] S3/MinIO operations
- [ ] GraphQL client

**AI/ML Nodes**
- [ ] OpenAI compatible
- [ ] Anthropic Claude
- [ ] Vector database (Qdrant, Weaviate)
- [ ] Document loaders (PDF, DOCX, etc.)
- [ ] Text splitters & chunkers
- [ ] Embedding search

**Integration Nodes**
- [ ] Email (SMTP, IMAP)
- [ ] Slack/Discord
- [ ] GitHub/GitLab
- [ ] Kafka/NATS
- [ ] gRPC client

**Control Flow**
- [ ] Loop/Iterator
- [ ] Parallel execution
- [ ] Error handler
- [ ] Rate limiter
- [ ] Circuit breaker

### Platform Features
**Execution**
- [ ] Distributed workers (NATS/Redis queue)
- [ ] Retry policies & dead letter queues
- [ ] Execution replay & debugging
- [ ] Partial execution & checkpoints
- [ ] Resource limits & quotas

**Collaboration**
- [ ] Multi-user support
- [ ] Flow versioning
- [ ] Template marketplace
- [ ] Team workspaces
- [ ] Audit logging

**Operations**
- [ ] Prometheus metrics
- [ ] OpenTelemetry tracing
- [ ] Grafana dashboards
- [ ] Kubernetes Helm chart
- [ ] Horizontal scaling

---

## 🔧 Technical Debt

### Code Quality
- [ ] Remove unused imports warnings
- [ ] Add comprehensive error handling
- [ ] Improve parameter resolution in executor
- [ ] Implement proper expression evaluator
- [ ] Add integration tests

### Performance
- [ ] Connection pooling for HTTP client
- [ ] Optimize graph traversal
- [ ] Implement node result caching
- [ ] Batch database operations
- [ ] Memory usage optimization

### Architecture
- [ ] Separate concern between schema and execution
- [ ] Better abstraction for node inputs/outputs
- [ ] Standardize error types across crates
- [ ] Implement proper event bus
- [ ] Add middleware system

---

## 💡 Future Ideas

### Advanced Features
- **Visual Debugging** - Step through execution with breakpoints
- **AI Assistant** - Natural language to flow generation
- **Smart Retry** - ML-based retry strategies
- **Flow Optimization** - Automatic parallelization detection
- **Version Control** - Git-like flow versioning

### Ecosystem
- **Zeke Integration** - Native Zig CLI bridge
- **WASI Plugins** - Sandboxed custom nodes
- **Flow Marketplace** - Community templates
- **Cloud Offering** - Optional managed service
- **Mobile App** - Monitor flows on the go

---

## 🎪 Definition of Done (v1.0)

### Functionality
- [ ] 50+ production-ready nodes
- [ ] Visual flow editor with live execution
- [ ] Full CRUD operations via UI and CLI
- [ ] Real-time execution monitoring
- [ ] Comprehensive error handling

### Quality
- [ ] 80% test coverage
- [ ] < 500ms flow start latency
- [ ] < 100ms node execution overhead
- [ ] Zero critical security issues
- [ ] Fully documented API

### Deployment
- [ ] One-command Docker deployment
- [ ] Kubernetes production ready
- [ ] Automatic backups
- [ ] Rolling updates
- [ ] High availability mode

---

## 📝 Notes

**Design Principles**
- Local-first, cloud-optional
- Developer experience > feature count
- Performance > convenience
- Security by default
- Extensible everywhere

**Success Metrics**
- Time to first flow: < 5 minutes
- Node development time: < 30 minutes
- Community nodes: 100+ in 6 months
- GitHub stars: 1000+ in year 1

---

**Remember**: We're building the workflow automation platform developers actually want to use. Fast, flexible, and fully under their control.