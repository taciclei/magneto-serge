<?php

declare(strict_types=1);

namespace MagnetoSerge\PHPUnit\Examples;

use MagnetoSerge\PHPUnit\MagnetoTestCase;
use MagnetoSerge\PHPUnit\Cassette;

/**
 * Advanced examples with cURL, nested cassettes, and complex workflows
 */
class AdvancedExample extends MagnetoTestCase
{
    protected string $cassetteDir = 'examples/cassettes';
    protected string $recordMode = 'new_episodes';

    /**
     * Using cURL
     */
    #[Cassette('curl_example')]
    public function testWithCurl(): void
    {
        $ch = curl_init('https://api.github.com/users/octocat');
        curl_setopt_array($ch, [
            CURLOPT_RETURNTRANSFER => true,
            CURLOPT_HTTPHEADER => [
                'User-Agent: magneto-serge-test',
                'Accept: application/json',
            ],
        ]);

        $response = curl_exec($ch);
        $statusCode = curl_getinfo($ch, CURLINFO_HTTP_CODE);
        $contentType = curl_getinfo($ch, CURLINFO_CONTENT_TYPE);
        curl_close($ch);

        $this->assertEquals(200, $statusCode);
        $this->assertStringContainsString('application/json', $contentType);

        $user = json_decode($response, true);
        $this->assertEquals('octocat', $user['login']);
    }

    /**
     * POST request with cURL
     */
    #[Cassette('curl_post')]
    public function testPostWithCurl(): void
    {
        $data = [
            'title' => 'Test Post',
            'body' => 'This is a test',
            'userId' => 1,
        ];

        $ch = curl_init('https://jsonplaceholder.typicode.com/posts');
        curl_setopt_array($ch, [
            CURLOPT_RETURNTRANSFER => true,
            CURLOPT_POST => true,
            CURLOPT_POSTFIELDS => json_encode($data),
            CURLOPT_HTTPHEADER => [
                'Content-Type: application/json',
            ],
        ]);

        $response = curl_exec($ch);
        $statusCode = curl_getinfo($ch, CURLINFO_HTTP_CODE);
        curl_close($ch);

        $this->assertEquals(201, $statusCode);

        $post = json_decode($response, true);
        $this->assertEquals('Test Post', $post['title']);
    }

    /**
     * Nested cassettes for complex workflows
     */
    public function testNestedCassettes(): void
    {
        $this->useCassette('user_workflow', function() {
            // Fetch user
            $userResponse = file_get_contents('https://jsonplaceholder.typicode.com/users/1');
            $user = json_decode($userResponse, true);
            $this->assertEquals(1, $user['id']);

            // Fetch user's posts in nested cassette
            $this->useCassette('user_posts', function() use ($user) {
                $postsUrl = 'https://jsonplaceholder.typicode.com/posts?userId=' . $user['id'];
                $postsResponse = file_get_contents($postsUrl);
                $posts = json_decode($postsResponse, true);

                $this->assertIsArray($posts);
                $this->assertNotEmpty($posts);

                // Fetch comments on first post
                if (!empty($posts)) {
                    $this->useCassette('post_comments', function() use ($posts) {
                        $commentsUrl = 'https://jsonplaceholder.typicode.com/posts/' . $posts[0]['id'] . '/comments';
                        $commentsResponse = file_get_contents($commentsUrl);
                        $comments = json_decode($commentsResponse, true);

                        $this->assertIsArray($comments);
                        $this->assertNotEmpty($comments);
                    });
                }
            });
        });
    }

