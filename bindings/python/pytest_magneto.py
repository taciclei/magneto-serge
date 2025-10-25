"""
pytest plugin for magneto-serge

Usage:
    Add to conftest.py:
        pytest_plugins = ["pytest_magneto"]

    Or install as package:
        pip install pytest-magneto-serge

    Then use in tests:
        @pytest.mark.magneto(cassette="api-test")
        def test_api():
            response = requests.get("https://api.example.com")
            assert response.status_code == 200
"""

import pytest
import sys
import os

# Add current directory to path for imports
sys.path.insert(0, os.path.dirname(__file__))

try:
    import magneto_serge
    from magneto_serge import MagnetoProxy, ProxyMode
except ImportError:
    magneto_serge = None
    MagnetoProxy = None
    ProxyMode = None


def pytest_addoption(parser):
    """Add command-line options for magneto-serge."""
    group = parser.getgroup("magneto")
    group.addoption(
        "--magneto-cassette-dir",
        action="store",
        default="./test_cassettes",
        help="Directory for cassette storage (default: ./test_cassettes)",
    )
    group.addoption(
        "--magneto-mode",
        action="store",
        default="auto",
        choices=["auto", "record", "replay", "strict"],
        help="Proxy mode: auto (default), record, replay, or strict",
    )
    group.addoption(
        "--magneto-port",
        action="store",
        type=int,
        default=8888,
        help="Proxy port (default: 8888)",
    )
    group.addoption(
        "--magneto-disable",
        action="store_true",
        default=False,
        help="Disable magneto-serge for this test run",
    )


def pytest_configure(config):
    """Register magneto marker."""
    config.addinivalue_line(
        "markers",
        "magneto(cassette=None, mode=None, port=None, strict=False): "
        "use magneto-serge proxy for HTTP/WebSocket recording/replay",
    )


@pytest.fixture(scope="function")
def magneto(request):
    """
    pytest fixture providing magneto-serge proxy for a test.

    Usage:
        def test_api(magneto):
            # Proxy automatically configured
            response = requests.get("https://api.example.com",
                                   proxies=magneto.proxies())
    """
    if magneto_serge is None:
        pytest.skip("magneto-serge not installed")

    config = request.config
    marker = request.node.get_closest_marker("magneto")

    # Check if magneto is disabled
    if config.getoption("--magneto-disable"):
        yield None
        return

    # Get cassette name
    cassette_name = None
    if marker:
        cassette_name = marker.kwargs.get("cassette")

    if not cassette_name:
        # Generate cassette name from test name
        test_name = request.node.name
        cassette_name = test_name.replace("[", "_").replace("]", "_")

    # Get cassette directory
    cassette_dir = config.getoption("--magneto-cassette-dir")

    # Get mode
    mode_str = config.getoption("--magneto-mode")
    if marker:
        marker_mode = marker.kwargs.get("mode")
        if marker_mode:
            mode_str = marker_mode

    # Get port
    port = config.getoption("--magneto-port")
    if marker:
        marker_port = marker.kwargs.get("port")
        if marker_port:
            port = marker_port

    # Check strict mode
    strict = False
    if marker:
        strict = marker.kwargs.get("strict", False)
    if mode_str == "strict":
        strict = True
        mode_str = "replay"

    # Map mode string to ProxyMode
    mode_map = {
        "auto": None,  # Will use hybrid()
        "record": ProxyMode.RECORD,
        "replay": ProxyMode.REPLAY,
    }

    # Create proxy
    proxy = MagnetoProxy(cassette_dir)
    proxy.set_port(port)

    # Start appropriate mode
    if mode_str == "auto":
        proxy.hybrid(cassette_name)
    elif mode_str == "record":
        proxy.set_mode(ProxyMode.RECORD)
        proxy.start_recording(cassette_name)
    elif mode_str == "replay":
        if strict:
            proxy.replay_strict(cassette_name)
        else:
            proxy.replay(cassette_name)

    # Helper method for proxies dict
    def proxies():
        return {
            "http": f"http://localhost:{port}",
            "https": f"http://localhost:{port}",
        }

    proxy.proxies = proxies

    # Yield proxy to test
    yield proxy

    # Cleanup
    try:
        if mode_str == "record":
            # Note: stop_recording not available in current API
            pass
        proxy.shutdown()
    except:
        pass


@pytest.fixture(scope="session")
def magneto_session(request):
    """
    Session-scoped magneto fixture - starts proxy once for entire test session.

    Usage:
        def test_api_1(magneto_session):
            proxy_url = magneto_session.proxies()["http"]
            # Use proxy...

        def test_api_2(magneto_session):
            # Same proxy instance
            pass
    """
    if magneto_serge is None:
        pytest.skip("magneto-serge not installed")

    config = request.config

    # Check if magneto is disabled
    if config.getoption("--magneto-disable"):
        yield None
        return

    cassette_dir = config.getoption("--magneto-cassette-dir")
    port = config.getoption("--magneto-port")
    mode_str = config.getoption("--magneto-mode")

    # Create proxy
    proxy = MagnetoProxy(cassette_dir)
    proxy.set_port(port)

    # Default cassette for session
    cassette_name = "pytest-session"

    # Start appropriate mode
    if mode_str == "auto":
        proxy.hybrid(cassette_name)
    elif mode_str == "record":
        proxy.set_mode(ProxyMode.RECORD)
        proxy.start_recording(cassette_name)
    elif mode_str == "replay":
        proxy.replay(cassette_name)

    # Helper method
    def proxies():
        return {
            "http": f"http://localhost:{port}",
            "https": f"http://localhost:{port}",
        }

    proxy.proxies = proxies

    yield proxy

    # Cleanup
    try:
        proxy.shutdown()
    except:
        pass


def pytest_collection_modifyitems(config, items):
    """Auto-apply magneto fixture to tests with @pytest.mark.magneto."""
    for item in items:
        if item.get_closest_marker("magneto"):
            # Ensure magneto fixture is used
            if "magneto" not in item.fixturenames:
                item.fixturenames.append("magneto")
