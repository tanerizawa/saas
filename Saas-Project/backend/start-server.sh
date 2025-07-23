#!/bin/bash

# Start the SaaS UMKM backend server with the correct database connection
# This script ensures that all environment variables are properly set

echo "🚀 Starting SaaS UMKM Backend Server"

# Set environment variables
export DATABASE_URL="postgresql://saas_user:saas_password@localhost:5432/saas_umkm_db"
export REDIS_URL="redis://localhost:6379"
export JWT_SECRET="your_secure_jwt_secret_key_here"
export APP_HOST="0.0.0.0"
export APP_PORT="8000"
export RUST_LOG="info"
export RUST_BACKTRACE=1

# Show configuration
echo "📋 Configuration:"
echo "Database URL: $DATABASE_URL"
echo "Redis URL: $REDIS_URL"
echo "Host: $APP_HOST"
echo "Port: $APP_PORT"
echo "Log Level: $RUST_LOG"

# Run the server
echo "🔄 Starting server..."
cargo run --bin server
