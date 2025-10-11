# Docker Integration Guide

This guide explains how to use magneto-serge with Docker for containerized testing environments.

## Quick Start

### Pull the Image

```bash
docker pull ghcr.io/taciclei/magneto-serge:latest
```

### Basic Usage

```bash
# Record mode
docker run -v $(pwd)/cassettes:/cassettes \
  -p 8888:8888 \
  ghcr.io/taciclei/magneto-serge:latest \
  record my-test

# Replay mode
docker run -v $(pwd)/cassettes:/cassettes \
  -p 8888:8888 \
  ghcr.io/taciclei/magneto-serge:latest \
  replay my-test

# Auto mode (record if missing, else replay)
docker run -v $(pwd)/cassettes:/cassettes \
  -p 8888:8888 \
  ghcr.io/taciclei/magneto-serge:latest \
  auto my-test
```

## Building the Image

### From Source

```bash
# Clone the repository
git clone https://github.com/taciclei/magneto-serge.git
cd magneto-serge

# Build the image
docker build -t magneto-serge:local .

# Run it
docker run -p 8888:8888 magneto-serge:local --help
```

### Multi-Platform Build

```bash
# Build for multiple architectures
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  -t magneto-serge:multi \
  .
```

## Docker Compose

### Basic Setup

Create a `docker-compose.yml`:

```yaml
version: '3.8'

services:
  magneto-proxy:
    image: ghcr.io/taciclei/magneto-serge:latest
    command: replay my-cassette
    ports:
      - "8888:8888"
    volumes:
      - ./cassettes:/cassettes
    environment:
      - RUST_LOG=info
    healthcheck:
      test: ["CMD", "pidof", "magneto"]
      interval: 10s
      timeout: 3s
      retries: 3

  app-under-test:
    image: your-app:latest
    depends_on:
      magneto-proxy:
        condition: service_healthy
    environment:
      - HTTP_PROXY=http://magneto-proxy:8888
      - HTTPS_PROXY=http://magneto-proxy:8888
    networks:
      - test-network

networks:
  test-network:
    driver: bridge
```

Run with:

```bash
docker-compose up
```

### Testing Workflow

```yaml
version: '3.8'

services:
  # Magneto proxy for recording/replaying
  magneto:
    image: ghcr.io/taciclei/magneto-serge:latest
    command: ${MAGNETO_MODE:-auto} ${CASSETTE_NAME:-test}
    ports:
      - "8888:8888"
    volumes:
      - ./cassettes:/cassettes
    environment:
      - RUST_LOG=${RUST_LOG:-info}

  # Your application
  app:
    build: .
    depends_on:
      - magneto
    environment:
      - HTTP_PROXY=http://magneto:8888
      - HTTPS_PROXY=http://magneto:8888
      - NO_PROXY=localhost,127.0.0.1
    command: npm test

  # Run tests
  tests:
    build:
      context: .
      dockerfile: Dockerfile.test
    depends_on:
      - magneto
      - app
    environment:
      - API_URL=http://app:3000
      - HTTP_PROXY=http://magneto:8888
    command: pytest tests/
```

Usage:

```bash
# Record mode
MAGNETO_MODE=record CASSETTE_NAME=integration-test docker-compose up

# Replay mode (for CI)
MAGNETO_MODE=replay CASSETTE_NAME=integration-test docker-compose up

# Auto mode (development)
docker-compose up
```

## Transparent Proxy Setup

For capturing **all** network traffic without explicit proxy configuration.

### Using iptables (Requires Privileges)

Create a `docker-entrypoint.sh`:

```bash
#!/bin/bash
set -e

# Start magneto in background
magneto replay "$CASSETTE_NAME" --port 8888 &
MAGNETO_PID=$!

# Wait for magneto to start
sleep 2

# Configure iptables to redirect traffic
iptables -t nat -A OUTPUT -p tcp --dport 80 -j REDIRECT --to-port 8888
iptables -t nat -A OUTPUT -p tcp --dport 443 -j REDIRECT --to-port 8888

# Execute the main application
exec "$@"

# Cleanup on exit
trap "kill $MAGNETO_PID" EXIT
```

Dockerfile:

