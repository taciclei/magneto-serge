/**
 * Advanced examples for @magneto-serge/jest
 */

import { magnetoTest, magnetoDescribe, useCassette, configure, getCurrentCassette } from '../src/index';

// Configure for all tests in this file
beforeAll(() => {
  configure({
    cassetteDir: 'examples/__cassettes__',
    mode: 'auto',
    verbose: true,
  });
});

describe('Advanced Features', () => {
  describe('magnetoDescribe - Suite-level cassette management', () => {
    magnetoDescribe('API Tests with shared config', { cassetteDir: 'api_tests' }, () => {
      test('test 1', async () => {
        // Cassette auto-managed in api_tests/ directory
        const response = await fetch('https://jsonplaceholder.typicode.com/posts/1');
        expect(response.status).toBe(200);
      });

      test('test 2', async () => {
        // Separate cassette, same directory
        const response = await fetch('https://jsonplaceholder.typicode.com/posts/2');
        expect(response.status).toBe(200);
      });
    });
  });

  describe('useCassette - Manual control', () => {
    test('single cassette in test', async () => {
      await useCassette('manual_cassette', async () => {
        const response = await fetch('https://jsonplaceholder.typicode.com/users');
        const users = await response.json();
        expect(users).toHaveLength(10);
      });
    });

    test('multiple cassettes in one test', async () => {
      // First cassette
      await useCassette('users_cassette', async () => {
        const response = await fetch('https://jsonplaceholder.typicode.com/users/1');
        const user = await response.json();
        expect(user.id).toBe(1);
      });

      // Second cassette
      await useCassette('posts_cassette', async () => {
        const response = await fetch('https://jsonplaceholder.typicode.com/posts/1');
        const post = await response.json();
        expect(post.id).toBe(1);
      });

      // Third cassette with custom options
      await useCassette({ name: 'comments', mode: 'record' }, async () => {
        const response = await fetch('https://jsonplaceholder.typicode.com/comments/1');
        const comment = await response.json();
        expect(comment.id).toBe(1);
      });
    });

    test('nested cassettes', async () => {
      await useCassette('outer', async () => {
        const response1 = await fetch('https://jsonplaceholder.typicode.com/users/1');
        expect(response1.status).toBe(200);

        await useCassette('inner', async () => {
          const response2 = await fetch('https://jsonplaceholder.typicode.com/posts/1');
          expect(response2.status).toBe(200);
        });
      });
    });
  });

  describe('Recording Modes', () => {
    magnetoTest('auto mode (default)', async () => {
      // Records if missing, replays if exists
      const response = await fetch('https://jsonplaceholder.typicode.com/posts/1');
      expect(response.status).toBe(200);
    });

    magnetoTest('force record mode', { record: 'all' }, async () => {
      // Always re-records, overwrites existing cassette
      const response = await fetch('https://jsonplaceholder.typicode.com/posts/2');
      expect(response.status).toBe(200);
    });

    magnetoTest('replay only mode', { record: 'none' }, async () => {
      // Only replays, never records (errors if cassette missing)
      const response = await fetch('https://jsonplaceholder.typicode.com/posts/3');
      expect(response.status).toBe(200);
    });

    magnetoTest('new episodes mode', { record: 'new_episodes' }, async () => {
      // Same as auto mode
      const response = await fetch('https://jsonplaceholder.typicode.com/posts/4');
      expect(response.status).toBe(200);
    });

    magnetoTest('passthrough mode', { mode: 'passthrough' }, async () => {
      // Direct connection, no recording/replay
      const response = await fetch('https://jsonplaceholder.typicode.com/posts/5');
      expect(response.status).toBe(200);
    });
  });

  describe('Custom Configuration per Test', () => {
    magnetoTest('custom port', { port: 9999 }, async () => {
      const response = await fetch('https://jsonplaceholder.typicode.com/users');
      expect(response.status).toBe(200);
    });

    magnetoTest('custom cassette directory', { cassetteDir: 'custom_dir' }, async () => {
      const response = await fetch('https://jsonplaceholder.typicode.com/posts');
      expect(response.status).toBe(200);
    });

    magnetoTest('all custom options', {
      name: 'fully_customized',
      mode: 'record',
      cassetteDir: 'custom_cassettes',
      port: 8080,
    }, async () => {
      const response = await fetch('https://jsonplaceholder.typicode.com/albums');
      expect(response.status).toBe(200);
    });
  });

  describe('getCurrentCassette', () => {
    magnetoTest('checks current cassette name', async () => {
      const cassetteName = getCurrentCassette();
      expect(cassetteName).toBe('checks_current_cassette_name');

      const response = await fetch('https://jsonplaceholder.typicode.com/posts/1');
      expect(response.status).toBe(200);

      // Cassette name still available
      expect(getCurrentCassette()).toBe('checks_current_cassette_name');
    });

    test('returns null when no cassette active', () => {
      expect(getCurrentCassette()).toBeNull();
    });
  });

  describe('Complex Workflows', () => {
    magnetoTest('multi-step API workflow', async () => {
      // 1. Fetch user
      const userResponse = await fetch('https://jsonplaceholder.typicode.com/users/1');
      const user = await userResponse.json();
      expect(user.id).toBe(1);

      // 2. Fetch user's posts
      const postsResponse = await fetch(`https://jsonplaceholder.typicode.com/posts?userId=${user.id}`);
      const posts = await postsResponse.json();
      expect(posts.length).toBeGreaterThan(0);

      // 3. Fetch comments on first post
      const commentsResponse = await fetch(`https://jsonplaceholder.typicode.com/posts/${posts[0].id}/comments`);
      const comments = await commentsResponse.json();
      expect(comments.length).toBeGreaterThan(0);
    });

    magnetoTest('parallel requests', async () => {
      const [users, posts, comments] = await Promise.all([
        fetch('https://jsonplaceholder.typicode.com/users').then(r => r.json()),
        fetch('https://jsonplaceholder.typicode.com/posts').then(r => r.json()),
        fetch('https://jsonplaceholder.typicode.com/comments').then(r => r.json()),
      ]);

      expect(users.length).toBeGreaterThan(0);
      expect(posts.length).toBeGreaterThan(0);
      expect(comments.length).toBeGreaterThan(0);
    });
  });

  describe('Error Scenarios', () => {
    magnetoTest('handles network errors gracefully', async () => {
      // Note: This will record the error response in the cassette
      const response = await fetch('https://httpstat.us/503');
      expect(response.status).toBe(503);
    });

    magnetoTest('handles malformed responses', async () => {
      const response = await fetch('https://httpstat.us/200');
      const text = await response.text();
      expect(response.status).toBe(200);
      expect(text).toBeDefined();
    });
  });

  describe('Headers and Authentication', () => {
    magnetoTest('records custom headers', async () => {
      const response = await fetch('https://jsonplaceholder.typicode.com/posts', {
        headers: {
          'X-Custom-Header': 'test-value',
          'Accept': 'application/json',
        },
      });
      expect(response.status).toBe(200);
    });

    magnetoTest('records bearer token auth', async () => {
      // Note: Sensitive headers should be filtered by the proxy
      const response = await fetch('https://jsonplaceholder.typicode.com/posts', {
        headers: {
          'Authorization': 'Bearer fake-token-for-testing',
        },
      });
      expect(response.status).toBe(200);
    });
  });
});
