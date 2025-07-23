# Monolith VPS Configuration Guide

This document provides guidance for configuring the SaaS UMKM platform on the specific VPS environment.

## System Specifications

- **Hostname**: srv903479.hstgr.cloud (31.97.109.198)
- **Operating System**: Ubuntu Linux 24.04.2
- **CPU**: AMD EPYC 9354P 32-Core Processor (limited to 2 cores)
- **Memory**: 7.75 GiB total
- **System Uptime**: 2 days, 0 hours, 40 minutes (as of July 22, 2025)
- **Load Averages**: 0.14 (1 min) 0.11 (5 mins) 0.08 (15 mins)

## Optimized Configuration

### Resource Allocation

| Component           | Memory Allocation | CPU Allocation |
| ------------------- | ----------------- | -------------- |
| Backend Application | 3.5 GiB max       | 1 core         |
| PostgreSQL          | 2 GiB max         | 0.5 cores      |
| Monitoring Stack    | 1 GiB max         | 0.5 cores      |
| OS & Other Services | 1.25 GiB          | remaining      |

### PostgreSQL Optimization

```conf
# postgresql.conf optimizations for 8GB VPS with limited cores
shared_buffers = 1GB                  # 25% of available RAM
effective_cache_size = 3GB            # ~50% of available RAM (estimate)
maintenance_work_mem = 256MB          # for maintenance operations
work_mem = 32MB                       # per operation memory

# Query tuning
random_page_cost = 1.1                # assumes SSD storage
effective_io_concurrency = 200        # SSD disks

# Parallelism settings - limited due to 2 cores
max_worker_processes = 2
max_parallel_workers_per_gather = 1
max_parallel_workers = 2

# WAL and checkpoints
wal_buffers = 16MB
checkpoint_timeout = 15min
checkpoint_completion_target = 0.9

# Connection settings - conservative due to memory constraints
max_connections = 50
```

### Backend Application Settings

Update the configuration in `.env` file or environment variables:

```
# Runtime settings for Rust backend
RUST_MIN_THREADS=4
RUST_MAX_THREADS=8
RUST_LOG=info
RUST_BACKTRACE=0
RUST_LOG_SPAN_EVENTS=never

# Memory optimizations
MALLOC_ARENA_MAX=2
MALLOC_TRIM_THRESHOLD_=65536
```

### Monitoring Configuration

The monitoring stack has been optimized for low resource usage:

- **Prometheus** scrape interval increased to 30 seconds
- **Prometheus** retention period reduced to 7 days
- **Metrics filtering** applied to store only essential metrics
- **Resource limits** applied to containers

## Deployment Instructions

1. Clone the repository to your server
2. Run the VPS setup script:

```bash
chmod +x ./scripts/setup-monitoring-vps.sh
sudo ./scripts/setup-monitoring-vps.sh
```

3. Deploy the application:

```bash
docker-compose -f docker-compose.vps.yml up -d
```

## Performance Monitoring

Monitor system performance regularly using:

```bash
htop
docker stats
```

## Alert Thresholds

The alert thresholds have been adjusted for your VPS:

- **CPU Usage**: Alert when >75% for 10 minutes
- **Memory Usage**: Alert when >4GB for 10 minutes
- **Response Time**: Alert when >1s for 5 minutes
- **Database Queries**: Alert when >500ms for 5 minutes
