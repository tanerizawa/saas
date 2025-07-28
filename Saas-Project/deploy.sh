#!/bin/bash

# SaaS UMKM Platform - Production Deployment Script
# Version: 1.0.0
# Date: July 28, 2025

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PROJECT_NAME="saas-umkm"
BACKEND_DIR="backend"
FRONTEND_DIR="frontend"
DATABASE_NAME="saas_umkm_db"
DATABASE_USER="saas_user"

echo -e "${BLUE}🚀 SaaS UMKM Platform - Production Deployment${NC}"
echo -e "${BLUE}================================================${NC}"

# Check if running as root for production
if [[ "$1" == "--production" ]] && [[ $EUID -eq 0 ]]; then
   echo -e "${RED}⚠️  Do not run production deployment as root!${NC}"
   exit 1
fi

# Function to print status
print_status() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

# Check system requirements
check_requirements() {
    print_info "Checking system requirements..."
    
    # Check for required tools
    command -v docker >/dev/null 2>&1 || { print_error "Docker is required but not installed."; exit 1; }
    command -v docker-compose >/dev/null 2>&1 || { print_error "Docker Compose is required but not installed."; exit 1; }
    command -v cargo >/dev/null 2>&1 || { print_error "Rust/Cargo is required but not installed."; exit 1; }
    command -v node >/dev/null 2>&1 || { print_error "Node.js is required but not installed."; exit 1; }
    command -v npm >/dev/null 2>&1 || { print_error "NPM is required but not installed."; exit 1; }
    
    print_status "All required tools are installed"
}

# Setup environment
setup_environment() {
    print_info "Setting up environment..."
    
    if [[ ! -f .env ]]; then
        print_warning ".env file not found. Creating from template..."
        cat > .env << EOF
# Production Environment Configuration
DATABASE_URL=postgresql://${DATABASE_USER}:saas_password@localhost:5432/${DATABASE_NAME}
DB_HOST=localhost
DB_PORT=5432
DB_NAME=${DATABASE_NAME}
DB_USER=${DATABASE_USER}  
DB_PASSWORD=saas_password

# Application Configuration
APP_HOST=0.0.0.0
APP_PORT=8000
JWT_SECRET=$(openssl rand -base64 32)
RUST_LOG=info

# Frontend Configuration  
NEXT_PUBLIC_API_URL=http://localhost:8000/api/v1
NEXT_PUBLIC_APP_NAME="SaaS UMKM Platform"

# Upload Configuration
UPLOAD_DIR=./uploads
MAX_FILE_SIZE=10485760

# Email Configuration (Configure for production)
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USER=your-email@gmail.com
SMTP_PASSWORD=your-app-password
FROM_EMAIL=noreply@your-domain.com
EOF
        print_warning "Please edit .env file with your production values before continuing!"
        print_info "Press Enter when ready to continue..."
        read -r
    fi
    
    print_status "Environment configuration ready"
}

# Start database
start_database() {
    print_info "Starting PostgreSQL database..."
    
    # Check if PostgreSQL is already running
    if docker-compose ps postgres | grep -q "Up"; then
        print_status "PostgreSQL is already running"
    else
        docker-compose up -d postgres
        print_info "Waiting for PostgreSQL to be ready..."
        sleep 10
        print_status "PostgreSQL started successfully"
    fi
}

# Run database migrations
run_migrations() {
    print_info "Running database migrations..."
    
    cd ${BACKEND_DIR}
    
    # Install sqlx-cli if not present
    if ! command -v sqlx >/dev/null 2>&1; then
        print_info "Installing sqlx-cli..."
        cargo install sqlx-cli --no-default-features --features postgres
    fi
    
    # Run migrations
    sqlx migrate run
    print_status "Database migrations completed"
    
    cd ..
}

# Build backend
build_backend() {
    print_info "Building backend..."
    
    cd ${BACKEND_DIR}
    
    # Clean previous build
    cargo clean
    
    # Build for production
    if [[ "$1" == "--production" ]]; then
        cargo build --release
        print_status "Backend built in release mode"
    else
        cargo build
        print_status "Backend built in development mode"
    fi
    
    cd ..
}

# Build frontend
build_frontend() {
    print_info "Building frontend..."
    
    cd ${FRONTEND_DIR}
    
    # Install dependencies
    npm install
    
    # Build for production
    if [[ "$1" == "--production" ]]; then
        npm run build
        print_status "Frontend built for production"
    else
        npm run dev &
        print_status "Frontend started in development mode"
    fi
    
    cd ..
}

