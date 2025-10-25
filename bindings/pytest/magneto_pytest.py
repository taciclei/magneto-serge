"""
Magnéto-Serge pytest plugin

Provides fixtures and assertions for testing with Magnéto-Serge cassettes.

Installation:
    pip install magneto-pytest

Usage:
    import pytest
    from magneto_pytest import assert_matches_cassette

    def test_user_login(cassette):
        response = requests.post('/api/authenticate', json={'username': 'admin'})
        assert_matches_cassette(response, 'user-login')
"""

import json
import os
from pathlib import Path
from typing import Any, Dict, Optional

import pytest


class CassetteNotFoundError(Exception):
    """Raised when cassette file is not found"""
    pass


class CassetteManager:
    """Manager for loading and accessing cassettes"""

    def __init__(self, cassette_dir: str = "./cassettes"):
        self.cassette_dir = Path(cassette_dir)

    def load(self, name: str) -> Dict[str, Any]:
        """Load a cassette from disk

        Args:
            name: Cassette name (without extension)

        Returns:
            Parsed cassette dictionary

        Raises:
            CassetteNotFoundError: If cassette file is not found
        """
        json_path = self.cassette_dir / f"{name}.json"
        msgpack_path = self.cassette_dir / f"{name}.msgpack"

        if json_path.exists():
            with open(json_path, 'r', encoding='utf-8') as f:
                return json.load(f)
        elif msgpack_path.exists():
            raise NotImplementedError("MessagePack cassettes not yet supported")
        else:
            raise CassetteNotFoundError(f"Cassette not found: {name}")

    def find_interaction(
        self,
        cassette: Dict[str, Any],
        method: str,
        url: str,
        body: Optional[Any] = None
    ) -> Optional[Dict[str, Any]]:
        """Find matching interaction in cassette

        Args:
            cassette: Loaded cassette dictionary
            method: HTTP method (GET, POST, etc.)
            url: Request URL
            body: Optional request body

        Returns:
            Matching interaction or None
        """
        interactions = cassette.get('interactions', [])

        for interaction in interactions:
            if 'kind' not in interaction or 'Http' not in interaction['kind']:
                continue

            http = interaction['kind']['Http']
            request = http['request']

            if request['method'] == method and request['url'] == url:
                # If body is provided, match it too
                if body is not None:
                    req_body = request.get('body')
                    if req_body is not None:
                        # TODO: Better body matching
                        pass

                return interaction

        return None


# Global cassette manager instance
_cassette_manager = CassetteManager()


@pytest.fixture(scope="session")
def cassette_dir(request):
    """Pytest fixture to configure cassette directory

    Usage:
        @pytest.fixture(scope="session")
        def cassette_dir():
            return "./e2e-cassettes"
    """
    cassette_dir = getattr(request.config, 'cassette_dir', './cassettes')
    _cassette_manager.cassette_dir = Path(cassette_dir)
    return cassette_dir


@pytest.fixture
def cassette():
    """Pytest fixture providing cassette manager

    Usage:
        def test_something(cassette):
            cass = cassette.load('user-login')
            assert cass['version'] == '2.0'
    """
    return _cassette_manager


def assert_matches_cassette(response, cassette_name: str):
    """Assert that HTTP response matches a cassette

    Args:
        response: HTTP response object (requests.Response or similar)
        cassette_name: Name of cassette to match against

    Raises:
        AssertionError: If response doesn't match cassette
    """
    cassette = _cassette_manager.load(cassette_name)

    # Extract request info from response
    request = response.request
    method = request.method
    url = str(request.url)

    interaction = _cassette_manager.find_interaction(cassette, method, url)

    assert interaction is not None, \
        f"No matching interaction found in cassette '{cassette_name}' for {method} {url}"

    # Validate status code
    expected_status = interaction['kind']['Http']['response']['status']
    actual_status = response.status_code

    assert actual_status == expected_status, \
        f"Expected status {expected_status} but got {actual_status}"


def assert_cassette_status(response, cassette_name: str, expected_status: int):
    """Assert that response status matches cassette

    Args:
        response: HTTP response object
        cassette_name: Name of cassette
        expected_status: Expected status code

    Raises:
        AssertionError: If status doesn't match
    """
    cassette = _cassette_manager.load(cassette_name)

    request = response.request
    method = request.method
    url = str(request.url)

    interaction = _cassette_manager.find_interaction(cassette, method, url)

    assert interaction is not None, \
        f"No matching interaction found in cassette '{cassette_name}'"

    cassette_status = interaction['kind']['Http']['response']['status']

    assert cassette_status == expected_status, \
        f"Expected status {expected_status} but cassette has {cassette_status}"


def assert_cassette_body(response, cassette_name: str):
    """Assert that response body matches cassette

    Args:
        response: HTTP response object
        cassette_name: Name of cassette

    Raises:
        AssertionError: If body doesn't match
    """
    cassette = _cassette_manager.load(cassette_name)

    request = response.request
    method = request.method
    url = str(request.url)

    interaction = _cassette_manager.find_interaction(cassette, method, url)

    assert interaction is not None, \
        f"No matching interaction found in cassette '{cassette_name}'"

    expected_body = interaction['kind']['Http']['response'].get('body')
    actual_body = response.content

    if expected_body is not None:
        expected_bytes = bytes(expected_body)
        assert actual_body == expected_bytes, \
            "Response body doesn't match cassette"


def assert_interaction_count(cassette_name: str, expected_count: int):
    """Assert that cassette has expected number of interactions

    Args:
        cassette_name: Name of cassette
        expected_count: Expected interaction count

    Raises:
        AssertionError: If count doesn't match
    """
    cassette = _cassette_manager.load(cassette_name)
    actual_count = len(cassette.get('interactions', []))

    assert actual_count == expected_count, \
        f"Expected {expected_count} interactions but found {actual_count}"


def assert_has_cookies(cassette_name: str):
    """Assert that cassette contains cookies

    Args:
        cassette_name: Name of cassette

    Raises:
        AssertionError: If cassette has no cookies
    """
    cassette = _cassette_manager.load(cassette_name)
    cookies = cassette.get('cookies', [])

    assert len(cookies) > 0, "Cassette has no cookies"


def assert_has_cookie(cassette_name: str, cookie_name: str):
    """Assert that cassette has specific cookie

    Args:
        cassette_name: Name of cassette
        cookie_name: Name of cookie to find

    Raises:
        AssertionError: If cookie not found
    """
    cassette = _cassette_manager.load(cassette_name)
    cookies = cassette.get('cookies', [])

    cookie_names = [c['name'] for c in cookies]

    assert cookie_name in cookie_names, \
        f"Cookie '{cookie_name}' not found in cassette"


def assert_cassette_version(cassette_name: str, expected_version: str):
    """Assert that cassette version matches expected

    Args:
        cassette_name: Name of cassette
        expected_version: Expected version (e.g., "2.0")

    Raises:
        AssertionError: If version doesn't match
    """
    cassette = _cassette_manager.load(cassette_name)
    actual_version = cassette.get('version')

    assert actual_version == expected_version, \
        f"Expected version {expected_version} but found {actual_version}"


# Pytest hooks
def pytest_configure(config):
    """Pytest configuration hook"""
    config.addinivalue_line(
        "markers", "cassette(name): mark test to use specific cassette"
    )
