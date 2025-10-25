# frozen_string_literal: true

require_relative "lib/magneto/serge/rspec/version"

Gem::Specification.new do |spec|
  spec.name = "magneto-serge-rspec"
  spec.version = Magneto::Serge::RSpec::VERSION
  spec.authors = ["Magnéto-Serge contributors"]
  spec.email = ["contact@taciclei.com"]

  spec.summary = "RSpec integration for Magnéto-Serge HTTP/WebSocket recording library"
  spec.description = <<~DESC
    RSpec integration for Magnéto-Serge, providing automatic cassette management,
    metadata-driven configuration, and seamless test framework integration for
    recording and replaying HTTP/WebSocket interactions.
  DESC
  spec.homepage = "https://github.com/taciclei/magneto-serge"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 3.0.0"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/taciclei/magneto-serge"
  spec.metadata["changelog_uri"] = "https://github.com/taciclei/magneto-serge/blob/main/CHANGELOG.md"
  spec.metadata["documentation_uri"] = "https://docs.rs/magneto-serge"

  # Specify which files should be added to the gem when it is released.
  spec.files = Dir.glob(%w[
    lib/**/*
    README.md
    LICENSE
  ])
  spec.require_paths = ["lib"]

  # Runtime dependencies
  spec.add_dependency "rspec", "~> 3.0"
  spec.add_dependency "magneto-serge", "~> 0.2"

  # Development dependencies
  spec.add_development_dependency "rake", "~> 13.0"
  spec.add_development_dependency "rubocop", "~> 1.0"
  spec.add_development_dependency "webmock", "~> 3.0"
end
