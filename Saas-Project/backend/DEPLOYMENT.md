# SaaS Backend Deployment Guide

This document provides instructions for deploying the SaaS backend application to various environments.

## Table of Contents
- [Prerequisites](#prerequisites)
- [Environment Variables](#environment-variables)
- [Deployment Options](#deployment-options)
  - [Docker Deployment](#docker-deployment)
  - [Kubernetes Deployment](#kubernetes-deployment)
- [Database Migrations](#database-migrations)
- [Monitoring](#monitoring)
- [Troubleshooting](#troubleshooting)

## Prerequisites

Before deploying, ensure you have:

- Docker and Docker Compose (for container deployment)
- PostgreSQL database
- Redis (optional, for caching and rate limiting)
- Access to SMTP server for email notifications

## Environment Variables

The application is configured using environment variables. Create a `.env` file with the following variables:

### Required Variables:
```
# Database
DATABASE_URL=postgres://username:password@hostname:5432/database_name

# Server
APP_HOST=0.0.0.0
APP_PORT=8080

# Authentication
JWT_SECRET=your_secure_secret_key
JWT_EXPIRES_IN=24h
JWT_REFRESH_EXPIRES_IN=7d

# File Storage
UPLOAD_DIR=/app/uploads
MAX_FILE_SIZE=10485760  # 10MB
```

### Optional Variables:
```
# Redis
REDIS_URL=redis://redis:6379

# SMTP (for emails)
SMTP_HOST=smtp.example.com
SMTP_PORT=587
SMTP_USERNAME=your_username
SMTP_PASSWORD=your_password
SMTP_FROM_EMAIL=noreply@example.com
SMTP_FROM_NAME=SaaS Application

# Rate Limiting
ENABLE_RATE_LIMITING=true
RATE_LIMIT_MAX_REQUESTS=100
RATE_LIMIT_WINDOW_SECS=60

# Logging
RUST_LOG=info,actix_web=info,sqlx=warn
```

## Deployment Options

### Docker Deployment

1. **Build the Docker image:**
   ```bash
   docker build -t saas-backend .
   ```

2. **Run with Docker Compose:**
   ```bash
   docker-compose up -d
   ```

3. **Check that services are running:**
   ```bash
   docker-compose ps
   ```

### Kubernetes Deployment

1. **Apply Kubernetes manifests:**
   ```bash
   kubectl apply -f k8s/
   ```

2. **Check pod status:**
   ```bash
   kubectl get pods -n your-namespace
   ```

## Database Migrations

Migrations are run automatically at application startup, but you can run them separately:

```bash
docker-compose run --rm app ./backend migrate
```

To revert the last migration:

```bash
docker-compose run --rm app ./backend migrate down
```

## Monitoring

The application exposes health and metrics endpoints:

- Health check: `GET /health`
- Metrics (Prometheus format): `GET /metrics`

## Troubleshooting

### Common Issues

1. **Database Connection Errors:**
   - Verify DATABASE_URL is correct
   - Check if PostgreSQL is running
   - Ensure the database exists and is accessible

2. **Permission Issues:**
   - Check file permissions for uploads directory
   - Ensure Docker has proper access to mounted volumes

3. **Application Won't Start:**
   - Check logs with `docker-compose logs app`
   - Verify all required environment variables are set

### Viewing Logs

```bash
# View application logs
docker-compose logs -f app

# View database logs
docker-compose logs -f db
```

For more assistance, please contact the development team.
