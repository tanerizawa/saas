FROM rust:1.73-slim as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    nodejs \
    npm \
    && rm -rf /var/lib/apt/lists/*

# Build backend
COPY backend/Cargo.toml backend/Cargo.lock ./
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -f target/release/deps/saas_umkm_backend*

# Copy actual source code and build
COPY backend/src ./src
COPY backend/migrations ./migrations
RUN cargo build --release

# Build frontend
WORKDIR /frontend
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci

COPY frontend ./
RUN npm run build

# Final stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    nodejs \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy backend binary and migrations
COPY --from=builder /app/target/release/saas-umkm-backend ./
COPY --from=builder /app/migrations ./migrations

# Copy frontend build
COPY --from=builder /frontend/.next ./.next
COPY --from=builder /frontend/public ./public
COPY --from=builder /frontend/next.config.ts ./
COPY --from=builder /frontend/package.json ./

# Create uploads directory
RUN mkdir -p uploads && chmod 755 uploads

# Copy startup script
COPY infrastructure/deployment/start.sh ./
RUN chmod +x start.sh

EXPOSE 3000 8000

CMD ["./start.sh"]
