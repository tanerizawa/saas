#!/bin/bash
# run_tests.sh - Script to run all tests for the SaaS backend

set -e  # Exit immediately if a command exits with a non-zero status

echo "=========================================="
echo "Running tests for SaaS Backend"
echo "=========================================="

# Run in-memory repository tests
echo "\n\n🧪 Running in-memory repository tests..."
cargo run --bin in_memory_user_repo_test

# Check if database is available
echo "\n\n🔍 Checking database availability..."
cargo run --bin db_setup

# If DB setup completed successfully, run the database-dependent tests
if [ $? -eq 0 ]; then
    echo "\n\n🧪 Running database repository tests..."
    # Extract the environment variables from db_setup output
    DB_URL=$(cargo run --bin db_setup | grep TEST_DATABASE_URL | cut -d'"' -f2)
    SCHEMA=$(cargo run --bin db_setup | grep TEST_SCHEMA | cut -d'"' -f2)
    
    # Run tests with the environment variables
    TEST_DATABASE_URL="$DB_URL" TEST_SCHEMA="$SCHEMA" cargo test
else
    echo "\n❌ Database setup failed, skipping database-dependent tests."
    echo "Run 'cargo run --bin db_setup' to troubleshoot database connection issues."
fi

# Run unit tests (these don't depend on a database)
echo "\n\n🧪 Running unit tests..."
cargo test --lib

echo "\n\n✅ Testing completed!"
