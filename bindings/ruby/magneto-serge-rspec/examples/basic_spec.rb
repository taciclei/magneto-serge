# frozen_string_literal: true

require 'spec_helper'
require 'http'

# Basic example showing auto-generated cassette names from metadata
RSpec.describe 'JSONPlaceholder API', :magneto do
  it 'fetches a single post' do
    # Cassette: spec/fixtures/cassettes/JSONPlaceholder_API/fetches_a_single_post.json
    response = HTTP.get('https://jsonplaceholder.typicode.com/posts/1')
    json = JSON.parse(response.body)

    expect(response.status).to eq(200)
    expect(json['userId']).to eq(1)
    expect(json['id']).to eq(1)
    expect(json['title']).to be_a(String)
    expect(json['body']).to be_a(String)
  end

  it 'fetches all posts' do
    # Cassette: spec/fixtures/cassettes/JSONPlaceholder_API/fetches_all_posts.json
    response = HTTP.get('https://jsonplaceholder.typicode.com/posts')
    json = JSON.parse(response.body)

    expect(response.status).to eq(200)
    expect(json).to be_an(Array)
    expect(json.length).to eq(100)
  end

  it 'creates a new post', magneto: { record: :all } do
    # Cassette: spec/fixtures/cassettes/JSONPlaceholder_API/creates_a_new_post.json
    # Force re-recording with record: :all
    response = HTTP.post(
      'https://jsonplaceholder.typicode.com/posts',
      json: {
        title: 'Test Post',
        body: 'This is a test post',
        userId: 1
      }
    )
    json = JSON.parse(response.body)

    expect(response.status).to eq(201)
    expect(json['id']).to be_a(Integer)
    expect(json['title']).to eq('Test Post')
  end
end

# Example with explicit cassette names
RSpec.describe 'GitHub API' do
  it 'fetches user profile', cassette: 'github_octocat' do
    # Cassette: spec/fixtures/cassettes/github_octocat.json
    response = HTTP.get('https://api.github.com/users/octocat')
    json = JSON.parse(response.body)

    expect(response.status).to eq(200)
    expect(json['login']).to eq('octocat')
    expect(json['type']).to eq('User')
  end

  it 'fetches repository info', cassette: 'github_hello_world' do
    # Cassette: spec/fixtures/cassettes/github_hello_world.json
    response = HTTP.get('https://api.github.com/repos/octocat/Hello-World')
    json = JSON.parse(response.body)

    expect(response.status).to eq(200)
    expect(json['name']).to eq('Hello-World')
    expect(json['owner']['login']).to eq('octocat')
  end
end

# Example with nested contexts
RSpec.describe 'API Testing', :magneto do
  context 'successful requests' do
    it 'returns 200 OK' do
      # Cassette: spec/fixtures/cassettes/API_Testing/successful_requests/returns_200_OK.json
      response = HTTP.get('https://jsonplaceholder.typicode.com/posts/1')
      expect(response.status).to eq(200)
    end
  end

  context 'error handling' do
    it 'handles 404 not found' do
      # Cassette: spec/fixtures/cassettes/API_Testing/error_handling/handles_404_not_found.json
      response = HTTP.get('https://jsonplaceholder.typicode.com/posts/99999')
      expect(response.status).to eq(404)
    end
  end
end
