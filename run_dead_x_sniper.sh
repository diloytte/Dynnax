#!/bin/bash

# Exit on error
set -e

MODE="release"
BUILD_FLAG="--release"
BINARY_PATH="target/release/dead_x_sniper"

# Check for --dev flag
if [[ "$1" == "--dev" ]]; then
  MODE="debug"
  BUILD_FLAG=""
  BINARY_PATH="target/debug/dead_x_sniper"
fi

# Build the dead x sniper project
echo "🔧 Building dead x sniper in $MODE mode..."
cargo build -p dead_x_sniper $BUILD_FLAG

# Check if the binary exists and run it
if [ -f "$BINARY_PATH" ]; then
  echo "🚀 Running dead_x_sniper..."
  "$BINARY_PATH"
else
  echo "❌ Build failed or binary not found!"
  exit 1
fi