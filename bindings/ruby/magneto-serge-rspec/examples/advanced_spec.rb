# frozen_string_literal: true

require 'spec_helper'
require 'http'

# Advanced example showing manual cassette control
RSpec.describe 'Advanced Cassette Control' do
  it 'uses manual cassette management' do
    use_cassette('weather_api', record: :new_episodes) do
      response = HTTP.get('https://api.openweathermap.org/data/2.5/weather?q=London')
      expect(response.status).to be_between(200, 299)
    end
  end

  it 'uses nested cassettes' do
    use_cassette('outer_cassette') do
      response1 = HTTP.get('https://jsonplaceholder.typicode.com/users/1')
      user = JSON.parse(response1.body)

      use_cassette('inner_cassette') do
        response2 = HTTP.get("https://jsonplaceholder.typicode.com/posts?userId=#{user['id']}")
        posts = JSON.parse(response2.body)

        expect(posts).to be_an(Array)
        expect(posts.first['userId']).to eq(user['id'])
      end
    end
  end

  it 'forces re-recording', magneto: { record: :all } do
    # This will always record fresh data, overwriting existing cassette
    response = HTTP.get('https://jsonplaceholder.typicode.com/posts/1')
    expect(response.status).to eq(200)
  end

  it 'uses strict replay mode', magneto: { record: :none } do
    # This will only replay, never record. Fails if cassette missing.
    response = HTTP.get('https://jsonplaceholder.typicode.com/posts/1')
    expect(response.status).to eq(200)
  end
end

# Example with custom configuration per cassette
RSpec.describe 'Custom Cassette Options' do
  it 'uses custom port', magneto: { port: 9999 } do
    # Uses port 9999 instead of default 8888
    response = HTTP.get('https://jsonplaceholder.typicode.com/posts/1')
    expect(response.status).to eq(200)
  end

  it 'uses passthrough mode', magneto: { mode: :passthrough } do
    # Direct connection, no recording
    response = HTTP.get('https://jsonplaceholder.typicode.com/posts/1')
    expect(response.status).to eq(200)
  end
end

# Example showing cassette inspection
RSpec.describe 'Cassette Inspection' do
  it 'accesses current cassette name', :magneto do
    expect(current_cassette).to eq('Cassette_Inspection/accesses_current_cassette_name')

    response = HTTP.get('https://jsonplaceholder.typicode.com/posts/1')
    expect(response.status).to eq(200)

    # Cassette name available throughout test
    expect(current_cassette).to eq('Cassette_Inspection/accesses_current_cassette_name')
  end
end

# Example with authentication headers (filtered automatically)
RSpec.describe 'Authenticated API Calls', :magneto do
  it 'filters sensitive headers' do
    response = HTTP
      .auth('Bearer super-secret-token')
      .get('https://api.example.com/protected')

    # The Authorization header will be filtered in the cassette
    expect(response.status).to be_between(200, 403)
  end

  it 'filters API keys' do
    response = HTTP
      .headers('X-API-Key' => 'my-secret-api-key')
      .get('https://api.example.com/data')

    # The X-API-Key header will be filtered in the cassette
    expect(response.status).to be_between(200, 403)
  end
end

# Example with different HTTP methods
RSpec.describe 'HTTP Methods', :magneto do
  it 'records GET requests' do
    response = HTTP.get('https://jsonplaceholder.typicode.com/posts/1')
    expect(response.status).to eq(200)
  end

  it 'records POST requests' do
    response = HTTP.post(
      'https://jsonplaceholder.typicode.com/posts',
      json: { title: 'Test', body: 'Content', userId: 1 }
    )
    expect(response.status).to eq(201)
  end

  it 'records PUT requests' do
    response = HTTP.put(
      'https://jsonplaceholder.typicode.com/posts/1',
      json: { id: 1, title: 'Updated', body: 'Updated content', userId: 1 }
    )
    expect(response.status).to eq(200)
  end

  it 'records DELETE requests' do
    response = HTTP.delete('https://jsonplaceholder.typicode.com/posts/1')
    expect(response.status).to eq(200)
  end

  it 'records PATCH requests' do
    response = HTTP.patch(
      'https://jsonplaceholder.typicode.com/posts/1',
      json: { title: 'Patched Title' }
    )
    expect(response.status).to eq(200)
  end
end

# Example with query parameters
RSpec.describe 'Query Parameters', :magneto do
  it 'records requests with query params' do
    response = HTTP.get('https://jsonplaceholder.typicode.com/posts', params: { userId: 1 })
    posts = JSON.parse(response.body)

    expect(response.status).to eq(200)
    expect(posts).to be_an(Array)
    expect(posts.all? { |p| p['userId'] == 1 }).to be true
  end

  it 'records requests with multiple params' do
    response = HTTP.get('https://jsonplaceholder.typicode.com/comments', params: {
      postId: 1,
      _limit: 5
    })
    comments = JSON.parse(response.body)

    expect(response.status).to eq(200)
    expect(comments.length).to eq(5)
  end
end
