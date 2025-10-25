/**
 * Basic examples for @magneto-serge/jest
 */

import { magnetoTest } from '../src/index';

describe('Basic HTTP Recording', () => {
  // Auto-generated cassette name from test name
  magnetoTest('fetches users from JSONPlaceholder', async () => {
    // Cassette: __cassettes__/fetches_users_from_jsonplaceholder.json
    const response = await fetch('https://jsonplaceholder.typicode.com/users');
    const users = await response.json();

    expect(response.status).toBe(200);
    expect(Array.isArray(users)).toBe(true);
    expect(users.length).toBeGreaterThan(0);
  });

  // Custom cassette name
  magnetoTest('fetches single post', { name: 'post_1' }, async () => {
    // Cassette: __cassettes__/post_1.json
    const response = await fetch('https://jsonplaceholder.typicode.com/posts/1');
    const post = await response.json();

    expect(response.status).toBe(200);
    expect(post.id).toBe(1);
    expect(post.title).toBeDefined();
  });

  // Force re-recording
  magnetoTest('creates new post', { record: 'all' }, async () => {
    // Always re-records this cassette
    const response = await fetch('https://jsonplaceholder.typicode.com/posts', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        title: 'Test Post',
        body: 'This is a test post',
        userId: 1,
      }),
    });
    const post = await response.json();

    expect(response.status).toBe(201);
    expect(post.id).toBeDefined();
  });
});

describe('Different HTTP Methods', () => {
  magnetoTest('GET request', async () => {
    const response = await fetch('https://jsonplaceholder.typicode.com/posts/1');
    expect(response.status).toBe(200);
  });

  magnetoTest('POST request', async () => {
    const response = await fetch('https://jsonplaceholder.typicode.com/posts', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ title: 'Test', body: 'Content', userId: 1 }),
    });
    expect(response.status).toBe(201);
  });

  magnetoTest('PUT request', async () => {
    const response = await fetch('https://jsonplaceholder.typicode.com/posts/1', {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ id: 1, title: 'Updated', body: 'Updated content', userId: 1 }),
    });
    expect(response.status).toBe(200);
  });

  magnetoTest('DELETE request', async () => {
    const response = await fetch('https://jsonplaceholder.typicode.com/posts/1', {
      method: 'DELETE',
    });
    expect(response.status).toBe(200);
  });

  magnetoTest('PATCH request', async () => {
    const response = await fetch('https://jsonplaceholder.typicode.com/posts/1', {
      method: 'PATCH',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ title: 'Patched Title' }),
    });
    expect(response.status).toBe(200);
  });
});

describe('Nested Describe Blocks', () => {
  describe('GitHub API', () => {
    describe('users', () => {
      magnetoTest('fetches user profile', async () => {
        // Cassette: __cassettes__/fetches_user_profile.json
        const response = await fetch('https://api.github.com/users/octocat');
        const user = await response.json();

        expect(response.status).toBe(200);
        expect(user.login).toBe('octocat');
      });
    });

    describe('repositories', () => {
      magnetoTest('lists user repositories', async () => {
        // Cassette: __cassettes__/lists_user_repositories.json
        const response = await fetch('https://api.github.com/users/octocat/repos');
        const repos = await response.json();

        expect(response.status).toBe(200);
        expect(Array.isArray(repos)).toBe(true);
      });
    });
  });
});

describe('Error Handling', () => {
  magnetoTest('handles 404 errors', async () => {
    const response = await fetch('https://jsonplaceholder.typicode.com/posts/999999');
    expect(response.status).toBe(404);
  });

  magnetoTest('handles 500 errors', async () => {
    const response = await fetch('https://httpstat.us/500');
    expect(response.status).toBe(500);
  });
});
