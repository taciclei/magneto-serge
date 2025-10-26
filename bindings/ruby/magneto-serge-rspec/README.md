# magneto-serge-rspec

RSpec integration for Magnéto-Serge, providing VCR-like automatic cassette management for HTTP/WebSocket recording and replay.

## Installation

Add this line to your application's Gemfile:

```ruby
gem 'magneto-serge-rspec'
```

And then execute:

    $ bundle install

Or install it yourself as:

    $ gem install magneto-serge-rspec

## Usage

### Basic Setup

Configure Magneto::Serge::RSpec in your `spec/spec_helper.rb`:

```ruby
require 'magneto/serge/rspec'

Magneto::Serge::RSpec.configure do |config|
  # Directory where cassettes are stored
  config.cassette_library_dir = 'spec/fixtures/cassettes'

  # Default cassette options
  config.default_cassette_options = {
    record: :new_episodes,  # :new_episodes, :once, :all, :none
    mode: :auto,            # :auto, :record, :replay, :passthrough
    match_requests_on: [:method, :uri, :body]
  }

  # Proxy port
  config.proxy_port = 8888

  # Filter sensitive headers
  config.filter_sensitive_headers = %w[Authorization Cookie Set-Cookie X-API-Key]

  # Strict matching mode
  config.strict_matching = false
end
```

### Automatic Cassette Management

Use RSpec metadata to automatically manage cassettes:

```ruby
# Auto-generated cassette name from example description
RSpec.describe 'GitHub API', :magneto do
  it 'fetches user info' do
    # Cassette: spec/fixtures/cassettes/GitHub_API/fetches_user_info.json
    response = HTTP.get('https://api.github.com/users/octocat')
    expect(response.status).to eq(200)
  end
end

# Explicit cassette name
RSpec.describe 'API Client' do
  it 'handles rate limiting', cassette: 'rate_limit_429' do
    # Cassette: spec/fixtures/cassettes/rate_limit_429.json
    response = HTTP.get('https://api.example.com/data')
    expect(response.status).to eq(429)
  end
end

# Cassette with options
RSpec.describe 'Live API Tests' do
  it 'records all requests', magneto: { record: :all } do
    # Forces re-recording of cassette
    response = HTTP.get('https://api.example.com/live')
    expect(response).to be_ok
  end
end
```

### Manual Cassette Control

Use `use_cassette` for fine-grained control:

```ruby
RSpec.describe 'Weather API' do
  it 'fetches forecast' do
    use_cassette('weather/forecast', record: :new_episodes) do
      response = HTTP.get('https://api.weather.com/forecast')
      expect(response.body['temperature']).to be > 0
    end
  end

  it 'handles multiple cassettes' do
    use_cassette('weather/current') do
      current = HTTP.get('https://api.weather.com/current')

      use_cassette('weather/forecast') do
        forecast = HTTP.get('https://api.weather.com/forecast')

        expect(forecast.body['temp']).to be >= current.body['temp']
      end
    end
  end
end
```

### Record Modes

- **`:new_episodes`** (default) - Record new interactions, replay existing ones (maps to `:auto` mode)
- **`:once`** - Record only if cassette doesn't exist (maps to `:replay` mode)
- **`:all`** - Always record, overwrite cassette (maps to `:record` mode)
- **`:none`** - Never record, only replay (maps to `:replay` mode)

```ruby
# Re-record a cassette
it 'updates cassette', magneto: { record: :all } do
  # Forces re-recording
end

# Use existing cassette only
it 'replays only', magneto: { record: :none } do
  # Errors if cassette missing
end
```

### Cassette Options

```ruby
Magneto::Serge::RSpec.configure do |config|
  config.default_cassette_options = {
    # Record mode
    record: :new_episodes,  # :new_episodes, :once, :all, :none

    # Proxy mode (overrides record mode translation)
    mode: :auto,            # :auto, :record, :replay, :passthrough

    # Request matching criteria
    match_requests_on: [:method, :uri, :body],

    # Custom port for this cassette
    port: 8888
  }
end
```

### Custom Cassette Names

Override the default cassette name generator:

```ruby
Magneto::Serge::RSpec.configure do |config|
  config.cassette_name_generator = ->(metadata) {
    # Use full example description path
    parts = []
    group = metadata[:example_group]
    while group
      parts.unshift(group[:description]) if group[:description]
      group = group[:parent_example_group]
    end
    parts << metadata[:description]

    # Join with underscores
    parts.join('_').gsub(/[^\w\-]/, '_').downcase
  }
end
```

### Accessing Current Cassette

```ruby
RSpec.describe 'API', :magneto do
  it 'checks cassette name' do
    expect(current_cassette).to eq('API/checks_cassette_name')
  end
end
```

## Examples

### Basic HTTP Recording

