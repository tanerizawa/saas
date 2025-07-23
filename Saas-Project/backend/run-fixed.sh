#!/bin/bash

# Script to start the backend server with host mappings
export DATABASE_URL="postgresql://saas_user:saas_password@127.0.0.1:5432/saas_umkm_db" 
export REDIS_URL="redis://127.0.0.1:6379"
export JWT_SECRET="your_secure_jwt_secret_key_here"
export APP_HOST="0.0.0.0"
export APP_PORT="8000"

echo "🚀 Starting backend server with fixed database connections..."
echo "📊 DATABASE_URL=$DATABASE_URL"
echo "📊 REDIS_URL=$REDIS_URL"

# Use IPv4 address instead of hostname
cargo run --bin server
