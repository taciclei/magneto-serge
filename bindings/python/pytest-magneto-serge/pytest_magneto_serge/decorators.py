"""
Decorators for pytest-magneto-serge
"""

import functools
from typing import Optional, Callable, Any
from pathlib import Path

from magneto_serge import MagnetoProxy


def magneto_cassette(
    name: Optional[str] = None,
    mode: Optional[str] = None,
    record: Optional[str] = None,
    cassette_dir: Optional[str] = None,
    port: Optional[int] = None,
):
    """
    Decorator to automatically use a cassette for a test function.

    Args:
        name: Cassette name (auto-generated from test name if None)
        mode: Recording mode ('auto', 'record', 'replay', 'passthrough')
        record: VCR-compatible mode ('new_episodes', 'once', 'all', 'none')
        cassette_dir: Directory for cassettes (overrides default)
        port: Proxy port (overrides default)

    Example:
        @magneto_cassette('github_users')
        def test_fetch_users():
            response = requests.get('https://api.github.com/users')
            assert response.status_code == 200

        @magneto_cassette(record='all')
        def test_force_record():
            # Always re-records
            pass
    """

    def decorator(func: Callable) -> Callable:
        @functools.wraps(func)
        def wrapper(*args, **kwargs):
            # Import here to avoid circular dependency
            from .plugin import _config

            # Determine cassette name
            cassette_name = name
            if not cassette_name:
                # Auto-generate from function name
                cassette_name = func.__name__
                if cassette_name.startswith("test_"):
                    cassette_name = cassette_name[5:]

            # Determine mode
            cassette_mode = mode
            if not cassette_mode:
                record_mode = record or _config.record_mode
                cassette_mode = _config.translate_record_mode(record_mode)

            # Determine cassette directory
            cassette_directory = cassette_dir or _config.cassette_dir

            # Create proxy
            proxy = MagnetoProxy(cassette_directory)

            try:
                # Start cassette
                if cassette_mode == "auto":
                    proxy.auto(cassette_name)
                elif cassette_mode == "record":
                    proxy.record(cassette_name)
                elif cassette_mode == "replay":
                    proxy.replay(cassette_name)
                elif cassette_mode == "passthrough":
                    proxy.passthrough()

                if _config.verbose:
                    print(f"[magneto-serge] Started cassette: {cassette_name} (mode: {cassette_mode})")

                # Execute test
                result = func(*args, **kwargs)

                return result
            finally:
                # Stop cassette
                proxy.stop()
                if _config.verbose:
                    print(f"[magneto-serge] Stopped cassette: {cassette_name}")

        return wrapper

    return decorator


class use_cassette:
    """
    Context manager for manual cassette control.

    Can be used standalone or as a decorator.

    Example as context manager:
        def test_api():
            with use_cassette('my_cassette'):
                response = requests.get('https://api.example.com/data')

    Example as decorator:
        @use_cassette('my_cassette')
        def test_api():
            response = requests.get('https://api.example.com/data')
    """

    def __init__(
        self,
        name: str,
        mode: Optional[str] = None,
        record: Optional[str] = None,
        cassette_dir: Optional[str] = None,
    ):
        self.name = name
        self.mode = mode
        self.record = record
        self.cassette_dir = cassette_dir
        self.proxy: Optional[MagnetoProxy] = None

    def __enter__(self):
        """Enter context manager"""
        from .plugin import _config

        # Determine mode
        cassette_mode = self.mode
        if not cassette_mode:
            record_mode = self.record or _config.record_mode
            cassette_mode = _config.translate_record_mode(record_mode)

        # Determine directory
        cassette_directory = self.cassette_dir or _config.cassette_dir

        # Create and start proxy
        self.proxy = MagnetoProxy(cassette_directory)

        if cassette_mode == "auto":
            self.proxy.auto(self.name)
        elif cassette_mode == "record":
            self.proxy.record(self.name)
        elif cassette_mode == "replay":
            self.proxy.replay(self.name)
        elif cassette_mode == "passthrough":
            self.proxy.passthrough()

        if _config.verbose:
            print(f"[magneto-serge] Started cassette: {self.name} (mode: {cassette_mode})")

        return self.proxy

    def __exit__(self, exc_type, exc_val, exc_tb):
        """Exit context manager"""
        from .plugin import _config

        if self.proxy:
            self.proxy.stop()
            if _config.verbose:
                print(f"[magneto-serge] Stopped cassette: {self.name}")

        return False

    def __call__(self, func: Callable) -> Callable:
        """Use as decorator"""

        @functools.wraps(func)
        def wrapper(*args, **kwargs):
            with self:
                return func(*args, **kwargs)

        return wrapper