```dockerfile
FROM ghcr.io/taciclei/magneto-serge:latest AS magneto

FROM your-app:latest

# Install iptables
RUN apt-get update && apt-get install -y iptables

# Copy magneto binary
COPY --from=magneto /usr/local/bin/magneto /usr/local/bin/

# Copy entrypoint
COPY docker-entrypoint.sh /
RUN chmod +x /docker-entrypoint.sh

ENTRYPOINT ["/docker-entrypoint.sh"]
CMD ["your-app"]
```

Run with elevated privileges:

```bash
docker run --cap-add=NET_ADMIN \
  -e CASSETTE_NAME=my-test \
  -v $(pwd)/cassettes:/cassettes \
  your-app-with-magneto
```

### Sidecar Pattern (Kubernetes)

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: app-with-magneto
spec:
  containers:
  # Main application
  - name: app
    image: your-app:latest
    env:
    - name: HTTP_PROXY
      value: "http://localhost:8888"
    - name: HTTPS_PROXY
      value: "http://localhost:8888"

  # Magneto sidecar
  - name: magneto
    image: ghcr.io/taciclei/magneto-serge:latest
    args: ["replay", "my-cassette"]
    ports:
    - containerPort: 8888
    volumeMounts:
    - name: cassettes
      mountPath: /cassettes

  volumes:
  - name: cassettes
    persistentVolumeClaim:
      claimName: cassettes-pvc
```

## Environment Variables

magneto-serge supports the following environment variables in Docker:

| Variable | Description | Default |
|----------|-------------|---------|
| `CASSETTE_DIR` | Directory for cassette storage | `/cassettes` |
| `MAGNETO_PORT` | Proxy port | `8888` |
| `RUST_LOG` | Logging level | `info` |
| `MAGNETO_MODE` | Proxy mode (auto/record/replay) | - |
| `CASSETTE_NAME` | Cassette name | - |

Example:

```bash
docker run \
  -e CASSETTE_DIR=/data \
  -e MAGNETO_PORT=9000 \
  -e RUST_LOG=debug \
  -v $(pwd)/data:/data \
  -p 9000:9000 \
  ghcr.io/taciclei/magneto-serge:latest \
  replay my-test
```

## CI/CD Integration

### GitHub Actions

```yaml
name: Integration Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest

    services:
      magneto:
        image: ghcr.io/taciclei/magneto-serge:latest
        ports:
          - 8888:8888
        volumes:
          - ${{ github.workspace }}/cassettes:/cassettes
        options: >-
          --health-cmd "pidof magneto"
          --health-interval 10s
          --health-timeout 3s
          --health-retries 3
        env:
          RUST_LOG: info

    steps:
      - uses: actions/checkout@v3

      - name: Run tests with proxy
        run: |
          export HTTP_PROXY=http://localhost:8888
          export HTTPS_PROXY=http://localhost:8888
          npm test
```

### GitLab CI

```yaml
test:
  image: node:18
  services:
    - name: ghcr.io/taciclei/magneto-serge:latest
      alias: magneto
      command: ["replay", "ci-test"]
  variables:
    HTTP_PROXY: "http://magneto:8888"
    HTTPS_PROXY: "http://magneto:8888"
  before_script:
    - mkdir -p cassettes
  script:
    - npm test
  artifacts:
    paths:
      - cassettes/
```

## Advanced Use Cases

### 1. Multi-Stage Testing

```yaml
version: '3.8'

services:
  # Record phase
  record:
    image: ghcr.io/taciclei/magneto-serge:latest
    command: record integration-test
    profiles: ["record"]
    volumes:
      - ./cassettes:/cassettes
    ports:
      - "8888:8888"

  # Replay phase
  replay:
    image: ghcr.io/taciclei/magneto-serge:latest
    command: replay integration-test
    profiles: ["replay"]
    volumes:
      - ./cassettes:/cassettes:ro  # Read-only in CI
    ports:
      - "8888:8888"

  app:
    image: your-app:latest
    environment:
      - HTTP_PROXY=http://${MAGNETO_SERVICE:-replay}:8888
```

Usage:

```bash
# Recording
docker-compose --profile record up

# Replaying (CI)
MAGNETO_SERVICE=replay docker-compose --profile replay up
```

### 2. Per-Test Cassettes

```dockerfile
FROM ghcr.io/taciclei/magneto-serge:latest

# Copy test cassettes
COPY test-cassettes/ /cassettes/

