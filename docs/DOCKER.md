# Docker Integration Guide

This guide explains how to use magneto-serge with Docker for transparent network interception and testing.

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Official Docker Image](#official-docker-image)
- [Transparent Proxy Setup](#transparent-proxy-setup)
- [Docker Compose Examples](#docker-compose-examples)
- [docker-vcr Project](#docker-vcr-project)
- [Configuration](#configuration)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)

---

## Overview

magneto-serge can be used in Docker environments in two main ways:

1. **Explicit Proxy** (Simple): Configure your app to use magneto-serge as HTTP/HTTPS proxy
2. **Transparent Proxy** (Advanced): Use iptables to intercept all traffic without app modification

### Use Cases

- **CI/CD Pipelines**: Deterministic tests with recorded cassettes
- **Integration Testing**: Isolated network environment
- **Development**: Consistent API responses across team
- **Zero Code Change**: Test existing apps without proxy configuration

---

## Quick Start

### Pull Official Image

```bash
docker pull ghcr.io/taciclei/magneto-serge:latest
```

### Run in Replay Mode

```bash
docker run -d \
  --name magneto-proxy \
  -p 8888:8888 \
  -v $(pwd)/cassettes:/cassettes \
  ghcr.io/taciclei/magneto-serge:latest \
  replay my-cassette --port 8888
```

### Record a New Cassette

```bash
docker run -d \
  --name magneto-recorder \
  -p 8888:8888 \
  -v $(pwd)/cassettes:/cassettes \
  ghcr.io/taciclei/magneto-serge:latest \
  record new-cassette --port 8888
```

### Auto Mode (Record if missing, else replay)

```bash
docker run -d \
  --name magneto-auto \
  -p 8888:8888 \
  -v $(pwd)/cassettes:/cassettes \
  ghcr.io/taciclei/magneto-serge:latest \
  auto api-cassette --port 8888
```

---

## Official Docker Image

The official image is available at `ghcr.io/taciclei/magneto-serge`.

### Image Tags

- `latest`: Latest stable release
- `v0.1.0`: Specific version
- `develop`: Development branch (unstable)

### What's Included

- magneto-serge CLI (`/usr/local/bin/magneto`)
- Pre-installed CA certificate utilities
- iptables for transparent proxy support
- Health check endpoint at `/health`
- Cassette directory at `/cassettes`

### Build Your Own Image

```bash
git clone https://github.com/taciclei/magneto-serge.git
cd magneto-serge
docker build -t magneto-serge:custom .
```

---

## Transparent Proxy Setup

Transparent proxy intercepts all HTTP/HTTPS traffic without requiring app configuration.

### Architecture

```
┌──────────────────────────────────────┐
│ Docker Container                      │
│                                       │
│  ┌────────────────┐                  │
│  │ App Under Test │                  │
│  └────────┬───────┘                  │
│           │                           │
│           ↓ (iptables redirect)      │
│  ┌────────────────┐                  │
│  │ magneto-serge  │ ← Transparent    │
│  │ (port 8888)    │    interception  │
│  └────────┬───────┘                  │
│           │                           │
└───────────┼───────────────────────────┘
            ↓
      Real Network (or replay)
```

### Dockerfile Example

```dockerfile
FROM ghcr.io/taciclei/magneto-serge:latest

# Copy your application
COPY ./my-app /app
WORKDIR /app

# Copy cassettes
COPY ./cassettes /cassettes

# Custom entrypoint with iptables
COPY docker-entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/docker-entrypoint.sh

ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]
CMD ["python", "app.py"]
```

### Entrypoint Script

```bash
#!/bin/bash
# docker-entrypoint.sh

set -e

# Start magneto-serge in background
magneto ${MAGNETO_MODE:-replay} ${CASSETTE_NAME:-default} \
  --port ${MAGNETO_PORT:-8888} \
  --cassette-dir /cassettes &

MAGNETO_PID=$!

# Wait for magneto to be ready
sleep 2

# Configure iptables for transparent proxying
# Redirect HTTP traffic
iptables -t nat -A OUTPUT -p tcp --dport 80 -j REDIRECT --to-port 8888

# Redirect HTTPS traffic
iptables -t nat -A OUTPUT -p tcp --dport 443 -j REDIRECT --to-port 8888

# Trust magneto CA certificate
if [ -f /cassettes/magneto-ca.pem ]; then
  cp /cassettes/magneto-ca.pem /usr/local/share/ca-certificates/magneto-ca.crt
  update-ca-certificates
fi

# Cleanup on exit
trap "kill $MAGNETO_PID" EXIT

# Run the application
exec "$@"
```

### Running with Elevated Privileges

Transparent proxy requires `NET_ADMIN` capability:

```bash
docker run --cap-add=NET_ADMIN \
  -v $(pwd)/cassettes:/cassettes \
  -e MAGNETO_MODE=replay \
  -e CASSETTE_NAME=api-cassette \
  my-app-with-magneto:latest
```

---

## Docker Compose Examples

### Example 1: Explicit Proxy (Simple)

```yaml
version: '3.8'

services:
  magneto-proxy:
    image: ghcr.io/taciclei/magneto-serge:latest
    command: replay api-cassette --port 8888
    ports:
      - "8888:8888"
    volumes:
      - ./cassettes:/cassettes
    networks:
      - test-network
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8888/health"]
      interval: 10s
      timeout: 5s
      retries: 3

  app-under-test:
    image: my-app:latest
    depends_on:
      magneto-proxy:
        condition: service_healthy
    environment:
      HTTP_PROXY: http://magneto-proxy:8888
      HTTPS_PROXY: http://magneto-proxy:8888
      NO_PROXY: localhost,127.0.0.1
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

### Example 2: Transparent Proxy (Advanced)

```yaml
version: '3.8'

services:
  app-with-transparent-proxy:
    build:
      context: .
      dockerfile: Dockerfile.transparent
    cap_add:
      - NET_ADMIN
    volumes:
      - ./cassettes:/cassettes
    environment:
      MAGNETO_MODE: replay
      CASSETTE_NAME: integration-test
      MAGNETO_PORT: 8888
```

### Example 3: Multi-Container Test Suite

```yaml
version: '3.8'

services:
  magneto-proxy:
    image: ghcr.io/taciclei/magneto-serge:latest
    command: auto ${CASSETTE_NAME:-default} --port 8888
    volumes:
      - ./cassettes:/cassettes
    networks:
      - test-network

  backend-api:
    image: my-backend:latest
    depends_on:
      - magneto-proxy
    environment:
      HTTP_PROXY: http://magneto-proxy:8888
      HTTPS_PROXY: http://magneto-proxy:8888
    networks:
      - test-network

  frontend-app:
    image: my-frontend:latest
    depends_on:
      - backend-api
    environment:
      API_URL: http://backend-api:3000
    networks:
      - test-network

  test-runner:
    image: cypress:latest
    depends_on:
      - frontend-app
    volumes:
      - ./e2e-tests:/tests
    command: cypress run
    networks:
      - test-network

networks:
  test-network:
    driver: bridge
```

---

## docker-vcr Project

For advanced use cases requiring transparent proxy with zero code changes, see the companion project **docker-vcr** (by @1000i100):

🔗 [1forma-tic/1vcr](https://framagit.org/1forma-tic/1vcr)

### What is docker-vcr?

docker-vcr is a wrapper around magneto-serge that provides:

- **Automatic network interception** via iptables
- **Zero-configuration setup** for existing Docker apps
- **Pre-built Docker images** for common frameworks
- **Docker Compose templates** for popular stacks
- **CA certificate auto-installation**

### When to Use docker-vcr

Use docker-vcr when:

- You cannot modify app code to add proxy configuration
- You need to capture **all** network traffic (including DNS)
- You want turnkey Docker integration
- You're testing legacy applications

Use magneto-serge directly when:

- You can configure your app to use a proxy
- You want more control over interception
- You're building custom integrations
- You need multi-language bindings (Python, Java, JS, etc.)

---

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `MAGNETO_MODE` | Proxy mode: `auto`, `record`, `replay`, `passthrough` | `auto` |
| `MAGNETO_PORT` | Proxy port | `8888` |
| `CASSETTE_NAME` | Cassette name to record/replay | `default` |
| `CASSETTE_DIR` | Cassette storage directory | `/cassettes` |
| `RUST_LOG` | Logging level: `error`, `warn`, `info`, `debug`, `trace` | `info` |

### Example with Environment Variables

```bash
docker run -d \
  -e MAGNETO_MODE=replay \
  -e MAGNETO_PORT=9999 \
  -e CASSETTE_NAME=my-api \
  -e RUST_LOG=debug \
  -v $(pwd)/cassettes:/cassettes \
  -p 9999:9999 \
  ghcr.io/taciclei/magneto-serge:latest
```

### Volume Mounts

Mount cassettes directory for persistence:

```bash
docker run -v $(pwd)/cassettes:/cassettes ghcr.io/taciclei/magneto-serge:latest
```

Mount CA certificate for HTTPS:

```bash
docker run -v $(pwd)/magneto-ca.pem:/usr/local/share/ca-certificates/magneto-ca.crt ghcr.io/taciclei/magneto-serge:latest
```

---

## Best Practices

### 1. Use Health Checks

Always define health checks for reliable startup:

```yaml
healthcheck:
  test: ["CMD", "curl", "-f", "http://localhost:8888/health"]
  interval: 10s
  timeout: 5s
  retries: 3
```

### 2. Separate Record and Replay Environments

**CI Pipeline**: Use `replay` mode with strict matching

```yaml
# .github/workflows/test.yml
- name: Run tests with cassettes
  run: |
    docker-compose -f docker-compose.test.yml up --abort-on-container-exit
  env:
    MAGNETO_MODE: replay
```

**Development**: Use `auto` mode for flexibility

```bash
MAGNETO_MODE=auto docker-compose up
```

### 3. Gitignore Large Cassettes

```gitignore
# .gitignore
cassettes/*.json
!cassettes/critical-*.json
```

Commit only essential cassettes, use CI to generate others.

### 4. Use Multi-Stage Builds

```dockerfile
FROM ghcr.io/taciclei/magneto-serge:latest AS magneto

FROM node:18
COPY --from=magneto /usr/local/bin/magneto /usr/local/bin/
COPY . /app
WORKDIR /app
RUN npm install
CMD ["npm", "test"]
```

### 5. Network Isolation

Use dedicated Docker networks for test isolation:

```yaml
networks:
  test-network:
    driver: bridge
    internal: true  # No external access
```

---

## Troubleshooting

### Issue: HTTPS not working (certificate errors)

**Cause**: magneto CA certificate not trusted

**Solution**: Install CA certificate in container

```dockerfile
COPY magneto-ca.pem /usr/local/share/ca-certificates/magneto-ca.crt
RUN update-ca-certificates
```

For Node.js apps:

```bash
export NODE_EXTRA_CA_CERTS=/usr/local/share/ca-certificates/magneto-ca.crt
```

For Python apps:

```bash
export REQUESTS_CA_BUNDLE=/usr/local/share/ca-certificates/magneto-ca.crt
```

### Issue: "Connection refused" errors

**Cause**: magneto-serge not started before app

**Solution**: Use health checks and `depends_on`

```yaml
depends_on:
  magneto-proxy:
    condition: service_healthy
```

### Issue: iptables rules not working

**Cause**: Missing `NET_ADMIN` capability

**Solution**: Add capability

```bash
docker run --cap-add=NET_ADMIN ...
```

For docker-compose:

```yaml
cap_add:
  - NET_ADMIN
```

### Issue: DNS resolution failing

**Cause**: DNS traffic not redirected to magneto

**Solution**: Use explicit DNS configuration

```yaml
dns:
  - 8.8.8.8
  - 8.8.4.4
```

Or use `dnsmasq` with iptables redirection.

### Issue: WebSocket connections failing

**Cause**: WebSocket upgrade not proxied correctly

**Solution**: Ensure magneto-serge is up to date (WebSocket support added in v0.1.0)

```bash
docker pull ghcr.io/taciclei/magneto-serge:latest
```

### Issue: High memory usage with large cassettes

**Cause**: Loading entire cassette into memory

**Solution**: Use streaming mode (if available) or split cassettes

```bash
# Split large cassette
magneto split large-cassette --output-dir ./cassettes/split/
```

---

## Performance Considerations

### Latency

- **Explicit proxy**: ~1ms overhead
- **Transparent proxy**: ~2-3ms overhead (iptables)

### Throughput

- HTTP: ≥5000 req/s per container
- WebSocket: ≥10k msg/s per container

### Memory

- Base image: ~50 MB
- Per cassette: ~1-10 MB depending on size
- Recommended limit: 512 MB

### CPU

- Minimal CPU usage (<5% for typical loads)
- Scales linearly with request volume

---

## CI/CD Integration Examples

### GitHub Actions

```yaml
name: Integration Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Pull magneto-serge image
        run: docker pull ghcr.io/taciclei/magneto-serge:latest

      - name: Run tests with cassettes
        run: |
          docker-compose -f docker-compose.test.yml up --abort-on-container-exit
        env:
          MAGNETO_MODE: replay
          CASSETTE_NAME: ci-integration-test

      - name: Check test results
        run: docker-compose logs test-runner
```

### GitLab CI

```yaml
test:
  image: docker:latest
  services:
    - docker:dind
  script:
    - docker pull ghcr.io/taciclei/magneto-serge:latest
    - docker-compose -f docker-compose.test.yml up --abort-on-container-exit
  variables:
    MAGNETO_MODE: replay
    CASSETTE_NAME: ci-integration-test
```

---

## Related Documentation

- [Architecture](./ARCHITECTURE.md) - Component design
- [Examples](./EXAMPLES.md) - Multi-language usage examples
- [Latency Simulation](./LATENCY_SIMULATION.md) - Timing control
- [docker-vcr project](https://framagit.org/1forma-tic/1vcr) - Transparent proxy wrapper

---

**Last updated**: 2025-10-11
**Version**: 0.1.0
