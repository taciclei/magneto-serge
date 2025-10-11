# Basic Docker Example

This example demonstrates a simple setup using magneto-serge with Docker Compose.

## Prerequisites

- Docker
- Docker Compose

## Running the Example

```bash
# Start the services
docker-compose up

# The app service will make HTTP/HTTPS requests through the magneto proxy
# All interactions will be recorded in ./cassettes/example-test.json
```

## What Happens

1. Docker Compose starts two services:
   - `magneto`: The proxy running in auto mode
   - `app`: A curl container that makes HTTP requests

2. The app is configured to use magneto as its HTTP/HTTPS proxy via environment variables

3. All requests are captured and stored in `./cassettes/example-test.json`

4. On subsequent runs, magneto will replay the recorded interactions instead of making real network requests

## Testing Record/Replay

```bash
# First run - records interactions
docker-compose up

# Check the cassette file
cat cassettes/example-test.json

# Second run - replays from cassette (no real network calls)
docker-compose up

# To force recording again, delete the cassette
rm cassettes/example-test.json
docker-compose up
```

## Clean Up

```bash
# Stop services
docker-compose down

# Remove cassettes
rm -rf cassettes/
```
