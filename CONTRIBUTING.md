# Contributing to liter-llm

Thank you for your interest in contributing to liter-llm! This guide will help you get started with development.

## Table of Contents

- [Development Setup](#development-setup)
    - [Task Installation](#task-installation)
    - [Quick Start](#quick-start)
- [Development Workflow](#development-workflow)
    - [Common Commands](#common-commands)
    - [Language-Specific Tasks](#language-specific-tasks)
- [Adding Providers](#adding-providers)
- [E2E Tests](#e2e-tests)
- [Exploring Tasks](#exploring-tasks)
- [Code Quality](#code-quality)
- [Submitting Changes](#submitting-changes)

## Development Setup

### Task Installation

This project uses [Task](https://taskfile.dev/) for task automation and orchestration. Task is a task runner that simplifies development workflows across multiple languages and platforms.

#### Install Task

Choose the installation method for your platform:

**macOS (Homebrew):**

```bash
brew install go-task
```

**Linux:**

```bash
# Using the installer script
sh -c "$(curl --location https://taskfile.dev/install.sh)" -- -d -b ~/.local/bin
# Or via package managers:
apt install go-task  # Debian/Ubuntu
pacman -S go-task    # Arch
```

**Windows:**

```powershell
# Using Scoop
scoop install task

# Or using Chocolatey
choco install go-task
```

For complete installation instructions, visit the [official Task documentation](https://taskfile.dev/installation/).

### Quick Start

After installing Task, set up your development environment:

```bash
# One-time setup - installs all dependencies
task setup

# Build in dev mode (fast iteration)
task build:dev
```

The setup command will install Rust, Python, Node.js, Go, Java, and Elixir tooling as needed.

## Development Workflow

### Common Commands

```bash
# Build all crates
task build

# Build in dev mode (fast iteration)
task build:dev

# Build in release mode (optimized)
task build:release
```

```bash
# Run all tests
task test

# Run all checks (lint + test)
task check
```

```bash
# Format all code
task format

# Run all linters via prek
task lint

# Generate READMEs from templates
task generate-readme
```

```bash
# Update all dependencies
task update

# Clean all build artifacts
task clean
```

### Language-Specific Tasks

Each language binding has its own namespace:

**Rust:**

```bash
task rust:build
task rust:test
task rust:format
task rust:lint
```

**Python:**

```bash
task python:install
task python:test
task python:format
task python:lint
```

**Node.js:**

```bash
task node:build        # Build NAPI-RS native module (release)
task node:build:dev    # Build in debug mode
task node:test
```

**Go:**

```bash
task go:build          # Build Go bindings (requires FFI)
task go:build:ffi      # Build FFI static library for Go
task go:test
task go:format
task go:lint
```

**Java:**

```bash
task java:build:ffi    # Build FFI shared library for Java
task java:test
```

**Elixir:**

```bash
task elixir:build      # Compile (includes Rustler NIF)
task elixir:test
task elixir:deps
```

**Ruby:**

```bash
task ruby:build        # Build Ruby native extension
task ruby:test         # Run Ruby tests
task ruby:format       # Format Ruby code
task ruby:lint         # Lint Ruby code
```

**WebAssembly:**

```bash
task wasm:build         # Build WASM package (web target)
task wasm:build:bundler # Build WASM package (bundler target)
task wasm:build:node    # Build WASM package (Node.js target)
task wasm:test          # Run WASM tests
```

**C:**

```bash
task c:build:ffi       # Build FFI library for C tests
task c:e2e:build       # Build C E2E tests
task c:e2e:test        # Run C E2E tests
```

## Adding Providers

### Steps

1. **Add a provider entry** to `schemas/providers.json`:

    ```json
    {
      "my-provider": {
        "base_url": "https://api.myprovider.com/v1",
        "auth_header": "Authorization",
        "auth_prefix": "Bearer",
        "model_prefixes": ["my-provider/"],
        "parameter_mappings": {}
      }
    }
    ```

    Fields:

    - `base_url` (required): Provider API base URL
    - `auth_header` (required): Header name for authentication
    - `auth_prefix` (optional): Prefix for the auth value (e.g. "Bearer")
    - `model_prefixes` (required): Model name prefixes that route to this provider
    - `parameter_mappings` (optional): Map OpenAI parameter names to provider-specific names

1. **Regenerate types**

    ```bash
    task generate:types
    ```

1. **Build and test**

    ```bash
    task build:dev
    task test
    ```

1. **Regenerate E2E tests**

    ```bash
    task e2e:generate:all
    task test
    ```

## E2E Tests

E2E tests are generated from JSON fixtures in `tools/e2e-generator/fixtures/` and produce runnable test suites for each language binding.

```bash
# Generate E2E tests for all languages
task e2e:generate:all

# Generate for a specific language
task e2e:generate:rust
task e2e:generate:python
task e2e:generate:go
task e2e:generate:java
task e2e:generate:elixir
task e2e:generate:ruby
task e2e:generate:c

# Run Rust E2E tests
task e2e:test:rust
```

Generated test files in `e2e/` should not be edited directly — modify fixtures or the generator source instead.

## Exploring Tasks

```bash
# Show all available tasks
task --list

# Show all tasks including internal ones
task --list-all
```

## Code Quality

### Pre-commit Hooks

The project uses [prek](https://github.com/Goldziher/gitfluff) for pre-commit hooks:

```bash
# Install hooks
prek install
prek install --hook-type commit-msg

# Run all hooks manually
prek run --all-files
```

### Commit Messages

We use conventional commits:

- `feat: add support for new-provider`
- `fix: correct auth header injection`
- `docs: update installation instructions`
- `chore: update dependencies`
- `test: add tests for streaming`

## Submitting Changes

1. **Create a feature branch**

    ```bash
    git checkout -b feat/add-provider-support
    ```

1. **Make your changes** and run checks locally:

    ```bash
    task check
    ```

1. **Commit and push**

    ```bash
    git commit -m "feat: add support for new provider"
    git push origin feat/add-provider-support
    ```

1. **Create a Pull Request** — link any related issues and ensure CI passes.

## Maintenance Tasks

### Version Synchronization

Version is managed in `Cargo.toml` workspace and synced across all manifests:

```bash
task version:sync
```

## Questions?

- Check existing [issues](https://github.com/kreuzberg-dev/liter-llm/issues)
- Join our [Discord community](https://discord.gg/xt9WY3GnKR)

Thank you for contributing to liter-llm!
