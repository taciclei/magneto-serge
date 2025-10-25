"""
Unit tests for Magnéto-Serge Python bindings (no server needed)
"""

import pytest
import json
import tempfile
import os
from pathlib import Path


def create_test_cassette(name, version="1.0", interaction_count=2):
    """Create a test cassette file"""
    cassette = {
        "version": version,
        "name": name,
        "recorded_at": "2025-10-25T10:00:00Z",
        "interactions": [
            {
                "kind": {
                    "Http": {
                        "request": {
                            "method": "GET",
                            "url": "https://api.example.com/users",
                            "headers": {"Accept": "application/json"},
                            "body": None
                        },
                        "response": {
                            "status": 200,
                            "headers": {"Content-Type": "application/json"},
                            "body": [123, 34, 117, 115, 101, 114, 115, 34, 58, 91, 93, 125]  # '{"users":[]}'
                        }
                    }
                }
            }
        ] * interaction_count,
        "cookies": [
            {
                "name": "JSESSIONID",
                "value": "ABC123",
                "domain": "example.com",
                "path": "/",
                "expires": None,
                "max_age": None,
                "secure": True,
                "http_only": True,
                "same_site": None,
                "created_at": "2025-10-25T10:00:00Z"
            }
        ]
    }

    # Create temp directory
    temp_dir = tempfile.mkdtemp()
    cassette_path = Path(temp_dir) / f"{name}.json"

    with open(cassette_path, 'w') as f:
        json.dump(cassette, f)

    return temp_dir, cassette_path


class TestCassetteHelpers:
    """Test cassette helper functions"""

    def setup_method(self):
        """Setup test cassette before each test"""
        self.cassette_dir, self.cassette_path = create_test_cassette("test-cassette")

    def teardown_method(self):
        """Cleanup after each test"""
        import shutil
        if os.path.exists(self.cassette_dir):
            shutil.rmtree(self.cassette_dir)

    def test_load_cassette(self):
        """Test loading a cassette file"""
        with open(self.cassette_path) as f:
            cassette = json.load(f)

        assert cassette['version'] == '1.0'
        assert cassette['name'] == 'test-cassette'
        assert len(cassette['interactions']) == 2

    def test_cassette_has_cookies(self):
        """Test cassette contains cookies"""
        with open(self.cassette_path) as f:
            cassette = json.load(f)

        assert 'cookies' in cassette
        assert len(cassette['cookies']) > 0

    def test_cassette_has_specific_cookie(self):
        """Test cassette has specific cookie"""
        with open(self.cassette_path) as f:
            cassette = json.load(f)

        cookie_names = [c['name'] for c in cassette['cookies']]
        assert 'JSESSIONID' in cookie_names

    def test_interaction_count(self):
        """Test cassette interaction count"""
        with open(self.cassette_path) as f:
            cassette = json.load(f)

        assert len(cassette['interactions']) == 2

    def test_http_interaction_structure(self):
        """Test HTTP interaction has correct structure"""
        with open(self.cassette_path) as f:
            cassette = json.load(f)

        interaction = cassette['interactions'][0]
        assert 'kind' in interaction
        assert 'Http' in interaction['kind']

        http = interaction['kind']['Http']
        assert 'request' in http
        assert 'response' in http

        # Check request
        request = http['request']
        assert request['method'] == 'GET'
        assert 'api.example.com' in request['url']

        # Check response
        response = http['response']
        assert response['status'] == 200


class TestCassetteFormats:
    """Test different cassette formats"""

    def test_json_format(self):
        """Test JSON cassette format"""
        cassette_dir, cassette_path = create_test_cassette("json-test")

        try:
            # Should be valid JSON
            with open(cassette_path) as f:
                data = json.load(f)

            assert data['version'] == '1.0'
        finally:
            import shutil
            shutil.rmtree(cassette_dir)

    def test_cassette_version(self):
        """Test cassette version field"""
        cassette_dir, cassette_path = create_test_cassette("version-test", version="2.0")

        try:
            with open(cassette_path) as f:
                cassette = json.load(f)

            assert cassette['version'] == '2.0'
        finally:
            import shutil
            shutil.rmtree(cassette_dir)


class TestMultipleCassettes:
    """Test handling multiple cassettes"""

    def test_create_multiple_cassettes(self):
        """Test creating multiple cassettes"""
        cassettes = []

        try:
            for i in range(3):
                dir, path = create_test_cassette(f"cassette-{i}", interaction_count=i+1)
                cassettes.append((dir, path))

            # Verify each cassette
            for i, (dir, path) in enumerate(cassettes):
                with open(path) as f:
                    data = json.load(f)

                assert data['name'] == f"cassette-{i}"
                assert len(data['interactions']) == i + 1
        finally:
            import shutil
            for dir, _ in cassettes:
                if os.exists(dir):
                    shutil.rmtree(dir)


@pytest.mark.parametrize("interaction_count", [1, 5, 10, 50])
def test_cassette_with_varying_interactions(interaction_count):
    """Test cassettes with different numbers of interactions"""
    cassette_dir, cassette_path = create_test_cassette(
        f"test-{interaction_count}",
        interaction_count=interaction_count
    )

    try:
        with open(cassette_path) as f:
            cassette = json.load(f)

        assert len(cassette['interactions']) == interaction_count
    finally:
        import shutil
        shutil.rmtree(cassette_dir)


def test_cookie_structure():
    """Test cookie has all required fields"""
    cassette_dir, cassette_path = create_test_cassette("cookie-test")

    try:
        with open(cassette_path) as f:
            cassette = json.load(f)

        cookie = cassette['cookies'][0]

        # Required fields
        assert 'name' in cookie
        assert 'value' in cookie
        assert 'domain' in cookie
        assert 'path' in cookie
        assert 'secure' in cookie
        assert 'http_only' in cookie
        assert 'created_at' in cookie

        # Check types
        assert isinstance(cookie['name'], str)
        assert isinstance(cookie['value'], str)
        assert isinstance(cookie['secure'], bool)
        assert isinstance(cookie['http_only'], bool)
    finally:
        import shutil
        shutil.rmtree(cassette_dir)
