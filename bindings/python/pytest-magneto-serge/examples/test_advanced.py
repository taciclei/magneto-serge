"""
Advanced examples for pytest-magneto-serge
"""

import pytest
import requests
from pytest_magneto_serge import magneto_cassette, use_cassette


# Using decorator instead of marker
@magneto_cassette('decorator_example')
def test_with_decorator():
    """Using @magneto_cassette decorator"""
    response = requests.get('https://jsonplaceholder.typicode.com/users/1')
    user = response.json()

    assert response.status_code == 200
    assert user['id'] == 1


# Manual cassette control with context manager
def test_with_context_manager():
    """Using use_cassette context manager"""
    with use_cassette('context_manager_test'):
        response = requests.get('https://jsonplaceholder.typicode.com/posts')
        posts = response.json()

        assert response.status_code == 200
        assert len(posts) == 100


# Multiple cassettes in one test
def test_multiple_cassettes():
    """Using multiple cassettes sequentially"""
    # First cassette
    with use_cassette('users_cassette'):
        response = requests.get('https://jsonplaceholder.typicode.com/users/1')
        user = response.json()
        assert user['id'] == 1

    # Second cassette
    with use_cassette('posts_cassette'):
        response = requests.get('https://jsonplaceholder.typicode.com/posts/1')
        post = response.json()
        assert post['id'] == 1

    # Third cassette with options
    with use_cassette('comments_cassette', record='all'):
        response = requests.get('https://jsonplaceholder.typicode.com/comments/1')
        comment = response.json()
        assert comment['id'] == 1


# Nested cassettes
def test_nested_cassettes():
    """Nested cassette usage"""
    with use_cassette('outer_cassette'):
        users_response = requests.get('https://jsonplaceholder.typicode.com/users/1')
        user = users_response.json()
        assert user['id'] == 1

        with use_cassette('inner_cassette'):
            posts_response = requests.get(f'https://jsonplaceholder.typicode.com/posts?userId={user["id"]}')
            posts = posts_response.json()
            assert isinstance(posts, list)
            assert len(posts) > 0


# Using fixture for manual control
@pytest.mark.magneto_cassette('fixture_test')
def test_with_fixture(magneto_proxy):
    """Using magneto_proxy fixture"""
    # Proxy already configured by marker
    response = requests.get('https://jsonplaceholder.typicode.com/users')
    users = response.json()

    assert response.status_code == 200
    assert len(users) > 0


# Manual fixture control without marker
def test_manual_fixture_control(magneto_proxy):
    """Manual control with fixture"""
    magneto_proxy.auto('manual_control')

    response = requests.get('https://jsonplaceholder.typicode.com/posts/1')
    post = response.json()

    assert response.status_code == 200
    assert post['id'] == 1

    magneto_proxy.stop()


# Complex workflow
@pytest.mark.magneto_cassette('complex_workflow')
def test_complex_workflow():
    """Multi-step API workflow"""
    # 1. Fetch user
    user_response = requests.get('https://jsonplaceholder.typicode.com/users/1')
    user = user_response.json()
    assert user['id'] == 1

    # 2. Fetch user's posts
    posts_response = requests.get(f'https://jsonplaceholder.typicode.com/posts?userId={user["id"]}')
    posts = posts_response.json()
    assert len(posts) > 0

    # 3. Fetch comments on first post
    comments_response = requests.get(f'https://jsonplaceholder.typicode.com/posts/{posts[0]["id"]}/comments')
    comments = comments_response.json()
    assert len(comments) > 0


# Different recording modes
class TestRecordingModes:
    """Testing different recording modes"""

    @pytest.mark.magneto_cassette(mode='auto')
    def test_auto_mode(self):
        """Auto mode - records if missing, replays if exists"""
        response = requests.get('https://jsonplaceholder.typicode.com/posts/1')
        assert response.status_code == 200

    @pytest.mark.magneto_cassette(record='all')
    def test_force_record(self):
        """Force re-recording"""
        response = requests.get('https://jsonplaceholder.typicode.com/posts/2')
        assert response.status_code == 200

    @pytest.mark.magneto_cassette(record='none')
    def test_replay_only(self):
        """Replay only, errors if cassette missing"""
        response = requests.get('https://jsonplaceholder.typicode.com/posts/3')
        assert response.status_code == 200

    @pytest.mark.magneto_cassette(mode='passthrough')
    def test_passthrough(self):
        """Passthrough mode - no recording/replay"""
        response = requests.get('https://jsonplaceholder.typicode.com/posts/4')
        assert response.status_code == 200


# Headers and authentication
@pytest.mark.magneto_cassette('authenticated_request')
def test_with_authentication():
    """Test with authentication headers (filtered in cassette)"""
    headers = {
        'Authorization': 'Bearer fake-token-for-testing',
        'X-API-Key': 'fake-api-key',
        'Accept': 'application/json',
    }

    response = requests.get('https://jsonplaceholder.typicode.com/posts/1', headers=headers)

    # Authorization and X-API-Key headers will be filtered in cassette
    assert response.status_code == 200


# Custom headers
@pytest.mark.magneto_cassette('custom_headers')
def test_with_custom_headers():
    """Test with custom headers"""
    headers = {
        'User-Agent': 'pytest-magneto-serge-test/1.0',
        'Accept': 'application/json',
        'X-Custom-Header': 'test-value',
    }

    response = requests.get('https://jsonplaceholder.typicode.com/users', headers=headers)
    assert response.status_code == 200


# Query parameters
@pytest.mark.magneto_cassette('query_params')
def test_with_query_parameters():
    """Test with query parameters"""
    params = {
        'userId': 1,
        '_limit': 5,
    }

    response = requests.get('https://jsonplaceholder.typicode.com/posts', params=params)
    posts = response.json()

    assert response.status_code == 200
    assert len(posts) <= 5
    assert all(post['userId'] == 1 for post in posts)


# Session usage
@pytest.mark.magneto_cassette('session_test')
def test_with_session():
    """Test with requests.Session"""
    session = requests.Session()
    session.headers.update({'User-Agent': 'test-agent'})

    response1 = session.get('https://jsonplaceholder.typicode.com/users/1')
    assert response1.status_code == 200

    response2 = session.get('https://jsonplaceholder.typicode.com/posts/1')
    assert response2.status_code == 200


# Error scenarios
class TestErrorScenarios:
    """Testing error scenarios"""

    @pytest.mark.magneto_cassette('404_error')
    def test_handles_404(self):
        response = requests.get('https://jsonplaceholder.typicode.com/posts/999999')
        assert response.status_code == 404

    @pytest.mark.magneto_cassette('500_error')
    def test_handles_500(self):
        response = requests.get('https://httpstat.us/500')
        assert response.status_code == 500

    @pytest.mark.magneto_cassette('network_error')
    def test_handles_network_error(self):
        """Network errors are also recorded"""
        try:
            response = requests.get('https://jsonplaceholder.typicode.com/invalid')
            assert response.status_code in [200, 404]
        except requests.RequestException:
            # Network errors are acceptable in tests
            pass
