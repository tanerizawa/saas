#!/bin/bash

# Script untuk menjalankan development environment dengan path yang tepat
# Gunakan script ini untuk memastikan semua command dijalankan dari direktori yang benar

set -e  # Exit on any error

WORKSPACE_ROOT="/Users/odangrodiana/Desktop/01_DEVELOPMENT_PROJECTS/saas"
BACKEND_DIR="$WORKSPACE_ROOT/Saas-Project/backend"
PROJECT_DIR="$WORKSPACE_ROOT/Saas-Project"

echo "🚀 Starting SaaS UMKM Development Environment"
echo "Workspace: $WORKSPACE_ROOT"
echo "Backend: $BACKEND_DIR"
echo "Project: $PROJECT_DIR"

# Function to run commands in backend directory
run_backend() {
    echo "📁 Running in backend directory: $1"
    cd "$BACKEND_DIR" && $1
}

# Function to run commands in project directory 
run_project() {
    echo "📁 Running in project directory: $1"
    cd "$PROJECT_DIR" && $1
}

# Check if backend directory exists
if [ ! -d "$BACKEND_DIR" ]; then
    echo "❌ Backend directory not found: $BACKEND_DIR"
    exit 1
fi

# Parse command line arguments
case "${1:-help}" in
    "server"|"run")
        echo "🔄 Starting backend server..."
        run_backend "cargo run --bin server"
        ;;
    "build")
        echo "🔨 Building backend..."
        run_backend "cargo build"
        ;;
    "check")
        echo "✅ Checking backend..."
        run_backend "cargo check"
        ;;
    "test")
        echo "🧪 Running tests..."
        run_backend "cargo test"
        ;;
    "docker-up")
        echo "🐳 Starting Docker services..."
        run_project "docker compose up -d"
        ;;
    "docker-down")
        echo "🐳 Stopping Docker services..."
        run_project "docker compose down"
        ;;
    "migrate")
        echo "🗄️  Running database migrations..."
        run_backend "sqlx migrate run"
        ;;
    "full-setup")
        echo "🔄 Running full development setup..."
        run_project "docker compose up -d"
        echo "⏳ Waiting for database to be ready..."
        sleep 5
        run_backend "sqlx migrate run"
        echo "🚀 Starting backend server..."
        run_backend "cargo run --bin server"
        ;;
    "test-api")
        echo "🧪 Testing API endpoints..."
        run_project "./test-api.sh"
        ;;
    "logs")
        echo "📋 Showing Docker logs..."
        run_project "docker compose logs -f"
        ;;
    "help"|*)
        echo "Available commands:"
        echo "  server, run     - Start backend server"
        echo "  build          - Build backend"
        echo "  check          - Check backend code"
        echo "  test           - Run tests"
        echo "  docker-up      - Start Docker services"
        echo "  docker-down    - Stop Docker services"
        echo "  migrate        - Run database migrations"
        echo "  full-setup     - Complete development setup"
        echo "  test-api       - Test API endpoints"
        echo "  logs           - Show Docker logs"
        echo "  help           - Show this help"
        ;;
esac
