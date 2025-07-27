# Multi-stage Docker build for Rust backend
# Optimized for development and production as recommended in document

# Build stage
FROM rust:1.75-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev pkgconfig openssl-dev

# Create app directory
WORKDIR /app

# Copy manifest files and toolchain info
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./

# Create empty source files to trigger dependency build for better layer caching
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release

# Copy source code
COPY src ./src
COPY migrations ./migrations

# Rebuild with actual source code
RUN touch src/main.rs && \
    cargo build --release

# Runtime stage
FROM alpine:latest AS runtime

# Install runtime dependencies
RUN apk add --no-cache ca-certificates openssl libgcc

# Create app user for security
RUN addgroup -g 1000 app && adduser -D -s /bin/sh -u 1000 -G app app

# Create app directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/backend /app/
COPY --from=builder /app/migrations /app/migrations

# Create uploads directory
RUN mkdir -p /app/uploads

# Change ownership to app user
RUN chown -R app:app /app

# Switch to app user for security
USER app

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8080/health || exit 1

# Set environment variables
ENV RUST_BACKTRACE=1

# Start the application
CMD ["./backend"]

# Development stage (for development with hot reload)
FROM rust:1.75-alpine AS development

# Install development dependencies
RUN apk add --no-cache musl-dev pkgconfig openssl-dev

# Install cargo-watch for hot reload
RUN cargo install cargo-watch

# Create app directory
WORKDIR /app

# Expose port
EXPOSE 8000

# Default command for development
CMD ["cargo", "watch", "-x", "run"]
