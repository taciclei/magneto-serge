# docker-vcr Guide

This directory contains templates and examples for using magneto-serge as a **transparent network proxy** with Docker, enabling zero-code-change testing.

## What is docker-vcr?

docker-vcr is an approach to encapsulate your application under test with magneto-serge in a Docker container, using iptables to intercept **all** network traffic without modifying your application code.

### Key Benefits

✅ **Zero Code Changes** - No need to configure HTTP_PROXY in your app
✅ **Complete Capture** - All HTTP/HTTPS traffic is intercepted automatically
✅ **Isolation** - Reproducible test environments
✅ **CI/CD Friendly** - Easy integration in pipelines
✅ **Network Discovery** - Captures implicit connections and DNS requests

### When to Use docker-vcr

Use docker-vcr when:
- You **cannot modify** application code to add proxy configuration
- You need to capture **all** network traffic including discovery protocols
- You're testing **legacy applications** without proxy support
- You want **turnkey Docker integration** for existing apps

Use explicit proxy (standard magneto-serge) when:
- You can configure your app to use `HTTP_PROXY`/`HTTPS_PROXY`
- You want simpler setup without elevated privileges
- You're building new applications with testing in mind

---

## Quick Start

### 1. Build the Transparent Proxy Image

```bash
cd /path/to/magneto-serge
docker build -f Dockerfile.transparent -t magneto-transparent:latest .
```

### 2. Run Your App with Transparent Proxy

```bash
docker run --cap-add=NET_ADMIN \
  -v $(pwd)/cassettes:/cassettes \
  -e MAGNETO_MODE=replay \
  -e CASSETTE_NAME=my-cassette \
  -e TRANSPARENT_PROXY=true \
  magneto-transparent:latest \
  your-app-command
```

### 3. Check Results

All HTTP/HTTPS traffic from `your-app-command` will be intercepted and replayed from `cassettes/my-cassette.json`.

---

## Architecture

```
┌────────────────────────────────────────────┐
│ Docker Container (--cap-add=NET_ADMIN)     │
│                                            │
│  ┌──────────────────┐                     │
│  │ Your Application │                     │
│  │ (no modification)│                     │
│  └────────┬─────────┘                     │
│           │                                │
│           │ HTTP/HTTPS request             │
│           ↓                                │
│  ┌──────────────────────────────────────┐ │
│  │ iptables REDIRECT                    │ │
│  │ 80 → 8888, 443 → 8888               │ │
│  └────────┬─────────────────────────────┘ │
│           ↓                                │
│  ┌──────────────────┐                     │
│  │ magneto-serge    │                     │
│  │ (port 8888)      │                     │
│  └────────┬─────────┘                     │
│           │                                │
│           ↓ (replay from cassette)        │
│  ┌──────────────────┐                     │
│  │ Cassette Files   │                     │
│  │ /cassettes/*.json│                     │
│  └──────────────────┘                     │
└────────────────────────────────────────────┘
```

---

## Templates

### Template 1: Wrap Existing Application

Create a custom Dockerfile based on your app:

```dockerfile
# Dockerfile.myapp-with-vcr
FROM magneto-transparent:latest

# Install your application
COPY ./my-app /app
WORKDIR /app

# Install dependencies (example for Node.js)
RUN npm install

# The entrypoint is already configured by magneto-transparent
# Just provide your app's start command as CMD
CMD ["npm", "start"]
```

Build and run:

```bash
docker build -f Dockerfile.myapp-with-vcr -t myapp-vcr:latest .

docker run --cap-add=NET_ADMIN \
  -v $(pwd)/cassettes:/cassettes \
  -e MAGNETO_MODE=replay \
  -e CASSETTE_NAME=integration-test \
  myapp-vcr:latest
```

### Template 2: Docker Compose for Multi-Container Tests

```yaml
# docker-compose.vcr.yml
version: '3.8'

services:
  app-with-vcr:
    build:
      context: .
      dockerfile: Dockerfile.myapp-with-vcr
    cap_add:
      - NET_ADMIN
    volumes:
      - ./cassettes:/cassettes
    environment:
      MAGNETO_MODE: ${MODE:-replay}
      CASSETTE_NAME: ${CASSETTE:-integration-test}
      TRANSPARENT_PROXY: "true"
    networks:
      - vcr-network

  test-runner:
    image: your-test-framework:latest
    depends_on:
      - app-with-vcr
    volumes:
      - ./tests:/tests
    command: ["pytest", "/tests"]
    networks:
      - vcr-network

networks:
  vcr-network:
    driver: bridge
```

Run tests:

```bash
# First time: record cassette
MODE=record CASSETTE=integration-test docker-compose -f docker-compose.vcr.yml up

# Subsequent runs: replay cassette
MODE=replay CASSETTE=integration-test docker-compose -f docker-compose.vcr.yml up
```

### Template 3: GitHub Actions CI/CD

