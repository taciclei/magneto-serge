# Build stage
FROM rust:1.75-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY build.rs ./

# Build the application
RUN cargo build --release --features cli --bin matgto

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies (including iptables for transparent proxy)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    iptables \
    curl \
    net-tools \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/magneto /usr/local/bin/magneto

# Create cassette directory
RUN mkdir -p /app/cassettes

# Set environment variables
ENV CASSETTE_DIR=/app/cassettes
ENV RUST_LOG=info

# Expose default proxy port
EXPOSE 8888

# Health check (magneto doesn't have /health endpoint yet, so we check if port is listening)
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD netstat -an | grep 8888 || exit 1

# Set the entrypoint
ENTRYPOINT ["magneto"]
CMD ["--help"]
