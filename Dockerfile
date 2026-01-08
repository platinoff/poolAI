# Multi-stage Dockerfile for PoolAI
# Stage 1: Build
FROM rust:1.87-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /build

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY build.rs ./

# Build the application
# Note: For production, you may want to enable features like "jwt" and "https"
# Example: RUN cargo build --release --features jwt,https
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1000 poolai

# Set working directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /build/target/release/poolai /app/poolai

# Copy configuration template
COPY config.example.toml /app/config.example.toml

# Create data and config directories
RUN mkdir -p /data /config && \
    chown -R poolai:poolai /app /data /config

# Switch to app user
USER poolai

# Expose ports
EXPOSE 8080 8443

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/api/v1/health || exit 1

# Set environment variables
ENV RUST_LOG=info
ENV POOLAI_CONFIG_PATH=/config/config.toml
ENV POOLAI_DATA_PATH=/data

# Run the application
CMD ["/app/poolai"]