```yaml
# .github/workflows/vcr-test.yml
name: VCR Integration Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Build VCR image
        run: |
          docker build -f Dockerfile.transparent -t magneto-transparent:latest .
          docker build -f Dockerfile.myapp-with-vcr -t myapp-vcr:latest .

      - name: Run tests with cassette replay
        run: |
          docker-compose -f docker-compose.vcr.yml up --abort-on-container-exit
        env:
          MODE: replay
          CASSETTE: ci-integration-test

      - name: Check test results
        run: docker-compose -f docker-compose.vcr.yml logs test-runner
```

---

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `MAGNETO_MODE` | Mode: `auto`, `record`, `replay`, `passthrough` | `replay` |
| `CASSETTE_NAME` | Cassette name to use | `default` |
| `MAGNETO_PORT` | Proxy port | `8888` |
| `CASSETTE_DIR` | Cassette directory | `/cassettes` |
| `TRANSPARENT_PROXY` | Enable iptables redirection | `false` |
| `REDIRECT_PORTS` | Additional ports to redirect (comma-separated) | None |
| `RUST_LOG` | Logging level | `info` |

### Custom Port Redirection

Redirect additional ports beyond 80 and 443:

```bash
docker run --cap-add=NET_ADMIN \
  -e TRANSPARENT_PROXY=true \
  -e REDIRECT_PORTS="8080,3000,5432" \
  magneto-transparent:latest
```

This redirects ports 8080, 3000, and 5432 to magneto.

---

## Real-World Examples

### Example 1: Python Flask App

```dockerfile
# Dockerfile.flask-vcr
FROM magneto-transparent:latest

# Install Python and Flask
RUN apt-get update && apt-get install -y python3 python3-pip

# Copy application
COPY ./flask-app /app
WORKDIR /app

# Install dependencies
RUN pip3 install -r requirements.txt

# Start Flask app (magneto will intercept all requests)
CMD ["python3", "app.py"]
```

```bash
# Record cassette
docker run --cap-add=NET_ADMIN \
  -v $(pwd)/cassettes:/cassettes \
  -e MAGNETO_MODE=record \
  -e CASSETTE_NAME=flask-api-calls \
  -p 5000:5000 \
  flask-vcr:latest

# Test Flask app (makes HTTP requests to external APIs)
curl http://localhost:5000/api/data

# Stop container (cassette saved)

# Replay in CI
docker run --cap-add=NET_ADMIN \
  -v $(pwd)/cassettes:/cassettes \
  -e MAGNETO_MODE=replay \
  -e CASSETTE_NAME=flask-api-calls \
  flask-vcr:latest \
  pytest tests/
```

### Example 2: Node.js Microservice

```dockerfile
# Dockerfile.node-vcr
FROM magneto-transparent:latest

# Install Node.js
RUN apt-get update && apt-get install -y nodejs npm

# Copy application
COPY ./microservice /app
WORKDIR /app

# Install dependencies
RUN npm install

# Start microservice
CMD ["npm", "start"]
```

```yaml
# docker-compose.node-vcr.yml
version: '3.8'

services:
  microservice-vcr:
    build:
      context: .
      dockerfile: Dockerfile.node-vcr
    cap_add:
      - NET_ADMIN
    volumes:
      - ./cassettes:/cassettes
    environment:
      MAGNETO_MODE: replay
      CASSETTE_NAME: microservice-deps
      TRANSPARENT_PROXY: "true"
    ports:
      - "3000:3000"

  integration-tests:
    image: node:18
    depends_on:
      - microservice-vcr
    volumes:
      - ./tests:/tests
    working_dir: /tests
    command: ["npm", "test"]
```

### Example 3: Java Spring Boot Application

```dockerfile
# Dockerfile.spring-vcr
FROM magneto-transparent:latest

# Install Java 17
RUN apt-get update && apt-get install -y openjdk-17-jre-headless

# Copy application JAR
COPY target/myapp.jar /app/app.jar

# Start Spring Boot (magneto intercepts RestTemplate/WebClient calls)
CMD ["java", "-jar", "/app/app.jar"]
```

```bash
# Record cassette with Spring Boot
docker run --cap-add=NET_ADMIN \
  -v $(pwd)/cassettes:/cassettes \
  -v $(pwd)/magneto-ca.pem:/usr/local/share/ca-certificates/magneto-ca.crt \
  -e MAGNETO_MODE=record \
  -e CASSETTE_NAME=spring-api-calls \
  spring-vcr:latest

# Run integration tests with cassette
docker run --cap-add=NET_ADMIN \
  -v $(pwd)/cassettes:/cassettes \
  -v $(pwd)/magneto-ca.pem:/usr/local/share/ca-certificates/magneto-ca.crt \
  -e MAGNETO_MODE=replay \
  -e CASSETTE_NAME=spring-api-calls \
  spring-vcr:latest \
  ./mvnw test
```

---

## HTTPS/TLS Support

### Generate CA Certificate

First run of magneto generates `magneto-ca.pem`. Install it in your container:

```dockerfile
# In your Dockerfile
COPY magneto-ca.pem /usr/local/share/ca-certificates/magneto-ca.crt
RUN update-ca-certificates
```

Or mount at runtime:

```bash
docker run --cap-add=NET_ADMIN \
  -v $(pwd)/cassettes:/cassettes \
  -v $(pwd)/magneto-ca.pem:/usr/local/share/ca-certificates/magneto-ca.crt \
  magneto-transparent:latest
```

### Language-Specific Certificate Configuration

**Node.js:**

```dockerfile
ENV NODE_EXTRA_CA_CERTS=/usr/local/share/ca-certificates/magneto-ca.crt
```

**Python:**

```dockerfile
ENV REQUESTS_CA_BUNDLE=/usr/local/share/ca-certificates/magneto-ca.crt
ENV SSL_CERT_FILE=/usr/local/share/ca-certificates/magneto-ca.crt
```

**Java:**

```bash
keytool -import -trustcacerts -alias magneto \
  -file /usr/local/share/ca-certificates/magneto-ca.crt \
  -keystore $JAVA_HOME/lib/security/cacerts \
  -storepass changeit -noprompt
```

---

## Troubleshooting

### Issue: "Operation not permitted" when starting container

**Cause**: Missing `NET_ADMIN` capability

**Solution**: Add `--cap-add=NET_ADMIN` to docker run:

```bash
docker run --cap-add=NET_ADMIN ...
```

For docker-compose:

```yaml
cap_add:
  - NET_ADMIN
```

### Issue: HTTPS connections fail with certificate errors

**Cause**: magneto CA certificate not installed

**Solution**: Mount and install CA certificate:

```bash
docker run --cap-add=NET_ADMIN \
  -v $(pwd)/magneto-ca.pem:/usr/local/share/ca-certificates/magneto-ca.crt \
  magneto-transparent:latest
```

Ensure `update-ca-certificates` runs in entrypoint.

### Issue: Some requests bypass the proxy

**Cause**: Application connects to localhost or uses non-HTTP protocols

**Solution**:
1. Check iptables rules: `docker exec <container> iptables -t nat -L`
2. Add custom port redirection: `-e REDIRECT_PORTS="8080,5432"`
3. Some protocols (gRPC, raw TCP) may not be intercepted

### Issue: DNS resolution fails

**Cause**: DNS queries not redirected

**Solution**: Use explicit DNS configuration:

```yaml
services:
  app-with-vcr:
    dns:
      - 8.8.8.8
      - 8.8.4.4
```

Or configure dnsmasq to work with magneto.

### Issue: Application performance degraded

**Cause**: iptables overhead + proxy latency

**Solution**:
1. Use latency simulation to speed up replays: `LatencyMode::Scaled(10)` (10x faster)
2. Optimize cassette size (filter large bodies)
3. Profile with `RUST_LOG=debug`

---

## Differences with Standard Proxy Approach

| Feature | Explicit Proxy | docker-vcr (Transparent) |
|---------|---------------|-------------------------|
| Code changes | Requires `HTTP_PROXY` env | No changes needed |
| Setup complexity | Simple | Moderate (iptables) |
| Privileges | None | `NET_ADMIN` required |
| HTTPS | CA cert in app | CA cert in container |
| DNS interception | No | Possible with dnsmasq |
| Performance | <1ms overhead | ~2-3ms overhead |
| Use case | New apps, tests | Legacy apps, integration tests |

---

## Integration with 1vcr Project

This guide provides a foundation for the **1vcr** project by @1000i100 ([framagit.org/1forma-tic/1vcr](https://framagit.org/1forma-tic/1vcr)).

**magneto-serge** provides:
- Core record/replay engine
- Multi-language bindings
- CLI and library APIs

**1vcr** (docker-vcr) adds:
- Pre-built Docker images for common frameworks
- Turnkey docker-compose templates
- Advanced iptables configuration
- DNS interception
- Framework-specific guides (Rails, Django, Spring Boot, etc.)

Both projects work together:
- magneto-serge = **core library** (stable, well-tested)
- 1vcr = **orchestration layer** (Docker-specific, turnkey)

---

## Contributing

To improve docker-vcr templates or add new examples:

1. Fork the repository
2. Create a new example in `examples/docker-vcr/`
3. Test with your framework
4. Submit a PR with documentation

### Example Contributions Welcome

- **PHP Laravel** with VCR
- **Ruby on Rails** with VCR
- **Go microservices** with VCR
- **Rust services** with VCR
- **Kubernetes** deployments with VCR sidecars

---

## Additional Resources

- [magneto-serge Docker Guide](../../docs/DOCKER.md)
- [Architecture Documentation](../../docs/ARCHITECTURE.md)
- [Latency Simulation](../../docs/LATENCY_SIMULATION.md)
- [1vcr Project](https://framagit.org/1forma-tic/1vcr)

---

**Last updated**: 2025-10-11
**Version**: 0.1.0
