# frozen_string_literal: true

require 'spec_helper'

RSpec.describe Magneto::Serge::RSpec::Configuration do
  let(:config) { described_class.new }

  describe '#initialize' do
    it 'sets default cassette_library_dir' do
      expect(config.cassette_library_dir).to eq('spec/fixtures/cassettes')
    end

    it 'sets default cassette options' do
      expect(config.default_cassette_options).to include(
        record: :new_episodes,
        mode: :auto,
        match_requests_on: [:method, :uri, :body]
      )
    end

    it 'sets default proxy_port' do
      expect(config.proxy_port).to eq(8888)
    end

    it 'sets default filter_sensitive_headers' do
      expect(config.filter_sensitive_headers).to include('Authorization', 'Cookie')
    end

    it 'sets default strict_matching' do
      expect(config.strict_matching).to be false
    end

    it 'sets default cassette_name_generator' do
      expect(config.cassette_name_generator).to respond_to(:call)
    end
  end

  describe '#validate!' do
    it 'does not raise with valid configuration' do
      expect { config.validate! }.not_to raise_error
    end

    it 'raises if cassette_library_dir is nil' do
      config.cassette_library_dir = nil
      expect { config.validate! }.to raise_error(Magneto::Serge::RSpec::Error, /cassette_library_dir must be set/)
    end

    it 'raises if cassette_library_dir is empty' do
      config.cassette_library_dir = ''
      expect { config.validate! }.to raise_error(Magneto::Serge::RSpec::Error, /cassette_library_dir must be set/)
    end

    it 'raises if proxy_port is too low' do
      config.proxy_port = 80
      expect { config.validate! }.to raise_error(Magneto::Serge::RSpec::Error, /proxy_port must be between/)
    end

    it 'raises if proxy_port is too high' do
      config.proxy_port = 70000
      expect { config.validate! }.to raise_error(Magneto::Serge::RSpec::Error, /proxy_port must be between/)
    end

    it 'raises if cassette_name_generator does not respond to call' do
      config.cassette_name_generator = 'not a proc'
      expect { config.validate! }.to raise_error(Magneto::Serge::RSpec::Error, /cassette_name_generator must respond to/)
    end
  end

  describe '#translate_record_mode' do
    it 'translates :new_episodes to :auto' do
      expect(config.translate_record_mode(:new_episodes)).to eq(:auto)
    end

    it 'translates :once to :replay' do
      expect(config.translate_record_mode(:once)).to eq(:replay)
    end

    it 'translates :all to :record' do
      expect(config.translate_record_mode(:all)).to eq(:record)
    end

    it 'translates :none to :replay' do
      expect(config.translate_record_mode(:none)).to eq(:replay)
    end

    it 'defaults unknown modes to :auto' do
      expect(config.translate_record_mode(:unknown)).to eq(:auto)
    end
  end

  describe 'custom configuration' do
    it 'allows setting custom cassette_library_dir' do
      config.cassette_library_dir = 'custom/path'
      expect(config.cassette_library_dir).to eq('custom/path')
    end

    it 'allows setting custom proxy_port' do
      config.proxy_port = 9999
      expect(config.proxy_port).to eq(9999)
    end

    it 'allows setting custom filter_sensitive_headers' do
      config.filter_sensitive_headers = %w[X-Custom-Token]
      expect(config.filter_sensitive_headers).to eq(%w[X-Custom-Token])
    end

    it 'allows setting custom cassette_name_generator' do
      custom_generator = ->(_metadata) { 'custom_name' }
      config.cassette_name_generator = custom_generator
      expect(config.cassette_name_generator).to eq(custom_generator)
    end
  end
end
