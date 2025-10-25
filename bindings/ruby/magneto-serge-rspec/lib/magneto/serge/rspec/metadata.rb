# frozen_string_literal: true

module Magneto
  module Serge
    module RSpec
      # Methods for interacting with cassettes from within examples
      module Metadata
        # Insert a cassette for the duration of a block
        #
        # @param name [String] cassette name
        # @param options [Hash] cassette options
        # @option options [Symbol] :record record mode (:new_episodes, :once, :all, :none)
        # @option options [Symbol] :mode proxy mode (:auto, :record, :replay, :passthrough)
        # @option options [Integer] :port proxy port
        # @option options [Array<Symbol>] :match_requests_on matching criteria
        # @yield block to execute with cassette active
        # @example
        #   use_cassette("api_call") do
        #     response = HTTP.get("https://api.example.com/users")
        #     expect(response.status).to eq(200)
        #   end
        def use_cassette(name, options = {}, &block)
          config = Magneto::Serge::RSpec.configuration
          cassette_options = config.default_cassette_options.merge(options)

          cassette_path = File.join(config.cassette_library_dir, "#{name}.json")
          cassette_dir = File.dirname(cassette_path)
          FileUtils.mkdir_p(cassette_dir) unless Dir.exist?(cassette_dir)

          proxy = MagnetoSerge::MagnetoProxy.new(cassette_dir)

          # Determine mode
          mode = cassette_options[:mode] ||
                 config.translate_record_mode(cassette_options[:record])

          # Start proxy in appropriate mode
          case mode
          when :auto
            proxy.auto(name)
          when :record
            proxy.record(name)
          when :replay
            proxy.replay(name)
          when :passthrough
            proxy.passthrough
          else
            raise Error, "Unknown mode: #{mode}"
          end

          begin
            # Execute test block with cassette active
            yield
          ensure
            # Stop proxy and save cassette
            proxy.stop
          end
        end

        # Access current cassette (if any)
        #
        # @return [String, nil] current cassette name
        def current_cassette
          @current_cassette_name
        end

        private

        # Set current cassette name (used by hooks)
        def current_cassette=(name)
          @current_cassette_name = name
        end
      end
    end
  end
end
