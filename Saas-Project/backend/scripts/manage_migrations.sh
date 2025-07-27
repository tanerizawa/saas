#!/bin/bash

# Migration management script for Saas Project backend
# This script helps with applying, rolling back, and checking the status of database migrations

set -e

# Colors for terminal output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get the directory of this script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
MIGRATIONS_DIR="$SCRIPT_DIR/migrations"
ROLLBACKS_DIR="$SCRIPT_DIR/migrations/rollbacks"

# Function to show usage information
function show_help {
    echo -e "${BLUE}Usage:${NC}"
    echo -e "  $0 [command]"
    echo
    echo -e "${BLUE}Commands:${NC}"
    echo -e "  ${GREEN}up${NC}           Apply all pending migrations"
    echo -e "  ${GREEN}down${NC}         Roll back the most recent migration"
    echo -e "  ${GREEN}down [version]${NC}  Roll back to the specified version"
    echo -e "  ${GREEN}status${NC}       Show the current migration status"
    echo -e "  ${GREEN}create [name]${NC}   Create a new migration file"
    echo -e "  ${GREEN}help${NC}         Show this help message"
    echo
}

# Function to apply migrations
function apply_migrations {
    echo -e "${BLUE}Applying pending migrations...${NC}"
    cargo run --bin migrate up
    echo -e "${GREEN}Migrations completed successfully.${NC}"
}

# Function to roll back migrations
function rollback_migrations {
    local version=$1
    
    if [ -z "$version" ]; then
        echo -e "${YELLOW}Rolling back the latest migration...${NC}"
        cargo run --bin migrate down
    else
        echo -e "${YELLOW}Rolling back to version $version...${NC}"
        cargo run --bin migrate down "$version"
    fi
    
    echo -e "${GREEN}Rollback completed successfully.${NC}"
}

# Function to show migration status
function show_status {
    echo -e "${BLUE}Current migration status:${NC}"
    cargo run --bin migrate status
}

# Function to create a new migration file
function create_migration {
    local name=$1
    
    if [ -z "$name" ]; then
        echo -e "${RED}Error: Migration name is required${NC}"
        show_help
        exit 1
    fi
    
    local timestamp=$(date +"%Y%m%d%H%M%S")
    local filename="${timestamp}_${name}.sql"
    local rollback_filename="${timestamp}_${name}_rollback.sql"
    
    echo -e "${BLUE}Creating new migration: $filename${NC}"
    
    # Create migration file
    cat > "$MIGRATIONS_DIR/$filename" << EOF
-- Migration: $name
-- Created at: $(date -u)

-- Write your SQL migration here

EOF
    
    # Create rollback file
    mkdir -p "$ROLLBACKS_DIR"
    cat > "$ROLLBACKS_DIR/$rollback_filename" << EOF
-- Rollback script for migration $filename

-- Write your rollback SQL here

EOF
    
    echo -e "${GREEN}Created migration file: $filename${NC}"
    echo -e "${GREEN}Created rollback file: $rollback_filename${NC}"
}

# Main script logic
if [ "$#" -eq 0 ]; then
    show_help
    exit 0
fi

case "$1" in
    "up")
        apply_migrations
        ;;
    "down")
        rollback_migrations "$2"
        ;;
    "status")
        show_status
        ;;
    "create")
        create_migration "$2"
        ;;
    "help" | "--help" | "-h")
        show_help
        ;;
    *)
        echo -e "${RED}Error: Unknown command '$1'${NC}"
        show_help
        exit 1
        ;;
esac
