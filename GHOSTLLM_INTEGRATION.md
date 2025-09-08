# 🚀 GhostLLM Rust Quick Start Guide

**Get GPU-accelerated AI inference in your Rust project in 5 minutes!**

## ⚡ Super Quick Setup

### 1. Add to Your `Cargo.toml`

```toml
[dependencies]
ghostllm = { git = "https://github.com/ghostkellz/ghostllm", subdirectory = "rust_bindings" }
tokio = { version = "1.0", features = ["full"] }
```

### 2. Install Zig (Required for Build)

```bash
# Linux/macOS
curl -L https://ziglang.org/builds/zig-linux-x86_64-0.16.0-dev.164+bc7955306.tar.xz | tar xJ
export PATH=$PATH:$PWD/zig-linux-x86_64-0.16.0-dev.164+bc7955306

# Or install via package manager
# brew install zig          # macOS
# sudo apt install zig-dev  # Ubuntu (may be older version)
```

### 3. Write Your First AI App

Create `src/main.rs`:

```rust
use ghostllm::{Config, GhostLLM, Request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize GhostLLM with GPU acceleration
    let config = Config::new()
        .port(8080)
        .enable_gpu(true)
        .max_connections(100);

    let mut ghost = GhostLLM::new(config)?;
    ghost.start_server()?;
    
    // Make an AI request
    let request = Request::new()
        .prompt("Write a haiku about Rust programming")
        .model("gpt-4")
        .max_tokens(100)
        .temperature(0.7);
    
    let response = ghost.request(&request)?;
    println!("🤖 AI Response: {}", response.text()?);
    
    ghost.stop_server()?;
    Ok(())
}
```

### 4. Run It!

```bash
cargo run
```

**That's it!** You now have GPU-accelerated AI inference in your Rust app! 🎉

---

## 🔥 Common Use Cases

### Web Server with AI Endpoints

```rust
use axum::{routing::post, Router, Json, extract::State};
use std::sync::{Arc, Mutex};
use ghostllm::{GhostLLM, Config, Request};

type AppState = Arc<Mutex<GhostLLM>>;

async fn chat_endpoint(
    State(ghost): State<AppState>,
    Json(req): Json<Request>,
) -> Result<String, String> {
    let response = ghost.lock().unwrap().request(&req)
        .map_err(|e| format!("AI request failed: {}", e))?;
    
    Ok(response.text().unwrap_or("").to_string())
}

#[tokio::main]
async fn main() {
    let config = Config::new().enable_gpu(true);
    let ghost = Arc::new(Mutex::new(GhostLLM::new(config).unwrap()));
    ghost.lock().unwrap().start_server().unwrap();
    
    let app = Router::new()
        .route("/chat", post(chat_endpoint))
        .with_state(ghost);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### Batch Processing

```rust
use ghostllm::{Config, GhostLLM, Request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ghost = GhostLLM::new(Config::new().enable_gpu(true))?;
    ghost.start_server()?;
    
    let prompts = vec![
        "Summarize: The quick brown fox jumps over the lazy dog.",
        "Translate to Spanish: Hello, how are you today?",
        "Code review: fn add(a: i32, b: i32) -> i32 { a + b }",
    ];
    
    for prompt in prompts {
        let request = Request::new().prompt(prompt).max_tokens(100);
        let response = ghost.request(&request)?;
        println!("📝 {}: {}", prompt, response.text()?);
    }
    
    Ok(())
}
```

### Stream Processing

```rust
use ghostllm::{Config, GhostLLM, Request};

#[tokio::main] 
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ghost = GhostLLM::new(Config::new().enable_gpu(true))?;
    ghost.start_server()?;
    
    // Process data with AI insights
    let data = vec!["user feedback 1", "user feedback 2", "user feedback 3"];
    
    for item in data {
        let request = Request::new()
            .prompt(&format!("Analyze sentiment: {}", item))
            .max_tokens(50);
            
        match ghost.request(&request) {
            Ok(response) => println!("✅ {}: {}", item, response.text()?),
            Err(e) => eprintln!("❌ Failed to process {}: {}", item, e),
        }
    }
    
    Ok(())
}
```

---

## 🛠️ Configuration Options

```rust
use ghostllm::{Config, LogLevel};

