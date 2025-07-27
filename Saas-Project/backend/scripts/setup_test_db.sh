#!/bin/bash

# This script sets up a test database for running repository tests

# Database configuration
DB_USER="saas_user"
DB_PASSWORD="saas_password"
DB_NAME="saas_test_db"
DB_HOST="localhost"
DB_PORT="5432"

# Check if PostgreSQL is running
echo "Checking PostgreSQL service..."
if ! pg_isready -h $DB_HOST -p $DB_PORT > /dev/null 2>&1; then
  echo "Error: PostgreSQL is not running on $DB_HOST:$DB_PORT"
  echo "Please start PostgreSQL and try again"
  exit 1
fi

# Check if the user already exists
echo "Checking if database user exists..."
user_exists=$(psql -h $DB_HOST -p $DB_PORT -U postgres -tAc "SELECT 1 FROM pg_roles WHERE rolname='$DB_USER'")

if [ "$user_exists" != "1" ]; then
  echo "Creating database user $DB_USER..."
  psql -h $DB_HOST -p $DB_PORT -U postgres -c "CREATE USER $DB_USER WITH PASSWORD '$DB_PASSWORD';"
else
  echo "User $DB_USER already exists"
fi

# Check if the database already exists
echo "Checking if database exists..."
db_exists=$(psql -h $DB_HOST -p $DB_PORT -U postgres -tAc "SELECT 1 FROM pg_database WHERE datname='$DB_NAME'")

if [ "$db_exists" != "1" ]; then
  echo "Creating database $DB_NAME..."
  psql -h $DB_HOST -p $DB_PORT -U postgres -c "CREATE DATABASE $DB_NAME;"
  psql -h $DB_HOST -p $DB_PORT -U postgres -c "GRANT ALL PRIVILEGES ON DATABASE $DB_NAME TO $DB_USER;"
else
  echo "Database $DB_NAME already exists"
fi

# Set the environment variable for tests
export TEST_DATABASE_URL="postgres://$DB_USER:$DB_PASSWORD@$DB_HOST:$DB_PORT/$DB_NAME"
echo "Test database is ready. Connection string:"
echo $TEST_DATABASE_URL

echo "You can run tests with:"
echo "TEST_DATABASE_URL=$TEST_DATABASE_URL cargo test tests::user_repository_test"
