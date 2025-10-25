#!/usr/bin/env python3
"""
Example Python client for Magneto-Serge REST API

Demonstrates how to:
- Start/stop the proxy
- Check proxy status
- List cassettes
- Navigate using Hydra links
"""

import requests
import json
from typing import Optional

class MagnetoAPI:
    """Client for Magneto-Serge REST API"""

    def __init__(self, base_url: str = "http://localhost:8889", api_key: Optional[str] = None):
        self.base_url = base_url.rstrip('/')
        self.api_key = api_key
        self.session = requests.Session()

        if api_key:
            self.session.headers.update({
                'Authorization': f'Bearer {api_key}'
            })

    def _request(self, method: str, path: str, **kwargs):
        """Make API request"""
        url = f"{self.base_url}{path}"
        response = self.session.request(method, url, **kwargs)
        response.raise_for_status()
        return response.json()

    def get_root(self):
        """Get API root with Hydra links"""
        return self._request('GET', '/')

    def health(self):
        """Check API health"""
        return self._request('GET', '/health')

    def start_proxy(self, cassette_name: str, mode: str = "auto",
                   port: Optional[int] = None, strict: bool = False):
        """Start proxy

        Args:
            cassette_name: Name of cassette to use
            mode: Proxy mode (auto, record, replay, passthrough)
            port: Proxy port (optional)
            strict: Enable strict replay mode
        """
        data = {
            "mode": mode,
            "cassette_name": cassette_name,
            "strict": strict
        }
        if port:
            data["port"] = port

        return self._request('POST', '/proxy/start', json=data)

    def stop_proxy(self, force: bool = False):
        """Stop proxy"""
        return self._request('POST', '/proxy/stop', json={"force": force})

    def get_status(self):
        """Get proxy status"""
        return self._request('GET', '/proxy/status')

    def get_stats(self):
        """Get proxy statistics"""
        return self._request('GET', '/proxy/stats')

    def list_cassettes(self):
        """List all cassettes"""
        return self._request('GET', '/cassettes')

    def get_cassette(self, name: str):
        """Get cassette content"""
        return self._request('GET', f'/cassettes/{name}')

    def delete_cassette(self, name: str):
        """Delete cassette"""
        return self._request('DELETE', f'/cassettes/{name}')

    def get_openapi_spec(self):
        """Get OpenAPI specification"""
        return self._request('GET', '/openapi.json')

    def follow_link(self, response: dict, link_title: str):
        """Follow a Hydra link from response

        Args:
            response: API response containing hydra:link
            link_title: Title of link to follow
        """
        links = response.get('hydra:link', [])
        for link in links:
            if link.get('title') == link_title:
                target = link.get('hydra:target')
                if target:
                    # Extract path from full URL
                    path = target.replace(self.base_url, '')
                    return self._request('GET', path)

        raise ValueError(f"Link with title '{link_title}' not found")


def main():
    """Example usage"""

    # Create API client (no auth for this example)
    api = MagnetoAPI()

    print("🌐 Magneto-Serge API Client Example\n")

    # 1. Get API root and discover endpoints
    print("1️⃣  Getting API root...")
    root = api.get_root()
    print(f"   API: {root['data']['title']}")
    print(f"   Version: {root['data']['version']}")
    print(f"   Available links: {len(root.get('hydra:link', []))}")
    print()

    # 2. Check health
    print("2️⃣  Checking health...")
    health = api.health()
    print(f"   Status: {health['data']['status']}")
    print(f"   Uptime: {health['data']['uptime_seconds']} seconds")
    print()

    # 3. Get proxy status
    print("3️⃣  Getting proxy status...")
    status = api.get_status()
    print(f"   Running: {status['data']['running']}")
    print(f"   Mode: {status['data']['mode']}")
    print()

    # 4. Start proxy in auto mode
    print("4️⃣  Starting proxy in auto mode...")
    try:
        start_response = api.start_proxy("example-test", mode="auto", port=8888)
        print(f"   ✓ {start_response['data']['message']}")
        print(f"   Mode: {start_response['data']['mode']}")
        print(f"   Cassette: {start_response['data']['cassette']}")
        print(f"   Port: {start_response['data']['port']}")

        # Follow Hydra link to check status
        print("\n   Following 'Check Proxy Status' link...")
        status = api.follow_link(start_response, 'Check Proxy Status')
        print(f"   Proxy running: {status['data']['running']}")

        # Stop proxy
        print("\n5️⃣  Stopping proxy...")
        stop_response = api.stop_proxy()
        print(f"   ✓ {stop_response['data']['message']}")

    except requests.exceptions.HTTPError as e:
        if e.response.status_code == 409:
            print("   ⚠ Proxy already running")
        else:
            raise
    print()

    # 6. List cassettes
    print("6️⃣  Listing cassettes...")
    cassettes_response = api.list_cassettes()
    cassettes = cassettes_response['data']
    print(f"   Found {len(cassettes)} cassettes:")
    for cassette in cassettes[:5]:  # Show first 5
        print(f"   • {cassette['name']} ({cassette['size_bytes']} bytes)")
    print()

    # 7. Get OpenAPI spec
    print("7️⃣  Getting OpenAPI specification...")
    spec = api.get_openapi_spec()
    print(f"   OpenAPI version: {spec['openapi']}")
    print(f"   API title: {spec['info']['title']}")
    print(f"   Endpoints: {len(spec['paths'])}")
    print()

    print("✅ All operations completed successfully!")


if __name__ == "__main__":
    try:
        main()
    except requests.exceptions.ConnectionError:
        print("❌ Error: Cannot connect to API server")
        print("   Make sure the API server is running:")
        print("   $ magneto api")
    except Exception as e:
        print(f"❌ Error: {e}")