# Start services
start_services() {
    print_info "Starting services..."
    
    if [[ "$1" == "--production" ]]; then
        # Production mode
        docker-compose -f docker-compose.prod.yml up -d
        print_status "Production services started"
    else
        # Development mode
        docker-compose up -d
        
        # Start backend server
        cd ${BACKEND_DIR}
        cargo run --bin server &
        BACKEND_PID=$!
        cd ..
        
        print_status "Development services started"
        print_info "Backend PID: $BACKEND_PID"
    fi
}

# Health check
health_check() {
    print_info "Performing health checks..."
    
    # Wait for services to start
    sleep 15
    
    # Check backend health
    if curl -s http://localhost:8000/health >/dev/null; then
        print_status "Backend health check passed"
    else
        print_error "Backend health check failed"
        return 1
    fi
    
    # Check authentication endpoints
    if curl -s http://localhost:8000/api/v1/auth/health >/dev/null; then
        print_status "Authentication system health check passed"
    else
        print_error "Authentication system health check failed"
        return 1
    fi
    
    print_status "All health checks passed"
}

# Display deployment info
show_deployment_info() {
    echo
    echo -e "${GREEN}🎉 Deployment completed successfully!${NC}"
    echo -e "${BLUE}===================================${NC}"
    echo
    echo -e "${BLUE}Service URLs:${NC}"
    echo -e "  Backend API: http://localhost:8000"
    echo -e "  Health Check: http://localhost:8000/health"
    echo -e "  Auth Health: http://localhost:8000/api/v1/auth/health"
    echo -e "  Frontend: http://localhost:3000"
    echo
    echo -e "${BLUE}Available API Endpoints:${NC}"
    echo -e "  POST /api/v1/auth/register - User registration"
    echo -e "  POST /api/v1/auth/login - User login"
    echo -e "  POST /api/v1/auth/refresh - Token refresh"
    echo -e "  POST /api/v1/auth/logout - User logout"
    echo -e "  POST /api/v1/auth/reset-password - Password reset"
    echo -e "  GET  /api/v1/users/profile - Get user profile"
    echo
    echo -e "${YELLOW}Next Steps:${NC}"
    echo -e "  1. Test authentication endpoints using test-auth-production.sh"
    echo -e "  2. Configure SSL certificates for HTTPS"
    echo -e "  3. Set up monitoring and logging"
    echo -e "  4. Configure backup procedures"
    echo
    echo -e "${BLUE}Logs and Debugging:${NC}"
    echo -e "  View backend logs: docker-compose logs backend"
    echo -e "  View database logs: docker-compose logs postgres"
    echo -e "  Backend source: ${BACKEND_DIR}/"
    echo -e "  Frontend source: ${FRONTEND_DIR}/"
    echo
}

# Cleanup function
cleanup() {
    if [[ -n "$BACKEND_PID" ]]; then
        print_info "Stopping backend server..."
        kill $BACKEND_PID 2>/dev/null || true
    fi
}

# Set trap for cleanup
trap cleanup EXIT

# Main deployment flow
main() {
    local mode=${1:-"--development"}
    
    print_info "Deployment mode: $mode"
    echo
    
    check_requirements
    setup_environment
    start_database
    run_migrations
    build_backend "$mode"
    build_frontend "$mode"
    start_services "$mode"
    health_check
    show_deployment_info
    
    if [[ "$mode" == "--development" ]]; then
        print_info "Development servers are running. Press Ctrl+C to stop."
        wait
    fi
}

# Show usage
show_usage() {
    echo "Usage: $0 [--production|--development]"
    echo
    echo "Options:"
    echo "  --production   Deploy for production environment"
    echo "  --development  Deploy for development environment (default)"
    echo
    echo "Examples:"
    echo "  $0                    # Development deployment"
    echo "  $0 --development      # Development deployment"
    echo "  $0 --production       # Production deployment"
    echo
}

# Handle command line arguments
case "${1:-}" in
    --production)
        main --production
        ;;
    --development|"")
        main --development
        ;;
    --help|-h)
        show_usage
        ;;
    *)
        print_error "Invalid option: $1"
        show_usage
        exit 1
        ;;
esac
