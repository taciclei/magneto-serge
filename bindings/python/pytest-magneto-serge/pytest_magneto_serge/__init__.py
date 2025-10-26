"""
pytest-magneto-serge - pytest integration for Magnéto-Serge

Provides automatic cassette management for pytest tests.
"""

__version__ = "0.3.1"

from .decorators import magneto_cassette, use_cassette
from .fixtures import magneto_proxy, magneto_config

__all__ = [
    "magneto_cassette",
    "use_cassette",
    "magneto_proxy",
    "magneto_config",
]
