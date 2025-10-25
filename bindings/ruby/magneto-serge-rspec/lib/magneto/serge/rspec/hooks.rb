# frozen_string_literal: true

module Magneto
  module Serge
    module RSpec
      # RSpec hooks for automatic cassette management
      module Hooks
        class << self
          # Around hook that manages cassette lifecycle based on metadata
          #
          # @param example [::RSpec::Core::Example] RSpec example
          def around_hook(example)
            metadata = example.metadata

            # Check if example has :magneto or :cassette metadata
            cassette_metadata = extract_cassette_metadata(metadata)

            if cassette_metadata
              run_with_cassette(example, cassette_metadata)
            else
              # No cassette metadata, run example normally
              example.run
            end
          end

          private

          # Extract cassette configuration from RSpec metadata
          #
          # @param metadata [Hash] RSpec example metadata
          # @return [Hash, nil] cassette configuration or nil if not using cassettes
          def extract_cassette_metadata(metadata)
            config = Magneto::Serge::RSpec.configuration

            # Check for :magneto or :cassette tag
            return nil unless metadata[:magneto] || metadata[:cassette]

            cassette_options = {}

            # If :magneto is a Hash, use it as options
            if metadata[:magneto].is_a?(Hash)
              cassette_options.merge!(metadata[:magneto])
            end

            # If :cassette is a String, use it as cassette name
            if metadata[:cassette].is_a?(String)
              cassette_options[:name] = metadata[:cassette]
            elsif metadata[:cassette].is_a?(Hash)
              cassette_options.merge!(metadata[:cassette])
            end

            # Generate cassette name if not provided
            unless cassette_options[:name]
              cassette_options[:name] = config.cassette_name_generator.call(metadata)
            end

            # Merge with default options
            config.default_cassette_options.merge(cassette_options)
          end

          # Run example with cassette active
          #
          # @param example [::RSpec::Core::Example] RSpec example
          # @param options [Hash] cassette options
          def run_with_cassette(example, options)
            config = Magneto::Serge::RSpec.configuration
            cassette_name = options[:name]

            # Ensure cassette directory exists
            cassette_dir = config.cassette_library_dir
            FileUtils.mkdir_p(cassette_dir) unless Dir.exist?(cassette_dir)

            # Create proxy instance
            proxy = MagnetoSerge::MagnetoProxy.new(cassette_dir)

            # Determine mode
            mode = options[:mode] || config.translate_record_mode(options[:record])

            # Start proxy in appropriate mode
            begin
              case mode
              when :auto
                proxy.auto(cassette_name)
              when :record
                proxy.record(cassette_name)
              when :replay
                proxy.replay(cassette_name)
              when :passthrough
                proxy.passthrough
              else
                raise Error, "Unknown mode: #{mode}"
              end

              # Set current cassette for access in examples
              example.example_group_instance.send(:current_cassette=, cassette_name)

              # Run the example
              example.run
            ensure
              # Always stop the proxy and save cassette
              begin
                proxy.stop
              rescue => e
                warn "Failed to stop Magneto proxy: #{e.message}"
              end

              # Clear current cassette
              example.example_group_instance.send(:current_cassette=, nil)
            end
          end
        end
      end
    end
  end
end
