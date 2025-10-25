# frozen_string_literal: true

require "rspec"
require "magneto_serge"
require_relative "rspec/version"
require_relative "rspec/configuration"
require_relative "rspec/metadata"
require_relative "rspec/hooks"

module Magneto
  module Serge
    module RSpec
      class Error < StandardError; end

      class << self
        # Global configuration instance
        attr_reader :configuration

        # Configure Magneto::Serge::RSpec
        #
        # @yield [Configuration] configuration object
        # @example
        #   Magneto::Serge::RSpec.configure do |config|
        #     config.cassette_library_dir = "spec/fixtures/cassettes"
        #     config.default_cassette_options = { record: :new_episodes }
        #   end
        def configure
          @configuration ||= Configuration.new
          yield(configuration) if block_given?
          configuration
        end

        # Reset configuration to defaults (mainly for testing)
        def reset_configuration!
          @configuration = Configuration.new
        end
      end
    end
  end
end

# Register RSpec hooks
::RSpec.configure do |config|
  config.include Magneto::Serge::RSpec::Metadata
  config.around(:each) do |example|
    Magneto::Serge::RSpec::Hooks.around_hook(example)
  end
end
