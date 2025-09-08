#!/bin/bash

# GhostFlow Quick Start Script

set -e

echo "🚀 Starting GhostFlow..."

# Check dependencies
command -v docker >/dev/null 2>&1 || { echo "❌ Docker is required but not installed. Aborting." >&2; exit 1; }
command -v docker-compose >/dev/null 2>&1 || { echo "❌ Docker Compose is required but not installed. Aborting." >&2; exit 1; }

# Parse arguments
MODE=${1:-dev}

case $MODE in
  dev)
    echo "📦 Starting in development mode..."
    docker-compose --profile dev up -d
    echo "✅ GhostFlow is running!"
    echo "   API: http://localhost:3000"
    echo "   UI: http://localhost:8080"
    echo "   Adminer: http://localhost:8081"
    echo "   MinIO Console: http://localhost:9001"
    echo "   Ollama: http://localhost:11434"
    ;;
  
  prod)
    echo "🏭 Starting in production mode..."
    docker-compose up -d
    echo "✅ GhostFlow is running!"
    echo "   API: http://localhost:3000"
    echo "   UI: http://localhost:8080"
    ;;
  
  build)
    echo "🔨 Building GhostFlow..."
    docker-compose build
    echo "✅ Build complete!"
    ;;
  
  stop)
    echo "🛑 Stopping GhostFlow..."
    docker-compose down
    echo "✅ GhostFlow stopped!"
    ;;
  
  clean)
    echo "🧹 Cleaning up GhostFlow..."
    docker-compose down -v
    echo "✅ Cleanup complete!"
    ;;
  
  logs)
    docker-compose logs -f ghostflow
    ;;
  
  *)
    echo "Usage: $0 {dev|prod|build|stop|clean|logs}"
    exit 1
    ;;
esac