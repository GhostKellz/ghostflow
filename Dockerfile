# Multi-stage build for GhostFlow

# Stage 1: Build environment
FROM rust:1.75-bookworm AS builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Build in release mode
RUN cargo build --release

# Stage 2: Runtime environment
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 ghostflow

# Copy built binaries
COPY --from=builder /app/target/release/ghostflow-server /usr/local/bin/
COPY --from=builder /app/target/release/gflow /usr/local/bin/

# Copy migrations
COPY migrations /app/migrations

# Set working directory
WORKDIR /app

# Switch to non-root user
USER ghostflow

# Expose ports
EXPOSE 3000 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Default command
CMD ["ghostflow-server"]