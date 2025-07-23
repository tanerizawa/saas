#!/bin/bash

# Warna untuk output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${YELLOW}=== SAAS UMKM Platform - Init Project ===${NC}"
echo -e "${YELLOW}This script will initialize your development environment.${NC}"
echo -e "${RED}WARNING: All existing data will be reset!${NC}"
echo ""
read -p "Do you want to continue? (y/n): " confirm

if [[ $confirm != [yY] ]]; then
  echo -e "${YELLOW}Operation cancelled.${NC}"
  exit 0
fi

# 1. Reset Docker containers
echo -e "\n${YELLOW}Step 1: Resetting Docker containers...${NC}"
./scripts/reset_docker_containers.sh
if [ $? -ne 0 ]; then
  echo -e "${RED}Failed to reset Docker containers.${NC}"
  exit 1
fi

# 2. Clean database
echo -e "\n${YELLOW}Step 2: Cleaning database...${NC}"
./scripts/clean_database.sh
if [ $? -ne 0 ]; then
  echo -e "${RED}Failed to clean database.${NC}"
  exit 1
fi

# 3. Setup development environment
echo -e "\n${YELLOW}Step 3: Setting up development environment...${NC}"
./scripts/setup-dev.sh
if [ $? -ne 0 ]; then
  echo -e "${RED}Failed to setup development environment.${NC}"
  exit 1
fi

# 4. Run migrations
echo -e "\n${YELLOW}Step 4: Running database migrations...${NC}"
cd backend && cargo run --bin migrate
if [ $? -ne 0 ]; then
  echo -e "${RED}Failed to run migrations.${NC}"
  exit 1
fi
cd ..

# 5. Verify setup
echo -e "\n${YELLOW}Step 5: Verifying setup...${NC}"

# Check if Docker containers are running
RUNNING_COUNT=$(docker-compose ps --services | xargs -I{} docker-compose ps -q {} | xargs docker inspect -f '{{.State.Running}}' | grep -c "true")
EXPECTED_COUNT=$(docker-compose ps --services | wc -l | tr -d ' ')

if [ "$RUNNING_COUNT" -eq "$EXPECTED_COUNT" ]; then
  echo -e "${GREEN}✓ All Docker containers are running${NC}"
else
  echo -e "${RED}✗ Some Docker containers are not running${NC}"
fi

# Check if database is accessible
DB_HOST=${DB_HOST:-localhost}
DB_PORT=${DB_PORT:-5432}
DB_USER=${DB_USER:-postgres}
DB_PASSWORD=${DB_PASSWORD:-postgres}
DB_NAME=${DB_NAME:-saas_umkm}

if PGPASSWORD="$DB_PASSWORD" psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -c '\q' 2>/dev/null; then
  echo -e "${GREEN}✓ Database is accessible${NC}"
else
  echo -e "${RED}✗ Database is not accessible${NC}"
fi

# Rust backend check
if [ -d "./backend/target" ]; then
  echo -e "${GREEN}✓ Rust backend is compiled${NC}"
else
  echo -e "${YELLOW}⚠ Rust backend is not compiled yet${NC}"
fi

# Frontend dependencies check
if [ -d "./frontend/node_modules" ]; then
  echo -e "${GREEN}✓ Frontend dependencies are installed${NC}"
else
  echo -e "${YELLOW}⚠ Frontend dependencies are not installed yet${NC}"
fi

echo -e "\n${GREEN}Project initialization completed!${NC}"
echo -e "${YELLOW}You can now start the backend with: ${NC}cd backend && cargo run --bin server"
echo -e "${YELLOW}You can now start the frontend with: ${NC}cd frontend && npm run dev"
