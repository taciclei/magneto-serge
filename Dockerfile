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

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/matgto /usr/local/bin/matgto

# Create cassette directory
RUN mkdir -p /app/cassettes

# Set environment variables
ENV CASSETTE_DIR=/app/cassettes
ENV RUST_LOG=info

# Expose default proxy port
EXPOSE 8888

# Set the entrypoint
ENTRYPOINT ["matgto"]
CMD ["--help"]
