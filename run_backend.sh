#!/bin/bash

# Exit on error
set -e

MODE="release"
BUILD_FLAG="--release --features production"
BINARY_PATH="target/release/backend"

# Check for --dev flag
if [[ "$1" == "--dev" ]]; then
  MODE="debug"
  BUILD_FLAG=""
  BINARY_PATH="target/debug/backend"
fi

# Build the backend project
echo "🔧 Building backend in $MODE mode..."
cargo build -p backend $BUILD_FLAG

# Check if the binary exists and run it
if [ -f "$BINARY_PATH" ]; then
  echo "🚀 Running backend..."
  "$BINARY_PATH"
else
  echo "❌ Build failed or binary not found!"
  exit 1
fi
