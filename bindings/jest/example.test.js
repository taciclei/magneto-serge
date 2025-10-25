/**
 * Example tests using Magnéto-Serge Jest matchers
 */

import '@magneto-serge/jest-matchers';
import axios from 'axios';

describe('User API with Magnéto-Serge', () => {
  beforeAll(() => {
    // Configure axios to use Magnéto proxy
    axios.defaults.proxy = {
      host: 'localhost',
      port: 8888,
    };
  });

  test('should match login response', async () => {
    const response = await axios.post('/api/authenticate', {
      username: 'admin',
      password: 'admin',
    });

    // Assert response matches cassette
    expect(response).toMatchCassette('user-login');
  });

  test('should have correct status code', async () => {
    const response = await axios.get('/api/account');

    // Assert status matches cassette
    expect(response).toMatchCassetteStatus('user-account', 200);
  });

  test('should match response body', async () => {
    const response = await axios.get('/api/users');

    // Assert body matches cassette
    expect(response).toMatchCassetteBody('user-list');
  });

  test('cassette should have correct interaction count', () => {
    // Assert cassette has expected number of interactions
    expect('user-login').toHaveInteractionCount(3);
  });

  test('cassette should have cookies', () => {
    // Assert cassette contains cookies
    expect('user-login').toHaveCookies();
  });

  test('cassette should have session cookie', () => {
    // Assert cassette has specific cookie
    expect('user-login').toHaveCookie('JSESSIONID');
  });

  test('cassette should be version 2.0', () => {
    // Assert cassette version
    expect('user-login').toHaveCassetteVersion('2.0');
  });
});

describe('Advanced matchers', () => {
  test('should chain matchers', async () => {
    const response = await axios.get('/api/users/1');

    expect(response).toMatchCassette('user-detail');
    expect(response).toMatchCassetteStatus('user-detail', 200);
    expect(response).toMatchCassetteBody('user-detail');
  });

  test('should work with fetch API', async () => {
    const response = await fetch('http://localhost:8888/api/users');
    const data = await response.json();

    expect({ data, status: response.status }).toMatchCassette('user-list');
  });
});
