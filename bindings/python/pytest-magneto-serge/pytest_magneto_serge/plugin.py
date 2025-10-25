"""
pytest plugin for Magneto-Serge integration
"""

import pytest
from pathlib import Path
from typing import Optional, Dict, Any

from magneto_serge import MagnetoProxy


class MagnetoConfig:
    """Global configuration for pytest-magneto-serge"""

    def __init__(self):
        self.cassette_dir: str = "tests/cassettes"
        self.mode: str = "auto"
        self.port: int = 8888
        self.record_mode: str = "new_episodes"
        self.verbose: bool = False

    def translate_record_mode(self, record_mode: str) -> str:
        """Translate VCR record mode to Magneto mode"""
        modes = {
            "new_episodes": "auto",
            "once": "replay",
            "all": "record",
            "none": "replay",
        }
        return modes.get(record_mode, "auto")


# Global config instance
_config = MagnetoConfig()


def pytest_configure(config):
    """Register magneto_cassette marker"""
    config.addinivalue_line(
        "markers",
        "magneto_cassette(name, mode=None, record=None, port=None): "
        "Mark test to use Magneto-Serge cassette"
    )


@pytest.fixture(scope="session")
def magneto_config():
    """Global Magneto-Serge configuration fixture"""
    return _config


@pytest.fixture
def magneto_proxy(request, magneto_config):
    """
    Fixture providing MagnetoProxy instance with automatic cassette management.

    Can be used directly or with marker:
        @pytest.mark.magneto_cassette('test_name')
        def test_api(magneto_proxy):
            ...
    """
    marker = request.node.get_closest_marker("magneto_cassette")

    if marker:
        # Extract marker arguments
        cassette_name = marker.args[0] if marker.args else None
        cassette_options = marker.kwargs

        if not cassette_name:
            # Auto-generate from test name
            cassette_name = _generate_cassette_name(request)

        # Get mode
        mode = cassette_options.get("mode")
        if not mode:
            record_mode = cassette_options.get("record", magneto_config.record_mode)
            mode = magneto_config.translate_record_mode(record_mode)

        # Create proxy
        proxy = MagnetoProxy(magneto_config.cassette_dir)

        # Start cassette
        if mode == "auto":
            proxy.auto(cassette_name)
        elif mode == "record":
            proxy.record(cassette_name)
        elif mode == "replay":
            proxy.replay(cassette_name)
        elif mode == "passthrough":
            proxy.passthrough()

        if magneto_config.verbose:
            print(f"[magneto-serge] Started cassette: {cassette_name} (mode: {mode})")

        # Store cassette name for access in tests
        request.node.magneto_cassette_name = cassette_name

        yield proxy

        # Stop proxy
        proxy.stop()

        if magneto_config.verbose:
            print(f"[magneto-serge] Stopped cassette: {cassette_name}")
    else:
        # No marker, provide proxy but don't auto-start cassette
        proxy = MagnetoProxy(magneto_config.cassette_dir)
        yield proxy
        proxy.stop()


def _generate_cassette_name(request) -> str:
    """Generate cassette name from test name and hierarchy"""
    parts = []

    # Get module name
    if request.module:
        module_name = request.module.__name__.split(".")[-1]
        if module_name != "test":
            parts.append(module_name)

    # Get class name if in class
    if request.cls:
        parts.append(request.cls.__name__)

    # Get function name
    function_name = request.function.__name__
    # Remove 'test_' prefix if present
    if function_name.startswith("test_"):
        function_name = function_name[5:]

    parts.append(function_name)

    return "/".join(parts)


@pytest.fixture
def use_cassette(magneto_config):
    """
    Context manager fixture for manual cassette control.

    Usage:
        def test_api(use_cassette):
            with use_cassette('my_cassette'):
                response = requests.get('https://api.example.com/data')
    """
    from contextlib import contextmanager

    @contextmanager
    def _use_cassette(name: str, mode: Optional[str] = None, record: Optional[str] = None):
        """Context manager for cassette lifecycle"""
        # Determine mode
        if not mode:
            record_mode = record or magneto_config.record_mode
            mode = magneto_config.translate_record_mode(record_mode)

        proxy = MagnetoProxy(magneto_config.cassette_dir)

        try:
            # Start cassette
            if mode == "auto":
                proxy.auto(name)
            elif mode == "record":
                proxy.record(name)
            elif mode == "replay":
                proxy.replay(name)
            elif mode == "passthrough":
                proxy.passthrough()

            if magneto_config.verbose:
                print(f"[magneto-serge] Started cassette: {name} (mode: {mode})")

            yield proxy
        finally:
            proxy.stop()
            if magneto_config.verbose:
                print(f"[magneto-serge] Stopped cassette: {name}")

    return _use_cassette
