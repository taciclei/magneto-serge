#!/bin/bash
#
# Example bash script for Magneto-Serge REST API
#
# Demonstrates how to:
# - Start/stop the proxy
# - Check proxy status
# - List cassettes
# - Work with Hydra/JSON-LD responses

set -e

# Configuration
API_BASE_URL="${MAGNETO_API_URL:-http://localhost:8889}"
API_KEY="${MAGNETO_API_KEY:-}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper function to make API requests
api_request() {
    local method="$1"
    local path="$2"
    local data="$3"
    local url="${API_BASE_URL}${path}"

    local curl_opts=(-s -X "$method")

    if [ -n "$API_KEY" ]; then
        curl_opts+=(-H "Authorization: Bearer $API_KEY")
    fi

    if [ -n "$data" ]; then
        curl_opts+=(-H "Content-Type: application/json" -d "$data")
    fi

    curl "${curl_opts[@]}" "$url"
}

# Pretty print JSON
pretty_json() {
    if command -v jq &> /dev/null; then
        jq '.'
    else
        cat
    fi
}

echo -e "${BLUE}🌐 Magneto-Serge API Client Example${NC}\n"

# 1. Get API root
echo -e "${GREEN}1️⃣  Getting API root...${NC}"
root_response=$(api_request GET "/" | pretty_json)
echo "$root_response"
echo ""

# 2. Check health
echo -e "${GREEN}2️⃣  Checking health...${NC}"
health_response=$(api_request GET "/health" | pretty_json)
status=$(echo "$health_response" | jq -r '.data.status')
uptime=$(echo "$health_response" | jq -r '.data.uptime_seconds')
echo "   Status: $status"
echo "   Uptime: $uptime seconds"
echo ""

# 3. Get proxy status
echo -e "${GREEN}3️⃣  Getting proxy status...${NC}"
status_response=$(api_request GET "/proxy/status" | pretty_json)
running=$(echo "$status_response" | jq -r '.data.running')
mode=$(echo "$status_response" | jq -r '.data.mode')
echo "   Running: $running"
echo "   Mode: $mode"
echo ""

# 4. Start proxy (if not running)
echo -e "${GREEN}4️⃣  Starting proxy in auto mode...${NC}"
if [ "$running" = "false" ]; then
    start_data='{
        "mode": "auto",
        "cassette_name": "example-test",
        "port": 8888,
        "strict": false
    }'

    start_response=$(api_request POST "/proxy/start" "$start_data")
    success=$(echo "$start_response" | jq -r '.success')

    if [ "$success" = "true" ]; then
        message=$(echo "$start_response" | jq -r '.data.message')
        cassette=$(echo "$start_response" | jq -r '.data.cassette')
        port=$(echo "$start_response" | jq -r '.data.port')
        echo -e "   ${GREEN}✓${NC} $message"
        echo "   Cassette: $cassette"
        echo "   Port: $port"

        # Extract Hydra link to status endpoint
        echo ""
        echo "   Following 'Check Proxy Status' link..."
        status_link=$(echo "$start_response" | jq -r '.["hydra:link"][] | select(.title == "Check Proxy Status") | .["hydra:target"]')
        if [ -n "$status_link" ]; then
            status_path="${status_link#$API_BASE_URL}"
            new_status=$(api_request GET "$status_path")
            proxy_running=$(echo "$new_status" | jq -r '.data.running')
            echo "   Proxy running: $proxy_running"
        fi

        # Stop proxy
        echo ""
        echo -e "${GREEN}5️⃣  Stopping proxy...${NC}"
        stop_data='{"force": false}'
        stop_response=$(api_request POST "/proxy/stop" "$stop_data")
        stop_message=$(echo "$stop_response" | jq -r '.data.message')
        echo -e "   ${GREEN}✓${NC} $stop_message"
    else
        error=$(echo "$start_response" | jq -r '.error')
        echo -e "   ${RED}✗${NC} $error"
    fi
else
    echo -e "   ${YELLOW}⚠${NC} Proxy already running"
fi
echo ""

# 6. List cassettes
echo -e "${GREEN}6️⃣  Listing cassettes...${NC}"
cassettes_response=$(api_request GET "/cassettes")
cassettes=$(echo "$cassettes_response" | jq -r '.data')
count=$(echo "$cassettes" | jq 'length')
echo "   Found $count cassettes:"
echo "$cassettes" | jq -r '.[] | "   • \(.name) (\(.size_bytes) bytes)"' | head -5
echo ""

# 7. Get OpenAPI spec
echo -e "${GREEN}7️⃣  Getting OpenAPI specification...${NC}"
spec_response=$(api_request GET "/openapi.json")
openapi_version=$(echo "$spec_response" | jq -r '.openapi')
api_title=$(echo "$spec_response" | jq -r '.info.title')
endpoints_count=$(echo "$spec_response" | jq '.paths | length')
echo "   OpenAPI version: $openapi_version"
echo "   API title: $api_title"
echo "   Endpoints: $endpoints_count"
echo ""

echo -e "${GREEN}✅ All operations completed successfully!${NC}"
echo ""
echo -e "${BLUE}Tip:${NC} Export MAGNETO_API_KEY to use authentication:"
echo "  export MAGNETO_API_KEY=your-secret-key"
