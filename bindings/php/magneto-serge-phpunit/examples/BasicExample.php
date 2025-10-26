<?php

declare(strict_types=1);

namespace MagnetoSerge\PHPUnit\Examples;

use MagnetoSerge\PHPUnit\MagnetoTestCase;
use MagnetoSerge\PHPUnit\Cassette;

/**
 * Basic examples of using Magneto-Serge with PHPUnit
 */
class BasicExample extends MagnetoTestCase
{
    protected string $cassetteDir = 'examples/cassettes';

    /**
     * Auto-generated cassette name from test method
     */
    #[Cassette]
    public function testFetchJsonPlaceholderUsers(): void
    {
        // Cassette: examples/cassettes/BasicExample/fetch_json_placeholder_users.json
        $response = file_get_contents('https://jsonplaceholder.typicode.com/users');
        $users = json_decode($response, true);

        $this->assertIsArray($users);
        $this->assertNotEmpty($users);
    }

    /**
     * Custom cassette name
     */
    #[Cassette('github_octocat')]
    public function testFetchGitHubUser(): void
    {
        // Cassette: examples/cassettes/github_octocat.json
        $context = stream_context_create([
            'http' => [
                'header' => ['User-Agent: magneto-serge-test'],
            ],
        ]);

        $response = file_get_contents('https://api.github.com/users/octocat', false, $context);
        $user = json_decode($response, true);

        $this->assertEquals('octocat', $user['login']);
        $this->assertEquals('User', $user['type']);
    }

    /**
     * Force re-recording
     */
    #[Cassette('live_data', record: 'all')]
    public function testForceRecording(): void
    {
        // Always re-records this cassette
        $response = file_get_contents('https://jsonplaceholder.typicode.com/posts/1');
        $post = json_decode($response, true);

        $this->assertEquals(1, $post['id']);
    }

    /**
     * Replay-only mode (strict)
     */
    #[Cassette('cached_data', record: 'none')]
    public function testReplayOnly(): void
    {
        // Only replays, errors if cassette missing
        $response = file_get_contents('https://jsonplaceholder.typicode.com/posts/2');
        $post = json_decode($response, true);

        $this->assertEquals(2, $post['id']);
    }

    /**
     * Manual cassette control
     */
    public function testManualCassetteControl(): void
    {
        $this->useCassette('manual_test', function() {
            $response = file_get_contents('https://jsonplaceholder.typicode.com/comments/1');
            $comment = json_decode($response, true);

            $this->assertEquals(1, $comment['id']);
            $this->assertEquals(1, $comment['postId']);
        });
    }

    /**
     * Multiple cassettes in one test
     */
    public function testMultipleCassettes(): void
    {
        // First cassette
        $this->useCassette('users', function() {
            $response = file_get_contents('https://jsonplaceholder.typicode.com/users/1');
            $user = json_decode($response, true);
            $this->assertEquals(1, $user['id']);
        });

        // Second cassette
        $this->useCassette('posts', function() {
            $response = file_get_contents('https://jsonplaceholder.typicode.com/posts/1');
            $post = json_decode($response, true);
            $this->assertEquals(1, $post['id']);
        });
    }

    /**
     * Error handling
     */
    #[Cassette('404_error')]
    public function testHandles404Error(): void
    {
        $context = stream_context_create([
            'http' => [
                'ignore_errors' => true,
            ],
        ]);

        $response = @file_get_contents(
            'https://jsonplaceholder.typicode.com/posts/999999',
            false,
            $context
        );

        // 404 responses are recorded too
        $this->assertNotEmpty($http_response_header);
    }
}
