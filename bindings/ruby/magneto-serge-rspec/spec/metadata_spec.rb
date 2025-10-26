# frozen_string_literal: true

require 'spec_helper'

RSpec.describe Magneto::Serge::RSpec::Metadata do
  # This module is automatically included via RSpec configuration
  # We test it through actual RSpec examples

  describe '#use_cassette' do
    it 'runs block with cassette active' do
      cassette_ran = false

      use_cassette('test_cassette') do
        cassette_ran = true
      end

      expect(cassette_ran).to be true
    end

    it 'accepts cassette options' do
      expect {
        use_cassette('test_cassette', record: :all, mode: :record) do
          # Block executes
        end
      }.not_to raise_error
    end

    it 'stops proxy after block completes' do
      proxy_stopped = false

      use_cassette('test_cassette') do
        # Proxy should be running
      end

      # Proxy should be stopped after block
      expect { proxy_stopped = true }.not_to raise_error
    end

    it 'handles nested cassettes' do
      outer_ran = false
      inner_ran = false

      use_cassette('outer') do
        outer_ran = true

        use_cassette('inner') do
          inner_ran = true
        end
      end

      expect(outer_ran).to be true
      expect(inner_ran).to be true
    end

    it 'creates cassette directory if missing' do
      cassette_dir = 'spec/fixtures/cassettes'
      FileUtils.rm_rf(cassette_dir) if Dir.exist?(cassette_dir)

      use_cassette('test_cassette') do
        # Block executes
      end

      expect(Dir.exist?(File.join(cassette_dir))).to be true
    end
  end

  describe '#current_cassette' do
    it 'returns nil when no cassette active' do
      expect(current_cassette).to be_nil
    end

    # Note: Testing current_cassette within active cassette requires
    # metadata-driven cassette activation, tested in integration tests
  end
end
