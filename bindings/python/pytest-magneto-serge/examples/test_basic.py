"""
Basic examples for pytest-magneto-serge
"""

import pytest
import requests


# Auto-generated cassette name from test name
@pytest.mark.magneto_cassette
def test_fetch_jsonplaceholder_users():
    """Cassette: test_basic/fetch_jsonplaceholder_users.json"""
    response = requests.get('https://jsonplaceholder.typicode.com/users')
    users = response.json()

    assert response.status_code == 200
    assert isinstance(users, list)
    assert len(users) > 0


# Custom cassette name
@pytest.mark.magneto_cassette('github_octocat')
def test_fetch_github_user():
    """Cassette: github_octocat.json"""
    response = requests.get('https://api.github.com/users/octocat')
    user = response.json()

    assert response.status_code == 200
    assert user['login'] == 'octocat'
    assert user['type'] == 'User'


# Force re-recording
@pytest.mark.magneto_cassette('live_data', record='all')
def test_force_recording():
    """Always re-records this cassette"""
    response = requests.get('https://jsonplaceholder.typicode.com/posts/1')
    post = response.json()

    assert response.status_code == 200
    assert post['id'] == 1


# Replay-only mode
@pytest.mark.magneto_cassette('cached_data', record='none')
def test_replay_only():
    """Only replays, errors if cassette missing"""
    response = requests.get('https://jsonplaceholder.typicode.com/posts/2')
    post = response.json()

    assert response.status_code == 200
    assert post['id'] == 2


class TestHTTPMethods:
    """Test different HTTP methods"""

    @pytest.mark.magneto_cassette
    def test_get_request(self):
        response = requests.get('https://jsonplaceholder.typicode.com/posts/1')
        assert response.status_code == 200

    @pytest.mark.magneto_cassette
    def test_post_request(self):
        data = {'title': 'Test Post', 'body': 'Test content', 'userId': 1}
        response = requests.post('https://jsonplaceholder.typicode.com/posts', json=data)
        assert response.status_code == 201

    @pytest.mark.magneto_cassette
    def test_put_request(self):
        data = {'id': 1, 'title': 'Updated', 'body': 'Updated content', 'userId': 1}
        response = requests.put('https://jsonplaceholder.typicode.com/posts/1', json=data)
        assert response.status_code == 200

    @pytest.mark.magneto_cassette
    def test_delete_request(self):
        response = requests.delete('https://jsonplaceholder.typicode.com/posts/1')
        assert response.status_code == 200


# Error handling
@pytest.mark.magneto_cassette('404_error')
def test_handles_404_error():
    """404 responses are recorded too"""
    response = requests.get('https://jsonplaceholder.typicode.com/posts/999999')
    assert response.status_code == 404


# Parametrized tests
@pytest.mark.parametrize('post_id', [1, 2, 3])
@pytest.mark.magneto_cassette
def test_fetch_multiple_posts(post_id):
    """Each parameter gets its own cassette"""
    response = requests.get(f'https://jsonplaceholder.typicode.com/posts/{post_id}')
    post = response.json()
    assert post['id'] == post_id