# Create wrapper script
COPY <<EOF /usr/local/bin/test-proxy
#!/bin/bash
CASSETTE_NAME=\${TEST_NAME:-default}
exec magneto replay "\$CASSETTE_NAME" --port 8888
EOF

RUN chmod +x /usr/local/bin/test-proxy

ENTRYPOINT ["test-proxy"]
```

### 3. Development with Hot Reload

```yaml
version: '3.8'

services:
  magneto-dev:
    image: ghcr.io/taciclei/magneto-serge:latest
    command: auto dev-session
    ports:
      - "8888:8888"
    volumes:
      - ./cassettes:/cassettes
      - ./magneto-dev.log:/var/log/magneto.log
    environment:
      - RUST_LOG=debug
    restart: unless-stopped

  app-dev:
    build:
      context: .
      target: development
    volumes:
      - .:/app
      - /app/node_modules
    environment:
      - HTTP_PROXY=http://magneto-dev:8888
      - NODE_ENV=development
    command: npm run dev
```

## TLS/HTTPS Support

### Installing the CA Certificate

magneto-serge generates a self-signed CA certificate for MITM HTTPS interception.

1. **Extract the certificate from the container:**

```bash
docker run --rm \
  -v $(pwd)/certs:/certs \
  ghcr.io/taciclei/magneto-serge:latest \
  sh -c "cp /root/.magneto/magneto-ca.pem /certs/"
```

2. **Install in your application container:**

```dockerfile
FROM your-app:latest

# Copy magneto CA certificate
COPY certs/magneto-ca.pem /usr/local/share/ca-certificates/magneto-ca.crt

# Update CA certificates
RUN update-ca-certificates
```

3. **For Node.js applications:**

```dockerfile
ENV NODE_EXTRA_CA_CERTS=/usr/local/share/ca-certificates/magneto-ca.crt
```

4. **For Python applications:**

```dockerfile
ENV REQUESTS_CA_BUNDLE=/usr/local/share/ca-certificates/magneto-ca.crt
```

## Troubleshooting

### Proxy Not Responding

Check if magneto is running:

```bash
docker-compose exec magneto pidof magneto
```

Check logs:

```bash
docker-compose logs magneto
```

### Certificate Errors

Ensure the CA certificate is properly installed:

```bash
# In the app container
update-ca-certificates --verbose
```

### iptables Not Working

Verify NET_ADMIN capability:

```bash
docker run --cap-add=NET_ADMIN your-image \
  sh -c "iptables -L -t nat"
```

### Network Isolation Issues

Check network connectivity:

```bash
docker-compose exec app ping magneto
docker-compose exec app curl -v -x http://magneto:8888 https://httpbin.org/get
```

## Performance Considerations

### Image Size

Current image size: ~150MB (with Debian slim base)

Optimize further with Alpine:

```dockerfile
FROM rust:1.75-alpine AS builder
# ... build steps ...

FROM alpine:latest
RUN apk add --no-cache ca-certificates libgcc
# ... runtime setup ...
```

### Memory Usage

magneto-serge typically uses <50MB of RAM. Limit in Docker:

```yaml
services:
  magneto:
    image: ghcr.io/taciclei/magneto-serge:latest
    deploy:
      resources:
        limits:
          memory: 128M
```

### Network Overhead

Proxy adds <1ms latency in replay mode. Use `LatencyMode::None` for instant responses.

## Examples

See the [`examples/docker/`](../../examples/docker/) directory for complete examples:

- `examples/docker/basic/` - Simple Docker setup
- `examples/docker/compose/` - Docker Compose integration
- `examples/docker/transparent/` - Transparent proxy with iptables
- `examples/docker/kubernetes/` - Kubernetes sidecar pattern
- `examples/docker/ci/` - CI/CD integration examples

## Related Documentation

- [ERROR_RECORDING.md](ERROR_RECORDING.md) - Recording network errors
- [LATENCY_SIMULATION.md](LATENCY_SIMULATION.md) - Simulating network delays
- [FILTERS.md](FILTERS.md) - Filtering sensitive data
- [ARCHITECTURE.md](ARCHITECTURE.md) - System architecture

## Contributing

Have ideas for improving Docker integration? Open an issue or PR on GitHub!

---

**Last updated:** 2025-10-11
