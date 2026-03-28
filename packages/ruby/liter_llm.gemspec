# frozen_string_literal: true

repo_root = File.expand_path('../..', __dir__)

# Collect ruby package files via Dir.glob (no git dependency)
ruby_files = Dir.chdir(__dir__) do
  Dir.glob(
    %w[
      README.md
      LICENSE
      ext/**/*.rs
      ext/**/*.rb
      ext/**/*.toml
      ext/**/*.lock
      ext/**/*.md
      ext/**/build.rs
      ext/**/Cargo.*
      lib/**/*.rb
      sig/**/*.rbs
    ],
    File::FNM_DOTMATCH
  )
end

# Collect crate source files for vendoring into the gem
crate_files = Dir.chdir(repo_root) do
  crate_dirs = %w[
    liter-llm
    liter-llm-ffi
  ]

  crate_dirs.flat_map do |crate|
    Dir.glob("crates/#{crate}/**/*", File::FNM_DOTMATCH)
       .reject { |f| File.directory?(f) }
       .reject { |f| f.include?('/target/') }
       .grep_v(/\.(swp|bak|tmp)$/)
       .grep_v(/~$/)
       .map { |path| "vendor/#{path.delete_prefix('crates/')}" }
  end
end

vendor_files = Dir.chdir(__dir__) do
  liter_llm_files = if Dir.exist?('vendor/liter-llm')
                      Dir.glob('vendor/liter-llm/**/*', File::FNM_DOTMATCH)
                         .reject { |f| File.directory?(f) }
                         .reject { |f| f.include?('/target/') }
                         .grep_v(/\.(swp|bak|tmp)$/)
                         .grep_v(/~$/)
                    else
                      []
                    end

  liter_llm_ffi_files = if Dir.exist?('vendor/liter-llm-ffi')
                          Dir.glob('vendor/liter-llm-ffi/**/*', File::FNM_DOTMATCH)
                             .reject { |f| File.directory?(f) }
                             .reject { |f| f.include?('/target/') }
                             .grep_v(/\.(swp|bak|tmp)$/)
                             .grep_v(/~$/)
                        else
                          []
                        end

  workspace_toml = if File.exist?('vendor/Cargo.toml')
                     ['vendor/Cargo.toml']
                   else
                     []
                   end

  liter_llm_files + liter_llm_ffi_files + workspace_toml
end

# When vendor files exist, get ext/ files from filesystem (to include modified Cargo.toml
# with vendor paths) instead of from git (which has original 5-level crate paths)
ext_files_from_fs = Dir.chdir(__dir__) do
  Dir.glob('ext/**/*', File::FNM_DOTMATCH)
     .reject { |f| File.directory?(f) }
     .reject { |f| f.include?('/target/') }
     .grep_v(/\.(swp|bak|tmp)$/)
     .grep_v(/~$/)
end

files = if vendor_files.any?
          # Use ext/ files from filesystem (modified by vendor script) + non-ext ruby files
          non_ext_ruby_files = ruby_files.reject { |f| f.start_with?('ext/') }
          non_ext_ruby_files + ext_files_from_fs + vendor_files
        else
          ruby_files + crate_files
        end

native_artifacts = Dir.chdir(__dir__) do
  Dir.glob(%w[
             lib/**/*.bundle
             lib/**/*.so
             lib/**/*.dll
             lib/**/*.dylib
           ])
end
files.concat(native_artifacts)

files = files.select { |f| File.exist?(f) }
files = files.uniq

Gem::Specification.new do |spec|
  spec.name = 'liter_llm'
  spec.version = '1.0.0.pre.rc.6'
  spec.authors = ["Na'aman Hirschfeld"]
  spec.email = ['naaman@kreuzberg.dev']

  spec.summary = 'Universal LLM API client — 142+ providers, streaming, tool calling. Rust-powered.'
  spec.description = <<~DESC
    liter-llm is a universal LLM API client with a Rust core and native Ruby bindings
    via Magnus. Provides a unified interface for streaming completions, tool calling,
    and provider routing across 142+ LLM providers. Rust-powered.
  DESC
  spec.homepage = 'https://kreuzberg.dev'
  spec.license = 'MIT'
  spec.required_ruby_version = ['>= 3.2.0', '< 5.0']

  spec.metadata = {
    'homepage_uri' => spec.homepage,
    'source_code_uri' => 'https://github.com/kreuzberg-dev/liter-llm',
    'changelog_uri' => 'https://github.com/kreuzberg-dev/liter-llm/blob/main/CHANGELOG.md',
    'bug_tracker_uri' => 'https://github.com/kreuzberg-dev/liter-llm/issues',
    'rubygems_mfa_required' => 'true',
    'keywords' => 'llm,llm-client,openai,anthropic,streaming,tool-calling,provider-routing,rust,native-extension'
  }

  spec.files = files
  spec.require_paths = ['lib']
  spec.extensions = ['ext/liter_llm_rb/extconf.rb']

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
