# Dockerfile for Magnéto-Serge
# Multi-stage build for optimal image size

# Stage 1: Builder
FROM rust:1.75-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY build.rs ./

# Build release binary with CLI features
RUN cargo build --release --features cli

# Stage 2: Runtime
FROM debian:bookworm-slim

WORKDIR /cassettes

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/magneto /usr/local/bin/magneto

# Create cassettes directory
RUN mkdir -p /cassettes

# Set environment variables
ENV CASSETTE_DIR=/cassettes
ENV RUST_LOG=info
ENV MAGNETO_PORT=8888

# Expose default proxy port
EXPOSE 8888

# Health check
HEALTHCHECK --interval=10s --timeout=3s --start-period=5s --retries=3 \
    CMD pidof magneto || exit 1

# Set the entrypoint
ENTRYPOINT ["magneto"]
CMD ["--help"]
