#!/usr/bin/env bash
# coverage flag
# build err handling
set -euo pipefail

echo "Running monogirl tests..."

echo "Running Rust tests..."
cargo test --all

echo "Running SDK tests..."
cd sdk
npm test
cd ..

echo "All tests passed."
