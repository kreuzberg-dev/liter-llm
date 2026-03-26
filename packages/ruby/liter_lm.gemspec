# frozen_string_literal: true

Gem::Specification.new do |spec|
  spec.name = 'liter_lm'
  spec.version = '1.0.0.pre.rc.1'
  spec.authors = ["Na'aman Hirschfeld"]
  spec.email = ['naaman@kreuzberg.dev']

  spec.summary = 'High-performance LLM client — unified streaming, tool calling, and provider routing'
  spec.description = <<~DESC
    liter-lm is a high-performance LLM client library with a Rust core and native
    Ruby bindings via Magnus. Provides a unified interface for streaming completions,
    tool calling, and provider routing across OpenAI, Anthropic, and 50+ LLM providers.
  DESC
  spec.homepage = 'https://github.com/kreuzberg-dev/liter-lm'
  spec.license = 'MIT'
  spec.required_ruby_version = ['>= 3.2.0', '< 5.0']

  spec.metadata = {
    'homepage_uri' => spec.homepage,
    'source_code_uri' => 'https://github.com/kreuzberg-dev/liter-lm',
    'changelog_uri' => 'https://github.com/kreuzberg-dev/liter-lm/blob/main/CHANGELOG.md',
    'bug_tracker_uri' => 'https://github.com/kreuzberg-dev/liter-lm/issues',
    'rubygems_mfa_required' => 'true',
    'keywords' => 'llm,llm-client,openai,anthropic,streaming,tool-calling,provider-routing,rust,native-extension'
  }

  spec.files = Dir.glob(%w[
                          README.md
                          LICENSE
                          lib/**/*.rb
                          sig/**/*.rbs
                        ], File::FNM_DOTMATCH).select { |f| File.exist?(f) }

  spec.require_paths = ['lib']
  spec.extensions = ['ext/liter_lm_rb/extconf.rb']

  spec.add_dependency 'rb_sys', '~> 0.9.119'

  spec.add_development_dependency 'bundler', '~> 4.0'
  spec.add_development_dependency 'rake', '~> 13.0'
  spec.add_development_dependency 'rake-compiler', '~> 1.2'
  spec.add_development_dependency 'rspec', '~> 3.12'
  unless Gem.win_platform?
    spec.add_development_dependency 'rbs', '~> 3.0'
    spec.add_development_dependency 'rubocop', '~> 1.66'
    spec.add_development_dependency 'rubocop-performance', '~> 1.21'
    spec.add_development_dependency 'rubocop-rspec', '~> 3.0'
    spec.add_development_dependency 'steep', '~> 1.8'
  end
  spec.add_development_dependency 'yard', '~> 0.9'
end
