"""
Fixtures for pytest-magneto-serge

Re-export fixtures from plugin module for convenience.
"""

from .plugin import magneto_proxy, magneto_config

__all__ = ["magneto_proxy", "magneto_config"]
