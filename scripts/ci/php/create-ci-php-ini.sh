#!/bin/bash

set -e

# This script creates a php.ini file for CI testing that loads the built PHP extension.
# This allows PHPUnit to find and load the locally-built extension without requiring
# system-wide installation or sudo access.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../../" && pwd)"
TARGET_DIR="$REPO_ROOT/target/release/deps"
OUTPUT_DIR="${OUTPUT_DIR:-.}"
INI_FILE="$OUTPUT_DIR/php-liter-llm.ini"

echo "=== Creating CI PHP ini file ==="
echo "Repo root: $REPO_ROOT"
echo "Target dir: $TARGET_DIR"
echo "Output file: $INI_FILE"
echo ""

# Determine the extension file based on OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
  EXT_FILE="libliter_llm_php.so"
elif [[ "$OSTYPE" == "darwin"* ]]; then
  EXT_FILE="libliter_llm_php.dylib"
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
  EXT_FILE="liter_llm_php.dll"
else
  echo "Warning: Unknown OS type: $OSTYPE - assuming Linux"
  EXT_FILE="libliter_llm_php.so"
fi

BUILT_EXT="$TARGET_DIR/$EXT_FILE"

if [ ! -f "$BUILT_EXT" ]; then
  echo "ERROR: Built extension not found at $BUILT_EXT"
  echo ""
  echo "Available files in $TARGET_DIR:"
  find "$TARGET_DIR" -maxdepth 1 -iname "*liter_llm*" -type f 2>/dev/null || echo "No liter_llm files found"
  exit 1
fi

echo "Found built extension: $BUILT_EXT"
echo "Extension file size: $(du -h "$BUILT_EXT" | cut -f1)"
echo ""

# Convert paths to format acceptable by PHP on Windows (forward slashes)
if [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
  DISPLAY_DIR="${TARGET_DIR//\\/\/}"
else
  DISPLAY_DIR="$TARGET_DIR"
fi

# Create the ini file with absolute path
if cat >"$INI_FILE" <<EOF; then
; Liter-LLM PHP Extension Configuration for CI Testing
; This file is generated automatically by create-ci-php-ini.sh
; It allows loading the locally-built extension without system-wide installation

; Load the Liter-LLM PHP extension using full path
; This avoids overriding extension_dir which would prevent core extensions from loading
extension="$DISPLAY_DIR/$EXT_FILE"
EOF
  echo "INI file created: $INI_FILE"
  echo ""
  echo "INI file contents:"
  cat "$INI_FILE"
else
  echo "Failed to create INI file"
  exit 1
fi
