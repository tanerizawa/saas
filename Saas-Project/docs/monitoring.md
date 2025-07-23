# Monitoring Setup for SaaS UMKM Platform

## Overview

This document describes the monitoring setup for the SaaS UMKM platform's monolithic architecture. It outlines how metrics are collected, visualized, and alerted on.

## Architecture

The monitoring stack consists of the following components:

- **Prometheus**: Time-series database for metrics collection
- **Grafana**: Visualization and dashboards
- **Alertmanager**: Alert handling and notifications

### Metrics Collection

The monolithic application exposes metrics through a `/metrics` endpoint that Prometheus scrapes. Key metrics include:

1. **HTTP Layer Metrics**

   - Request count by endpoint and status code
   - Request duration by endpoint
   - Error rates

2. **Domain-Specific Metrics**
   - License operations (creation, updates, verifications)
   - Company operations (registrations, updates, document submissions)
   - User authentication (logins, registrations, password resets)
3. **Database Metrics**

   - Query durations
   - Connection pool usage
   - Transaction rates and errors

4. **System Metrics**
   - CPU usage
   - Memory usage
   - Disk I/O
   - Network traffic

## Deployment Options

### Local Development

For local development, use the Docker Compose configuration in `infrastructure/monitoring-monolith/docker-compose.monitoring.yml`:

```bash
docker-compose -f infrastructure/monitoring-monolith/docker-compose.monitoring.yml up -d
```

Access Grafana at http://localhost:3000 (admin/admin)

### Production Environment

For production deployment, use the Kubernetes manifests in the `infrastructure/monitoring-monolith/` directory:

```bash
kubectl create namespace monitoring
kubectl apply -f infrastructure/monitoring-monolith/
```

## Dashboard Overview

The monitoring setup includes pre-configured dashboards for:

1. **API Performance Dashboard**

   - Request rates and latencies
   - Error rates
   - Top endpoints by traffic

2. **Domain Operations Dashboard**

   - License operations by type
   - Company registrations and updates
   - Document processing metrics

3. **Database Performance Dashboard**

   - Query durations by type
   - Connection pool usage
   - Transaction rates

4. **System Resource Dashboard**
   - CPU/Memory usage
   - Disk I/O
   - Network traffic

## Alert Rules

Key alerts configured in the system:

1. **High Response Time**

   - Triggers when 95th percentile response time exceeds 500ms for 5 minutes

2. **High Error Rate**

   - Triggers when error rate exceeds 5% for 5 minutes

3. **Slow Database Queries**

   - Triggers when 95th percentile database query time exceeds 200ms for 5 minutes

4. **Instance Down**

   - Triggers when application instance is not responding

5. **High Memory Usage**
   - Triggers when memory usage exceeds 6GB (optimized for 8GB VPS)

## Integration with Application

The application has been instrumented with metrics collection using the Prometheus client library. Key integration points:

1. HTTP middleware for request metrics
2. Database query timing wrappers
3. Domain-specific operation counters
4. Custom business metrics for license processing

## Maintenance and Scaling

The monitoring stack is designed to be lightweight and optimized for an 8GB RAM VPS:

- Prometheus retention set to 15 days
- Grafana uses minimal resources
- Alert rules are focused on critical path performance
