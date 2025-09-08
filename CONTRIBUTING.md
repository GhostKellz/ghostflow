# Contributing to GhostFlow 🤝

Thank you for your interest in contributing to GhostFlow! This guide will help you get started with contributing to our open-source AI orchestration platform.

## 🌟 Ways to Contribute

- 🐛 **Report bugs** - Help us find and fix issues
- 💡 **Suggest features** - Share your ideas for improvements
- 🔧 **Submit code** - Fix bugs, add features, improve performance
- 📚 **Improve documentation** - Help others understand GhostFlow
- 🎨 **Design improvements** - UI/UX enhancements
- 🧪 **Write tests** - Increase code coverage and reliability
- 🔌 **Create nodes** - Extend GhostFlow's capabilities

## 🚀 Getting Started

### 1. Fork and Clone

```bash
# Fork the repository on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/ghostflow.git
cd ghostflow

# Add the original repository as upstream
git remote add upstream https://github.com/ghostkellz/ghostflow.git
```

### 2. Set Up Development Environment

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install development tools
cargo install sqlx-cli --no-default-features --features postgres
cargo install cargo-watch

# Start development environment
./scripts/start.sh dev

# Verify everything works
cargo test
```

### 3. Create a Branch

```bash
# Create a new branch for your work
git checkout -b feature/amazing-new-feature

# Or for bug fixes
git checkout -b fix/issue-description
```

## 📝 Development Guidelines

### Code Style

We use standard Rust conventions and tools:

```bash
# Format your code
cargo fmt

# Run clippy for lints
cargo clippy -- -D warnings

# Run tests
cargo test

# Check documentation
cargo doc --open
```

### Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): description

feat(nodes): add OpenAI integration node
fix(engine): resolve deadlock in parallel execution
docs(api): update WebSocket documentation
test(core): add unit tests for node registry
refactor(ui): simplify flow editor components
perf(engine): optimize graph traversal algorithm
```

**Types:**
- `feat`: New features
- `fix`: Bug fixes
- `docs`: Documentation changes
- `test`: Adding or updating tests
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `style`: Code style changes (formatting, etc.)
- `ci`: CI/CD changes
- `chore`: Maintenance tasks

### Pull Request Process

1. **Create Quality Code**
   - Write tests for new functionality
   - Ensure all tests pass
   - Update documentation as needed
   - Follow the existing code style

2. **Write a Good PR Description**
   ```markdown
   ## Description
   Brief description of changes

   ## Changes Made
   - Added X feature
   - Fixed Y bug
   - Updated Z documentation

   ## Testing
   - [ ] Unit tests added/updated
   - [ ] Integration tests pass
   - [ ] Manual testing completed

   ## Breaking Changes
   None / List any breaking changes

   ## Screenshots (if applicable)
   Add screenshots for UI changes
   ```

3. **Review Process**
   - All PRs require review from maintainers
   - Address feedback promptly
   - Keep PRs focused and reasonably sized
   - Be patient and respectful during reviews

## 🐛 Bug Reports

When reporting bugs, please include:

```markdown
**Bug Description**
A clear description of what the bug is.

**Steps to Reproduce**
1. Go to '...'
2. Click on '...'
3. See error

**Expected Behavior**
What you expected to happen.

**Actual Behavior**
What actually happened.

**Environment**
- OS: [e.g., Ubuntu 22.04]
- Rust version: [e.g., 1.75.0]
- GhostFlow version: [e.g., 0.1.0]
- Docker version (if applicable): [e.g., 24.0.0]

**Additional Context**
- Error logs
- Screenshots
- Configuration files (sanitized)
```

## 💡 Feature Requests

For feature requests, please provide:

```markdown
**Feature Description**
Clear description of the feature you'd like to see.

**Use Case**
Why would this feature be useful? What problem does it solve?

**Proposed Solution**
How do you envision this working?

**Alternatives Considered**
What other solutions have you considered?

**Additional Context**
- Mockups or examples
- Related issues or discussions
- Implementation ideas
```

## 🔌 Creating New Nodes

New nodes are one of the most valuable contributions! See our [Node Development Guide](docs/DEVELOPMENT.md#creating-custom-nodes) for detailed instructions.

### Node Contribution Checklist

- [ ] Node implements the `Node` trait correctly
- [ ] Comprehensive input/output validation
- [ ] Error handling for all failure modes
- [ ] Unit tests with good coverage
- [ ] Integration test with real flow execution
- [ ] Documentation with usage examples
- [ ] Icon and color specified
- [ ] Follows naming conventions

### Popular Node Ideas

**Integrations:**
- Slack/Discord notifications
- GitHub/GitLab operations
- Email (SMTP/IMAP)
- AWS services (S3, Lambda, etc.)
- Google Cloud services
- Microsoft Office 365

**Data Processing:**
- CSV/JSON/XML parsing
- Data validation and cleaning
- Format conversions
- Aggregations and transformations

**AI/ML:**
- OpenAI/Anthropic integrations
- Vector database operations (Qdrant, Weaviate)
- Document processing (PDF, DOCX)
- Image processing
- Text analysis

## 🧪 Testing Guidelines

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use ghostflow_schema::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_node_execution() {
        let node = MyNode::new();
        let context = create_test_context();
        
        let result = node.execute(context).await;
        
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output["status"], "success");
    }

    fn create_test_context() -> ExecutionContext {
        ExecutionContext {
            execution_id: Uuid::new_v4(),
            flow_id: Uuid::new_v4(),
            node_id: "test".to_string(),
            input: serde_json::json!({
                "test_param": "test_value"
            }),
            variables: std::collections::HashMap::new(),
            secrets: std::collections::HashMap::new(),
            artifacts: std::collections::HashMap::new(),
        }
    }
}
```

### Test Categories

1. **Unit Tests** - Test individual functions and methods
2. **Integration Tests** - Test component interactions
3. **End-to-End Tests** - Test complete flow execution
4. **Performance Tests** - Benchmark critical paths

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests with specific log level
RUST_LOG=debug cargo test

# Run integration tests only
cargo test --test integration
```

