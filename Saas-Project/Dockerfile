# Multi-stage production Docker build for SaaS UMKM Platform
FROM rust:1.73-slim as backend-builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy backend source and build
COPY backend/Cargo.toml backend/Cargo.lock ./
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -f target/release/deps/saas_umkm_backend*

COPY backend/src ./src
COPY backend/migrations ./migrations
RUN cargo build --release

# Frontend builder stage  
FROM node:18-alpine as frontend-builder

WORKDIR /frontend
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci

COPY frontend ./
RUN npm run build

# Production runtime stage
FROM debian:bullseye-slim as production

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    curl \
    nodejs \
    npm \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy backend binary and migrations
COPY --from=backend-builder /app/target/release/server ./backend-server
COPY --from=backend-builder /app/migrations ./migrations

# Copy frontend build
COPY --from=frontend-builder /frontend/.next ./.next
COPY --from=frontend-builder /frontend/public ./public
COPY --from=frontend-builder /frontend/next.config.ts ./
COPY --from=frontend-builder /frontend/package.json ./package.json

# Create required directories
RUN mkdir -p uploads && chmod 755 uploads

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8000/health || exit 1

EXPOSE 3000 8000

# Start both backend and frontend
CMD ["sh", "-c", "./backend-server & npm start"]