    /**
     * Different HTTP methods
     */
    #[Cassette('http_methods')]
    public function testDifferentHttpMethods(): void
    {
        // GET
        $response = file_get_contents('https://jsonplaceholder.typicode.com/posts/1');
        $this->assertNotEmpty($response);

        // POST
        $context = stream_context_create([
            'http' => [
                'method' => 'POST',
                'header' => ['Content-Type: application/json'],
                'content' => json_encode(['title' => 'Test', 'body' => 'Content', 'userId' => 1]),
            ],
        ]);
        $response = file_get_contents('https://jsonplaceholder.typicode.com/posts', false, $context);
        $this->assertNotEmpty($response);

        // PUT
        $context = stream_context_create([
            'http' => [
                'method' => 'PUT',
                'header' => ['Content-Type: application/json'],
                'content' => json_encode(['id' => 1, 'title' => 'Updated', 'body' => 'Updated', 'userId' => 1]),
            ],
        ]);
        $response = file_get_contents('https://jsonplaceholder.typicode.com/posts/1', false, $context);
        $this->assertNotEmpty($response);

        // DELETE
        $context = stream_context_create([
            'http' => ['method' => 'DELETE'],
        ]);
        $response = file_get_contents('https://jsonplaceholder.typicode.com/posts/1', false, $context);
        $this->assertNotEmpty($response);
    }

    /**
     * Authentication headers (filtered in cassette)
     */
    #[Cassette('authenticated_request')]
    public function testWithAuthentication(): void
    {
        $context = stream_context_create([
            'http' => [
                'header' => [
                    'Authorization: Bearer fake-token-for-testing',
                    'X-API-Key: fake-api-key',
                    'Accept: application/json',
                ],
            ],
        ]);

        $response = @file_get_contents('https://jsonplaceholder.typicode.com/posts/1', false, $context);

        // Authorization and X-API-Key headers will be filtered in cassette
        $this->assertNotEmpty($response);
    }

    /**
     * Custom cassette directory per test
     */
    #[Cassette('custom_dir_test')]
    public function testCustomDirectory(): void
    {
        // Uses the configured cassetteDir: examples/cassettes
        $response = file_get_contents('https://jsonplaceholder.typicode.com/users/1');
        $this->assertNotEmpty($response);
    }

    /**
     * Testing with query parameters
     */
    #[Cassette('query_params')]
    public function testWithQueryParameters(): void
    {
        $params = http_build_query([
            'userId' => 1,
            '_limit' => 5,
        ]);

        $url = 'https://jsonplaceholder.typicode.com/posts?' . $params;
        $response = file_get_contents($url);
        $posts = json_decode($response, true);

        $this->assertIsArray($posts);
        $this->assertCount(5, $posts);
        $this->assertTrue(array_all($posts, fn($post) => $post['userId'] === 1));
    }

    /**
     * Parallel requests (all recorded in same cassette)
     */
    #[Cassette('parallel_requests')]
    public function testParallelRequests(): void
    {
        // Initialize multiple cURL handles
        $mh = curl_multi_init();
        $handles = [];

        $urls = [
            'https://jsonplaceholder.typicode.com/users/1',
            'https://jsonplaceholder.typicode.com/posts/1',
            'https://jsonplaceholder.typicode.com/comments/1',
        ];

        foreach ($urls as $url) {
            $ch = curl_init($url);
            curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
            curl_multi_add_handle($mh, $ch);
            $handles[] = $ch;
        }

        // Execute all queries simultaneously
        $active = null;
        do {
            $status = curl_multi_exec($mh, $active);
            if ($active) {
                curl_multi_select($mh);
            }
        } while ($active && $status === CURLM_OK);

        // Get responses
        $responses = [];
        foreach ($handles as $ch) {
            $responses[] = curl_multi_getcontent($ch);
            curl_multi_remove_handle($mh, $ch);
            curl_close($ch);
        }
        curl_multi_close($mh);

        $this->assertCount(3, $responses);
        foreach ($responses as $response) {
            $this->assertNotEmpty($response);
        }
    }

    /**
     * Error scenarios
     */
    #[Cassette('error_scenarios')]
    public function testErrorScenarios(): void
    {
        $context = stream_context_create([
            'http' => ['ignore_errors' => true],
        ]);

        // 404 error
        $response404 = @file_get_contents(
            'https://jsonplaceholder.typicode.com/posts/999999',
            false,
            $context
        );
        $this->assertNotEmpty($http_response_header);

        // 500 error
        $response500 = @file_get_contents('https://httpstat.us/500', false, $context);
        $this->assertNotEmpty($http_response_header);
    }
}