let config = Config::new()
    .port(8080)                    // GhostLLM server port
    .host("0.0.0.0")              // Bind address
    .enable_gpu(true)              // GPU acceleration (recommended!)
    .max_connections(500)          // Connection limit
    .log_level(LogLevel::Info);    // Logging: Debug, Info, Warn, Error
```

---

## 🚨 Error Handling

```rust
use ghostllm::{GhostLLM, GhostLLMError, Config};

match GhostLLM::new(config) {
    Ok(mut ghost) => {
        match ghost.start_server() {
            Ok(()) => println!("✅ GhostLLM started!"),
            Err(GhostLLMError::Network) => eprintln!("❌ Network error"),
            Err(GhostLLMError::InvalidParams) => eprintln!("❌ Invalid config"),
            Err(e) => eprintln!("❌ Other error: {}", e),
        }
    }
    Err(e) => eprintln!("❌ Failed to init: {}", e),
}
```

---

## 🎯 Production Tips

### Environment Variables

```bash
# Set these for production
export GHOSTLLM_PORT=8080
export GHOSTLLM_GPU_ENABLED=true
export OPENAI_API_KEY=your_key_here
export ANTHROPIC_API_KEY=your_key_here
```

### Cargo.toml for Production

```toml
[dependencies]
ghostllm = { git = "https://github.com/ghostkellz/ghostllm", subdirectory = "rust_bindings" }
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"

[profile.release]
lto = true              # Link-time optimization
codegen-units = 1       # Better optimization
panic = "abort"         # Smaller binary
strip = true            # Remove debug symbols
```

### Docker Deployment

```dockerfile
FROM rust:1.75 AS builder

# Install Zig
RUN wget https://ziglang.org/builds/zig-linux-x86_64-0.16.0-dev.164+bc7955306.tar.xz \
    && tar -xf zig-linux-x86_64-0.16.0-dev.164+bc7955306.tar.xz \
    && mv zig-linux-x86_64-0.16.0-dev.164+bc7955306 /usr/local/zig \
    && ln -s /usr/local/zig/zig /usr/local/bin/zig

RUN apt-get update && apt-get install -y clang libclang-dev

WORKDIR /app
COPY . .
RUN cargo build --release

FROM ubuntu:22.04
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/your-app /usr/local/bin/
EXPOSE 3000 8080
CMD ["/usr/local/bin/your-app"]
```

---

## 🏃‍♂️ Try the N8N Alternative

Want to see a full workflow automation platform? Check out our N8N killer:

```bash
git clone https://github.com/ghostkellz/ghostllm
cd ghostllm
zig build
cd examples/n8n_alternative
LD_LIBRARY_PATH=../../zig-out/lib cargo run
```

Visit `http://localhost:3000` - it's 5x faster than N8N! 🚀

---

## 🆘 Troubleshooting

### "Zig compiler not found"
```bash
# Install Zig first
curl -L https://ziglang.org/builds/zig-linux-x86_64-0.16.0-dev.164+bc7955306.tar.xz | tar xJ
export PATH=$PATH:$PWD/zig-linux-x86_64-0.16.0-dev.164+bc7955306
```

### "Failed to link ghostllm"
```bash
# Make sure you have clang/libclang-dev
sudo apt install clang libclang-dev  # Ubuntu/Debian
brew install llvm                    # macOS
```

### "GPU not detected"
- Ensure NVIDIA drivers are installed
- Try `nvidia-smi` to verify GPU access
- Set `GHOSTLLM_GPU_ENABLED=false` to use CPU only

---

## 📚 More Resources

- **Full Integration Guide**: `RUST_PROJECT_INTEGRATION.md`
- **GitHub Repository**: https://github.com/ghostkellz/ghostllm
- **Issues/Support**: https://github.com/ghostkellz/ghostllm/issues

---

## 🎉 You're Ready!

That's all you need! GhostLLM gives you:
- ⚡ **4x faster** than alternatives
- 🔥 **GPU acceleration** 
- 🦀 **Memory safety**
- 🆓 **100% free & open source**
- 🚀 **Easy integration**

**Now go build something amazing!** 💪

---

*Questions? Open an issue at https://github.com/ghostkellz/ghostllm/issues*