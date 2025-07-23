# 🔧 Platform SaaS UMKM - Troubleshooting Guide

## Common Issues and Solutions

### 🦀 Backend Issues

#### Compilation Errors

**Problem**: `cargo build` fails with dependency errors

```bash
error: failed to resolve dependencies
```

**Solution**:

```bash
# Update Rust toolchain
rustup update

# Clean build artifacts
cargo clean

# Rebuild
cargo build
```

#### Database Connection Issues

**Problem**: `Error: Connection refused (os error 61)`

**Solution**:

1. Check if PostgreSQL is running:

   ```bash
   docker-compose ps
   ```

2. Restart PostgreSQL service:

   ```bash
   docker-compose restart postgres
   ```

3. Verify connection string in `.env`:
   ```env
   DATABASE_URL=postgresql://saas_user:saas_password@localhost:5432/saas_umkm_db
   ```

#### Migration Failures

**Problem**: `cargo run --bin migrate` fails

**Solution**:

```bash
# Check if database exists
psql -h localhost -U saas_user -d saas_umkm_db -c "SELECT version();"

# Drop and recreate database
docker-compose exec postgres psql -U postgres -c "DROP DATABASE IF EXISTS saas_umkm_db;"
docker-compose exec postgres psql -U postgres -c "CREATE DATABASE saas_umkm_db OWNER saas_user;"

# Run migration again
cd backend && cargo run --bin migrate
```

#### Port Already in Use

**Problem**: `Error: Address already in use (os error 48)`

**Solution**:

```bash
# Find process using port 8000
lsof -i :8000

# Kill the process
kill -9 <PID>

# Or use different port in .env
APP_PORT=8001
```

### ⚛️ Frontend Issues

#### Node.js Version Incompatibility

**Problem**: `npm install` fails with version errors

**Solution**:

```bash
# Check Node.js version
node --version

# Use Node Version Manager (nvm)
nvm install 18
nvm use 18

# Clear cache and reinstall
rm -rf node_modules package-lock.json
npm install
```

#### Build Errors

**Problem**: `npm run build` fails with TypeScript errors

**Solution**:

```bash
# Type check
npm run type-check

# Fix TypeScript issues or add type assertions
# Then rebuild
npm run build
```

#### Next.js Port Conflicts

**Problem**: Port 3000 already in use

**Solution**:

```bash
# Use different port
npm run dev -- -p 3001

# Or kill existing process
npx kill-port 3000
```

### 🐳 Docker Issues

#### Services Not Starting

**Problem**: `docker-compose up -d` fails

**Solution**:

```bash
# Check Docker daemon
docker info

# Check for port conflicts
docker-compose ps
netstat -an | grep LISTEN

# Restart Docker service
sudo systemctl restart docker  # Linux
# Or restart Docker Desktop on macOS/Windows
```

#### PostgreSQL Init Errors

**Problem**: Database initialization fails

**Solution**:

```bash
# Remove existing volume
docker-compose down -v

# Start fresh
docker-compose up -d

# Check logs
docker-compose logs postgres
```

### 🔐 Authentication Issues

#### JWT Token Invalid

**Problem**: API returns 401 Unauthorized

**Solution**:

1. Check JWT secret in `.env`
2. Ensure token is properly formatted:
   ```
   Authorization: Bearer <your-jwt-token>
   ```
3. Check token expiration

#### CORS Errors

**Problem**: Browser blocks API requests

**Solution**:
Add to backend CORS configuration:

```rust
.allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
```

### 🔍 Debugging Tips

#### Enable Debug Logging

```bash
# Backend
export RUST_LOG=debug,saas_umkm_backend=trace

# Frontend
export NODE_ENV=development
export NEXT_PUBLIC_DEBUG=true
```

#### Use VS Code Debugger

1. Set breakpoints in VS Code
2. Press F5 or use "Run and Debug" panel
3. Select appropriate debug configuration

#### Database Debugging

```bash
# Connect to database
docker-compose exec postgres psql -U saas_user -d saas_umkm_db

# Check tables
\dt

# Check user data
SELECT * FROM users LIMIT 5;
```

#### Network Debugging

```bash
# Test backend API
curl -X GET http://localhost:8000/health

# Test with authentication
curl -X GET http://localhost:8000/api/v1/users \
  -H "Authorization: Bearer <your-token>"

# Check network connectivity
ping localhost
telnet localhost 8000
```

### 📱 Performance Issues

#### Slow Backend Response

**Solution**:

1. Check database connection pool settings
2. Add indexes to frequently queried columns
3. Use database query optimization
4. Enable request logging:
   ```rust
   RUST_LOG=tower_http=debug
   ```

#### Frontend Loading Slowly

**Solution**:

1. Check for large bundle sizes:
   ```bash
   npm run analyze
   ```
2. Implement code splitting
3. Optimize images and assets
4. Use Next.js Image optimization

### 🧪 Testing Issues

#### Tests Failing

**Problem**: `cargo test` or `npm test` fails

**Solution**:

```bash
# Backend tests
cargo test -- --nocapture

# Frontend tests
npm test -- --verbose

# Run specific test
cargo test test_user_creation
npm test UserComponent
```

### 🚀 Deployment Issues

#### Environment Variables

**Problem**: Application crashes in production

**Solution**:

1. Verify all required environment variables are set
2. Check variable types (strings vs numbers)
3. Validate database connection in production

#### Docker Build Failures

**Solution**:

```bash
# Build with no cache
docker-compose build --no-cache

# Check Dockerfile syntax
docker build -t test-build .

# Use multi-stage builds for optimization
```

## Getting Help

If you're still experiencing issues:

1. **Check Logs**: Always check application logs for detailed error messages
2. **Search Issues**: Look through existing GitHub issues
3. **Create Issue**: Provide detailed information including:
   - OS and version
   - Node.js and Rust versions
   - Complete error messages
   - Steps to reproduce

## Quick Health Check Script

Save this as `health-check.sh`:

```bash
#!/bin/bash

echo "🏥 Platform SaaS UMKM Health Check"
echo "=================================="

# Check Docker
if docker info > /dev/null 2>&1; then
    echo "✅ Docker is running"
else
    echo "❌ Docker is not running"
fi

# Check services
if docker-compose ps | grep -q "Up"; then
    echo "✅ Docker services are running"
else
    echo "❌ Docker services are not running"
fi

# Check backend
if curl -s http://localhost:8000/health > /dev/null; then
    echo "✅ Backend is responding"
else
    echo "❌ Backend is not responding"
fi

# Check frontend
if curl -s http://localhost:3000 > /dev/null; then
    echo "✅ Frontend is responding"
else
    echo "❌ Frontend is not responding"
fi

echo "=================================="
echo "Health check complete!"
```

Run with: `chmod +x health-check.sh && ./health-check.sh`
