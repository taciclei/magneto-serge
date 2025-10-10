#!/usr/bin/env python3
"""
Basic example of using matgto-serge from Python

This example shows how to:
1. Create a proxy in record mode
2. Start recording HTTP/WebSocket interactions
3. Stop recording and save to cassette
"""

import sys
import requests
from matgto_serge import create_proxy, ProxyMode

def main():
    print("🎬 matgto-serge Python Example - Basic Recording")
    print("=" * 50)

    # Create proxy instance
    print("\n1️⃣ Creating proxy...")
    proxy = create_proxy("./cassettes")
    proxy = proxy.with_port(8888)
    proxy = proxy.with_mode(ProxyMode.RECORD)
    print(f"   ✅ Proxy created on port {proxy.port()}")

    # Start recording
    print("\n2️⃣ Starting recording...")
    cassette_name = "python-example"
    proxy.start_recording(cassette_name)
    print(f"   ✅ Recording to cassette: {cassette_name}")

    # Make HTTP requests through proxy
    print("\n3️⃣ Making HTTP requests through proxy...")
    print("   (In a real scenario, configure your HTTP client to use proxy)")
    print("   Example: requests.get('https://httpbin.org/get', proxies={'http': 'http://localhost:8888'})")

    # Stop recording
    print("\n4️⃣ Stopping recording...")
    proxy.stop_recording()
    print(f"   ✅ Cassette saved: ./cassettes/{cassette_name}.json")

    # Shutdown proxy
    print("\n5️⃣ Shutting down proxy...")
    proxy.shutdown()
    print("   ✅ Proxy stopped")

    print("\n" + "=" * 50)
    print("✅ Example complete!")

if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        print(f"\n❌ Error: {e}", file=sys.stderr)
        sys.exit(1)
