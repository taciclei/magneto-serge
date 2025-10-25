"""
Example tests using Magnéto-Serge pytest plugin
"""

import pytest
import requests
from magneto_pytest import (
    assert_matches_cassette,
    assert_cassette_status,
    assert_cassette_body,
    assert_interaction_count,
    assert_has_cookies,
    assert_has_cookie,
    assert_cassette_version,
)


@pytest.fixture(scope="session")
def http_client():
    """Configure HTTP client to use Magnéto proxy"""
    session = requests.Session()
    session.proxies = {
        'http': 'http://localhost:8888',
        'https': 'http://localhost:8888',
    }
    return session


def test_user_login(http_client):
    """Test user login matches cassette"""
    response = http_client.post(
        'http://localhost:8080/api/authenticate',
        json={'username': 'admin', 'password': 'admin'}
    )

    # Assert response matches cassette
    assert_matches_cassette(response, 'user-login')


def test_user_account(http_client):
    """Test user account status matches cassette"""
    response = http_client.get('http://localhost:8080/api/account')

    # Assert status matches cassette
    assert_cassette_status(response, 'user-account', 200)


def test_user_list_body(http_client):
    """Test user list body matches cassette"""
    response = http_client.get('http://localhost:8080/api/users')

    # Assert body matches cassette
    assert_cassette_body(response, 'user-list')


def test_cassette_interaction_count():
    """Test cassette has correct number of interactions"""
    assert_interaction_count('user-login', 3)


def test_cassette_cookies():
    """Test cassette contains cookies"""
    assert_has_cookies('user-login')


def test_session_cookie():
    """Test cassette has session cookie"""
    assert_has_cookie('user-login', 'JSESSIONID')


def test_cassette_version():
    """Test cassette version"""
    assert_cassette_version('user-login', '2.0')


@pytest.mark.cassette('user-login')
def test_with_cassette_marker(http_client, cassette):
    """Test using cassette marker and fixture"""
    cass = cassette.load('user-login')

    assert cass['version'] == '2.0'
    assert len(cass['interactions']) > 0


class TestUserAPI:
    """Test class for user API"""

    def test_login(self, http_client):
        response = http_client.post(
            'http://localhost:8080/api/authenticate',
            json={'username': 'admin', 'password': 'admin'}
        )

        assert_matches_cassette(response, 'user-login')
        assert_has_cookie('user-login', 'JSESSIONID')

    def test_account(self, http_client):
        response = http_client.get('http://localhost:8080/api/account')

        assert response.status_code == 200
        assert_cassette_status(response, 'user-account', 200)
