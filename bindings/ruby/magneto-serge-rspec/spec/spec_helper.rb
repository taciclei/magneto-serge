# frozen_string_literal: true

require 'magneto/serge/rspec'

# Configure Magneto::Serge::RSpec for tests
Magneto::Serge::RSpec.configure do |config|
  config.cassette_library_dir = 'spec/fixtures/cassettes'
  config.default_cassette_options = {
    record: :new_episodes,
    mode: :auto,
    match_requests_on: [:method, :uri, :body]
  }
  config.proxy_port = 8888
  config.filter_sensitive_headers = %w[Authorization Cookie Set-Cookie X-API-Key]
  config.strict_matching = false
end

RSpec.configure do |config|
  # Enable flags like --only-failures and --next-failure
  config.example_status_persistence_file_path = '.rspec_status'

  # Disable RSpec exposing methods globally on `Module` and `main`
  config.disable_monkey_patching!

  config.expect_with :rspec do |c|
    c.syntax = :expect
  end

  # Clean up cassettes after test suite
  config.after(:suite) do
    # Optional: Clean up test cassettes
    # FileUtils.rm_rf('spec/fixtures/cassettes') if ENV['CLEAN_CASSETTES']
  end
end