```ruby
require 'spec_helper'
require 'http'

RSpec.describe 'JSONPlaceholder API', :magneto do
  it 'fetches posts' do
    response = HTTP.get('https://jsonplaceholder.typicode.com/posts/1')
    json = JSON.parse(response.body)

    expect(json['userId']).to eq(1)
    expect(json['title']).to be_a(String)
  end

  it 'creates a post', magneto: { record: :all } do
    response = HTTP.post(
      'https://jsonplaceholder.typicode.com/posts',
      json: { title: 'Test', body: 'Content', userId: 1 }
    )

    expect(response.status).to eq(201)
  end
end
```

### Nested Contexts

```ruby
RSpec.describe 'GitHub API', :magneto do
  context 'users' do
    it 'fetches user profile' do
      # Cassette: GitHub_API/users/fetches_user_profile.json
      response = HTTP.get('https://api.github.com/users/octocat')
      expect(response.status).to eq(200)
    end
  end

  context 'repositories' do
    it 'lists repos', cassette: 'octocat_repos' do
      # Cassette: octocat_repos.json
      response = HTTP.get('https://api.github.com/users/octocat/repos')
      expect(response.status).to eq(200)
    end
  end
end
```

### Error Handling

```ruby
RSpec.describe 'API Errors', :magneto do
  it 'handles 404 errors' do
    response = HTTP.get('https://api.example.com/nonexistent')
    expect(response.status).to eq(404)
  end

  it 'handles network timeouts' do
    expect {
      HTTP.timeout(1).get('https://httpstat.us/524?sleep=5000')
    }.to raise_error(HTTP::TimeoutError)
  end
end
```

### WebSocket Recording

```ruby
require 'faye/websocket'
require 'eventmachine'

RSpec.describe 'WebSocket Echo', :magneto do
  it 'records websocket messages' do
    messages = []

    EM.run do
      ws = Faye::WebSocket::Client.new('wss://echo.websocket.org/')

      ws.on :open do |event|
        ws.send('Hello, WebSocket!')
      end

      ws.on :message do |event|
        messages << event.data
        ws.close
      end

      ws.on :close do |event|
        EM.stop
      end
    end

    expect(messages).to include('Hello, WebSocket!')
  end
end
```

## Configuration Reference

### Global Configuration

```ruby
Magneto::Serge::RSpec.configure do |config|
  # Cassette storage directory
  config.cassette_library_dir = 'spec/fixtures/cassettes'

  # Default cassette options
  config.default_cassette_options = {
    record: :new_episodes,
    mode: :auto,
    match_requests_on: [:method, :uri, :body]
  }

  # Proxy port
  config.proxy_port = 8888

  # HTTP library hooks (future feature)
  config.hook_into = []  # [:webmock, :faraday, :httpclient]

  # Allow HTTP when no cassette
  config.allow_http_connections_when_no_cassette = false

  # Sensitive headers to filter
  config.filter_sensitive_headers = %w[
    Authorization
    Cookie
    Set-Cookie
    X-API-Key
  ]

  # Strict matching mode
  config.strict_matching = false

  # Custom cassette name generator
  config.cassette_name_generator = ->(metadata) {
    # Your custom logic here
  }
end
```

### Metadata Options

```ruby
# Boolean flag - use auto-generated cassette name
it 'test', :magneto do
end

# String - explicit cassette name
it 'test', cassette: 'my_cassette' do
end

# Hash - cassette name and options
it 'test', cassette: { name: 'my_cassette', record: :all } do
end

# Hash - options only (auto-generated name)
it 'test', magneto: { record: :new_episodes, port: 9999 } do
end
```

## Comparison with VCR

Magneto-Serge RSpec integration provides similar functionality to VCR with these differences:

| Feature | VCR | Magneto-Serge |
|---------|-----|---------------|
| HTTP Recording | ✅ | ✅ |
| WebSocket Recording | ❌ | ✅ |
| Performance | ~1000 req/s | ~5000+ req/s |
| Language Support | Ruby only | Ruby, JS, Python, Java, etc. |
| RSpec Integration | ✅ | ✅ |
| Metadata API | ✅ | ✅ |
| Record Modes | ✅ | ✅ (translated) |
| Cassette Format | YAML | JSON/MessagePack |
| Hook System | ✅ | ✅ (Rust-based) |

## Migration from VCR

Minimal changes required:

```ruby
# Before (VCR)
require 'vcr'

VCR.configure do |config|
  config.cassette_library_dir = 'spec/cassettes'
  config.hook_into :webmock
end

RSpec.describe 'API', :vcr do
  it 'works' do
    # ...
  end
end

# After (Magneto-Serge)
require 'magneto/serge/rspec'

Magneto::Serge::RSpec.configure do |config|
  config.cassette_library_dir = 'spec/cassettes'
  # hook_into not yet needed
end

RSpec.describe 'API', :magneto do
  it 'works' do
    # Same test code!
  end
end
```

## Development

After checking out the repo, run `bundle install` to install dependencies.

Run tests with:

    $ bundle exec rspec

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/taciclei/magneto-serge.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