## 📚 Documentation

### Types of Documentation

1. **Code Comments** - Explain complex logic
2. **Doc Comments** - Rust documentation (`///`)
3. **README Updates** - Keep the main README current
4. **API Documentation** - Update docs/API.md
5. **Tutorials** - Step-by-step guides
6. **Examples** - Working code examples

### Documentation Standards

```rust
/// Brief description of the function.
/// 
/// More detailed explanation if needed. Explain parameters,
/// return values, and any side effects.
/// 
/// # Arguments
/// 
/// * `param1` - Description of parameter 1
/// * `param2` - Description of parameter 2
/// 
/// # Returns
/// 
/// Description of return value
/// 
/// # Errors
/// 
/// This function will return an error if:
/// - Condition 1 occurs
/// - Condition 2 happens
/// 
/// # Examples
/// 
/// ```
/// use ghostflow_core::Node;
/// 
/// let node = MyNode::new();
/// let result = node.execute(context).await?;
/// ```
pub async fn my_function(param1: &str, param2: i32) -> Result<String> {
    // Implementation
}
```

## 🎨 UI/UX Contributions

For UI improvements:

1. **Design Consistency** - Follow existing patterns
2. **Accessibility** - Ensure WCAG compliance
3. **Responsive Design** - Test on different screen sizes
4. **User Testing** - Get feedback from actual users
5. **Documentation** - Update UI documentation

### UI Development Setup

```bash
cd crates/ghostflow-ui

# Install trunk for WASM development
cargo install trunk

# Start development server
trunk serve --open

# Build for production
trunk build --release
```

## 🏆 Recognition

We value all contributions! Contributors are recognized through:

- **GitHub Contributors** - Listed in repository
- **Release Notes** - Major contributions mentioned
- **Hall of Fame** - Regular contributors highlighted
- **Swag** - Stickers and merchandise (when available)
- **Early Access** - Preview new features
- **Community Status** - Special roles in Discord

## 📞 Getting Help

**Before Contributing:**
- Check existing [Issues](https://github.com/ghostkellz/ghostflow/issues)
- Read the [Development Guide](docs/DEVELOPMENT.md)
- Browse [Discussions](https://github.com/ghostkellz/ghostflow/discussions)

**Need Help?**
- 💬 [GitHub Discussions](https://github.com/ghostkellz/ghostflow/discussions) - Questions and general discussion
- 🐛 [GitHub Issues](https://github.com/ghostkellz/ghostflow/issues) - Bug reports and feature requests
- 💬 [Discord Community](https://discord.gg/ghostflow) - Real-time chat and support
- 📧 Email: contributors@ghostflow.dev

## 🌟 Contributor Resources

### Learning Resources

- [Rust Book](https://doc.rust-lang.org/book/) - Learn Rust programming
- [Async Rust](https://rust-lang.github.io/async-book/) - Asynchronous programming
- [Leptos Guide](https://leptos.dev/) - Web UI framework
- [Axum Examples](https://github.com/tokio-rs/axum/tree/main/examples) - Web server framework

### Development Tools

```bash
# Useful cargo tools
cargo install cargo-watch      # Auto-rebuild on changes
cargo install cargo-expand     # Expand macros
cargo install cargo-audit      # Security audit
cargo install cargo-machete    # Find unused dependencies
cargo install cargo-tarpaulin  # Code coverage
```

### IDE Setup

**VS Code Extensions:**
- rust-analyzer
- CodeLLDB (debugging)
- Error Lens
- GitLens

**Settings:**
```json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.cargo.features": "all",
    "rust-analyzer.procMacro.enable": true
}
```

## 🤔 Decision Making

### RFC Process

For major changes, we use an RFC (Request for Comments) process:

1. **Create RFC** - Document in `rfcs/` directory
2. **Community Discussion** - Get feedback
3. **Maintainer Review** - Technical review
4. **Implementation** - Code the approved RFC

### Governance

- **Maintainers** - Core team with commit access
- **Community** - All contributors and users
- **Decisions** - Made through consensus and discussion
- **Conflicts** - Resolved through respectful dialogue

## 📜 Code of Conduct

We are committed to providing a welcoming and inspiring community for all. Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md).

### Our Standards

**Positive Behavior:**
- Using welcoming and inclusive language
- Being respectful of differing viewpoints
- Gracefully accepting constructive criticism
- Focusing on what is best for the community
- Showing empathy towards other community members

**Unacceptable Behavior:**
- Harassment or discrimination
- Trolling or insulting comments
- Publishing others' private information
- Any conduct that could be considered inappropriate

## 🎉 Thank You!

Every contribution makes GhostFlow better. Whether you:
- Fix a typo in documentation
- Report a bug
- Suggest a feature
- Contribute code
- Help other users

**You are making a difference!** 

The open-source community is built on collaboration, and we're grateful to have you as part of the GhostFlow family.

---

*Happy Contributing! 🚀*

---

*Contributing guidelines version 1.0 - Last updated: 2025-01-08*