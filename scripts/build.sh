#!/usr/bin/env bash
# version check sh
set -euo pipefail

echo "Building monogirl..."

echo "Building Rust workspace..."
cargo build --release

echo "Building TypeScript SDK..."
cd sdk
npm install
npm run build
cd ..

echo "Build complete."
