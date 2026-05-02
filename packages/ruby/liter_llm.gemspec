# frozen_string_literal: true

Gem::Specification.new do |spec|
  spec.name = 'liter_llm'
  spec.version = '1.4.0.pre.rc.27'
  spec.authors       = ["Na'aman Hirschfeld"]
  spec.summary       = 'Universal LLM API client with Rust-powered polyglot bindings.'
  spec.description   = 'Universal LLM API client with Rust-powered polyglot bindings.'
  spec.homepage      = 'https://github.com/kreuzberg-dev/liter-llm'
  spec.license       = 'MIT'
  spec.required_ruby_version = '>= 3.2.0'
  spec.metadata['keywords'] = %w[llm api-client openai anthropic].join(',')
  spec.metadata['rubygems_mfa_required'] = 'true'

  spec.files         = Dir.glob(%w[lib/**/* ext/**/* sig/**/* Steepfile])
  spec.require_paths = ['lib']
  spec.extensions    = ['ext/liter_llm_rb/extconf.rb']

  spec.add_dependency 'rb_sys', '~> 0.9'
end
