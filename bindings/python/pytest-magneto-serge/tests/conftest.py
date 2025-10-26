"""
Pytest configuration for tests
"""

import pytest


@pytest.fixture(scope='session', autouse=True)
def configure_magneto(magneto_config):
    """Configure Magneto-Serge for tests"""
    magneto_config.cassette_dir = 'tests/cassettes'
    magneto_config.mode = 'auto'
    magneto_config.record_mode = 'new_episodes'
    magneto_config.verbose = False
