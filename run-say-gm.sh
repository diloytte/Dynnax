#!/bin/bash

# Exit on error
set -e

MODE="release"
BUILD_FLAG="--release"
BINARY_PATH="target/release/say_gm"

# Check for --dev flag
if [[ "$1" == "--dev" ]]; then
  MODE="debug"
  BUILD_FLAG=""
  BINARY_PATH="target/debug/say_gm"
fi

# Build the dead x sniper project
echo "🔧 Building say gm in $MODE mode..."
cargo build -p say_gm $BUILD_FLAG

# Check if the binary exists and run it
if [ -f "$BINARY_PATH" ]; then
  echo "🚀 Running say_gm..."
  "$BINARY_PATH"
else
  echo "❌ Build failed or binary not found!"
  exit 1
fi