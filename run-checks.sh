#!/bin/bash

set -e

#echo "🧽 Formatting code..."
#cargo fmt --all

echo "🕵️ Checking for unused dependencies..."
cargo +nightly udeps --workspace --all-targets

#echo "🔍 Running Clippy..."
#cargo clippy --workspace --all-targets --all-features -- -D warnings

#echo "🧪 Running tests..."
#cargo test --workspace

echo "✅ All checks passed!"
