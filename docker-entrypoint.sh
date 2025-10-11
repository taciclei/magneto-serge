#!/bin/bash
# docker-entrypoint.sh
# Transparent proxy entrypoint for magneto-serge
#
# This script configures iptables to redirect all HTTP/HTTPS traffic
# through magneto-serge, enabling zero-code-change testing.

set -e

# Configuration via environment variables
MAGNETO_MODE="${MAGNETO_MODE:-replay}"
CASSETTE_NAME="${CASSETTE_NAME:-default}"
MAGNETO_PORT="${MAGNETO_PORT:-8888}"
CASSETTE_DIR="${CASSETTE_DIR:-/cassettes}"
TRANSPARENT_PROXY="${TRANSPARENT_PROXY:-false}"

echo "========================================="
echo "  Magnéto-Serge Transparent Proxy"
echo "========================================="
echo "Mode:         $MAGNETO_MODE"
echo "Cassette:     $CASSETTE_NAME"
echo "Port:         $MAGNETO_PORT"
echo "Directory:    $CASSETTE_DIR"
echo "Transparent:  $TRANSPARENT_PROXY"
echo "========================================="

# Start magneto-serge in background
echo "Starting magneto-serge..."
magneto "$MAGNETO_MODE" "$CASSETTE_NAME" \
  --port "$MAGNETO_PORT" \
  --cassette-dir "$CASSETTE_DIR" &

MAGNETO_PID=$!

# Wait for magneto to be ready
echo "Waiting for magneto to start..."
sleep 3

# Check if magneto is running
if ! kill -0 $MAGNETO_PID 2>/dev/null; then
    echo "ERROR: magneto-serge failed to start"
    exit 1
fi

echo "✓ magneto-serge is running (PID: $MAGNETO_PID)"

# Configure transparent proxy if enabled
if [ "$TRANSPARENT_PROXY" = "true" ]; then
    echo "Configuring transparent proxy with iptables..."

    # Redirect HTTP traffic (port 80)
    iptables -t nat -A OUTPUT -p tcp --dport 80 -j REDIRECT --to-port "$MAGNETO_PORT"
    echo "✓ HTTP traffic (port 80) redirected to magneto"

    # Redirect HTTPS traffic (port 443)
    iptables -t nat -A OUTPUT -p tcp --dport 443 -j REDIRECT --to-port "$MAGNETO_PORT"
    echo "✓ HTTPS traffic (port 443) redirected to magneto"

    # Optional: Redirect custom ports
    if [ -n "$REDIRECT_PORTS" ]; then
        IFS=',' read -ra PORTS <<< "$REDIRECT_PORTS"
        for port in "${PORTS[@]}"; do
            iptables -t nat -A OUTPUT -p tcp --dport "$port" -j REDIRECT --to-port "$MAGNETO_PORT"
            echo "✓ Custom port $port redirected to magneto"
        done
    fi
fi

# Install CA certificate if available
if [ -f "$CASSETTE_DIR/magneto-ca.pem" ]; then
    echo "Installing magneto CA certificate..."
    cp "$CASSETTE_DIR/magneto-ca.pem" /usr/local/share/ca-certificates/magneto-ca.crt
    update-ca-certificates >/dev/null 2>&1
    echo "✓ CA certificate installed"
elif [ "$TRANSPARENT_PROXY" = "true" ]; then
    echo "⚠ Warning: No CA certificate found. HTTPS interception may fail."
    echo "  Expected location: $CASSETTE_DIR/magneto-ca.pem"
fi

# Cleanup function on exit
cleanup() {
    echo ""
    echo "Shutting down magneto-serge..."

    # Remove iptables rules if transparent proxy was enabled
    if [ "$TRANSPARENT_PROXY" = "true" ]; then
        echo "Removing iptables rules..."
        iptables -t nat -D OUTPUT -p tcp --dport 80 -j REDIRECT --to-port "$MAGNETO_PORT" 2>/dev/null || true
        iptables -t nat -D OUTPUT -p tcp --dport 443 -j REDIRECT --to-port "$MAGNETO_PORT" 2>/dev/null || true

        if [ -n "$REDIRECT_PORTS" ]; then
            IFS=',' read -ra PORTS <<< "$REDIRECT_PORTS"
            for port in "${PORTS[@]}"; do
                iptables -t nat -D OUTPUT -p tcp --dport "$port" -j REDIRECT --to-port "$MAGNETO_PORT" 2>/dev/null || true
            done
        fi
    fi

    # Kill magneto process
    kill $MAGNETO_PID 2>/dev/null || true
    wait $MAGNETO_PID 2>/dev/null || true

    echo "✓ Cleanup complete"
    exit 0
}

# Register cleanup handler
trap cleanup SIGTERM SIGINT EXIT

echo ""
echo "========================================="
echo "  Ready to accept connections"
echo "========================================="
echo ""

# If no command provided, keep magneto running
if [ $# -eq 0 ]; then
    echo "magneto-serge is running in background..."
    echo "Press Ctrl+C to stop"

    # Wait for magneto process to exit
    wait $MAGNETO_PID
    exit_code=$?

    echo "magneto-serge exited with code $exit_code"
    exit $exit_code
fi

# Otherwise, run the provided command
echo "Starting application: $@"
echo ""

# Execute the application
exec "$@"
