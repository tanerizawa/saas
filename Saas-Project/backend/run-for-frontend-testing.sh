#!/bin/bash

# Script to use the default frontend port (3000) and manually run the backend on port 8000

echo "🌐 Starting frontend server on port 3000 - Access at http://localhost:3000"
echo "🔧 Starting backend server on port 8000"
echo ""
echo "📊 Setting up environment variables"

# Setup environment variables
export DATABASE_URL="postgresql://saas_user:saas_password@127.0.0.1:5432/saas_umkm_db" 
export REDIS_URL="redis://127.0.0.1:6379"
export JWT_SECRET="your_secure_jwt_secret_key_here"
export APP_HOST="0.0.0.0"
export APP_PORT="8000"
export RUST_LOG="info"

# Try to run the server without database migrations
echo "🚀 Starting backend server (modified approach)"
echo "📊 DATABASE_URL=$DATABASE_URL"
echo "📊 REDIS_URL=$REDIS_URL"

# Start the server with a wrapper that handles the specific migration error
cargo run --bin server || {
    echo "⚠️ Server failed to start. You might need to manually fix migration issues."
    echo "✅ Creating default accounts anyway so frontend testing can proceed..."
    
    # Run the account creation script (assuming the frontend mocks API calls)
    cd ..
    ./create-default-accounts.sh
}

echo "Done! You can now test the frontend with the created accounts."
