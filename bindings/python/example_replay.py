#!/usr/bin/env python3
"""
Replay example of using matgto-serge from Python

This example shows how to:
1. Create a proxy in replay mode
2. Replay a previously recorded cassette
3. Verify requests are served from cassette
"""

import sys
from matgto_serge import create_proxy, ProxyMode

def main():
    print("▶️  matgto-serge Python Example - Replay")
    print("=" * 50)

    # Create proxy instance in replay mode
    print("\n1️⃣ Creating proxy in REPLAY mode...")
    proxy = create_proxy("./cassettes")
    proxy = proxy.with_port(8888)
    proxy = proxy.with_mode(ProxyMode.REPLAY)
    print(f"   ✅ Proxy created on port {proxy.port()}")
    print(f"   ✅ Mode: {proxy.mode()}")

    # Replay cassette
    print("\n2️⃣ Loading cassette for replay...")
    cassette_name = "python-example"
    proxy.replay(cassette_name)
    print(f"   ✅ Replaying cassette: {cassette_name}")

    # Make HTTP requests (will be served from cassette)
    print("\n3️⃣ Making HTTP requests...")
    print("   Requests will be served from cassette (no real network calls)")
    print("   Configure your HTTP client:")
    print("   proxies={'http': 'http://localhost:8888', 'https': 'http://localhost:8888'}")

    # Example with requests library
    print("\n   Example code:")
    print("   ```python")
    print("   import requests")
    print("   response = requests.get(")
    print("       'https://httpbin.org/get',")
    print("       proxies={'http': 'http://localhost:8888'}")
    print("   )")
    print("   print(response.json())")
    print("   ```")

    # Shutdown
    print("\n4️⃣ Shutting down proxy...")
    proxy.shutdown()
    print("   ✅ Proxy stopped")

    print("\n" + "=" * 50)
    print("✅ Replay example complete!")

if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        print(f"\n❌ Error: {e}", file=sys.stderr)
        sys.exit(1)
