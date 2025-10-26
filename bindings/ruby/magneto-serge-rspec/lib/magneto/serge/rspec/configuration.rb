# frozen_string_literal: true

module Magneto
  module Serge
    module RSpec
      # Configuration for Magneto::Serge::RSpec integration
      class Configuration
        # @return [String] directory where cassettes are stored
        attr_accessor :cassette_library_dir

        # @return [Hash] default cassette options applied to all cassettes
        attr_accessor :default_cassette_options

        # @return [Integer] default proxy port
        attr_accessor :proxy_port

        # @return [Boolean] whether to hook HTTP libraries automatically
        attr_accessor :hook_into

        # @return [Boolean] whether to allow real HTTP when cassette not found
        attr_accessor :allow_http_connections_when_no_cassette

        # @return [Array<String>] headers to filter from recordings
        attr_accessor :filter_sensitive_headers

        # @return [Boolean] whether to use strict matching
        attr_accessor :strict_matching

        # @return [Proc] custom cassette name generator
        attr_accessor :cassette_name_generator

        def initialize
          @cassette_library_dir = "spec/fixtures/cassettes"
          @default_cassette_options = {
            record: :new_episodes,  # :new_episodes, :once, :all, :none
            mode: :auto,            # :auto, :record, :replay, :passthrough
            match_requests_on: [:method, :uri, :body]
          }
          @proxy_port = 8888
          @hook_into = []  # [:webmock, :faraday, :httpclient, etc.]
          @allow_http_connections_when_no_cassette = false
          @filter_sensitive_headers = %w[Authorization Cookie Set-Cookie X-API-Key]
          @strict_matching = false
          @cassette_name_generator = ->(metadata) {
            # Generate cassette name from example metadata
            # Default: spec/fixtures/cassettes/MyClass/my_example.json
            parts = []

            if metadata[:example_group]
              # Walk up the example group hierarchy
              group = metadata[:example_group]
              while group
                parts.unshift(group[:description]) if group[:description]
                group = group[:parent_example_group]
              end
            end

            # Add example description
            parts << metadata[:description] if metadata[:description]

            # Sanitize for filename
            parts.map { |p| p.to_s.gsub(/[^\w\-]/, "_").squeeze("_") }.join("/")
          }
        end

        # Validate configuration
        #
        # @raise [Error] if configuration is invalid
        def validate!
          raise Error, "cassette_library_dir must be set" if cassette_library_dir.nil? || cassette_library_dir.empty?
          raise Error, "proxy_port must be between 1024 and 65535" unless proxy_port.between?(1024, 65535)
          raise Error, "cassette_name_generator must respond to :call" unless cassette_name_generator.respond_to?(:call)
        end

        # Convert record mode symbol to Magneto mode
        #
        # @param record_mode [Symbol] :new_episodes, :once, :all, :none
        # @return [Symbol] :auto, :record, :replay, :passthrough
        def translate_record_mode(record_mode)
          case record_mode
          when :new_episodes then :auto
          when :once then :replay
          when :all then :record
          when :none then :replay
          else :auto
          end
        end
      end
    end
  end
end
