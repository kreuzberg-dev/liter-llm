#!/usr/bin/env bash
# scripts/lib/common.sh — shared helper functions for liter-lm shell scripts.
#
# Source this file at the top of any script:
#   source "$(dirname "${BASH_SOURCE[0]}")/lib/common.sh"

set -euo pipefail

# ---------------------------------------------------------------------------
# Repository root detection
# ---------------------------------------------------------------------------

get_repo_root() {
  local start_dir current_dir
  start_dir="$(pwd)"
  current_dir="$start_dir"

  while [ "$current_dir" != "/" ]; do
    if [ -f "$current_dir/Cargo.toml" ]; then
      echo "$current_dir"
      return 0
    fi
    current_dir="$(dirname "$current_dir")"
  done

  echo "Error: Could not find repository root (Cargo.toml) from: $start_dir" >&2
  return 1
}

validate_repo_root() {
  local repo_root="${1:-${REPO_ROOT:-}}"

  if [ -z "$repo_root" ]; then
    echo "Error: REPO_ROOT not provided and env var not set" >&2
    return 1
  fi

  if [ ! -f "$repo_root/Cargo.toml" ]; then
    echo "Error: REPO_ROOT validation failed. Expected Cargo.toml at: $repo_root/Cargo.toml" >&2
    echo "REPO_ROOT resolved to: $repo_root" >&2
    return 1
  fi

  return 0
}

# ---------------------------------------------------------------------------
# Platform detection
# ---------------------------------------------------------------------------

get_platform() {
  # Respect GitHub Actions RUNNER_OS when set.
  if [ -n "${RUNNER_OS:-}" ]; then
    echo "$RUNNER_OS"
    return
  fi

  case "$(uname -s)" in
  Linux*) echo "Linux" ;;
  Darwin*) echo "macOS" ;;
  MINGW* | MSYS* | CYGWIN*) echo "Windows" ;;
  *) echo "unknown" ;;
  esac
}

is_linux() { [ "$(get_platform)" = "Linux" ]; }
is_macos() { [ "$(get_platform)" = "macOS" ]; }
is_windows() { [ "$(get_platform)" = "Windows" ]; }

get_arch() {
  case "$(uname -m)" in
  x86_64 | amd64) echo "x86_64" ;;
  aarch64 | arm64) echo "aarch64" ;;
  *) uname -m ;;
  esac
}

# ---------------------------------------------------------------------------
# Colour output helpers
# ---------------------------------------------------------------------------

# Check whether we should use colour (TTY + not explicitly disabled).
_use_color() {
  [ -t 1 ] && [ "${NO_COLOR:-}" = "" ] && [ "${TERM:-dumb}" != "dumb" ]
}

_RED='\033[31m'
_GREEN='\033[32m'
_YELLOW='\033[33m'
_CYAN='\033[36m'
_BOLD='\033[1m'
_RESET='\033[0m'

_color() {
  local code="$1"
  shift
  if _use_color; then
    printf "${code}%s${_RESET}" "$*"
  else
    printf "%s" "$*"
  fi
}

# ---------------------------------------------------------------------------
# Logging functions
# ---------------------------------------------------------------------------

log_info() {
  printf "  %s %s\n" "$(_color "$_CYAN" "•")" "$*"
}

log_ok() {
  printf "  %s %s\n" "$(_color "$_GREEN" "✓")" "$*"
}

log_warn() {
  printf "  %s %s\n" "$(_color "$_YELLOW" "!")" "$*" >&2
}

log_error() {
  printf "  %s %s\n" "$(_color "$_RED" "✗")" "$*" >&2
}

log_section() {
  printf "\n%s\n" "$(_color "$_BOLD" "$*")"
}

# ---------------------------------------------------------------------------
# Error helpers
# ---------------------------------------------------------------------------

error_exit() {
  local message="${1:-Unknown error}"
  local exit_code="${2:-1}"
  log_error "$message"
  exit "$exit_code"
}

require_command() {
  local cmd="$1"
  if ! command -v "$cmd" &>/dev/null; then
    error_exit "Required command not found: $cmd"
  fi
}

# ---------------------------------------------------------------------------
# Exports
# ---------------------------------------------------------------------------

export -f get_repo_root
export -f validate_repo_root
export -f get_platform
export -f is_linux
export -f is_macos
export -f is_windows
export -f get_arch
export -f log_info
export -f log_ok
export -f log_warn
export -f log_error
export -f log_section
export -f error_exit
export -f require_command
