#!/bin/bash
# Comprehensive test script

set -e

echo "Running RustForge OS test suite..."

# Test each component
echo "Testing allocator..."
cargo test -p allocator

echo "Testing common..."
cargo test -p common

echo "Testing lang..."
cargo test -p lang

echo "Testing net..."
cargo test -p net

echo "Testing db..."
cargo test -p db

echo "Testing runtime..."
cargo test -p runtime

echo "Testing sandbox..."
cargo test -p sandbox

echo "Testing distributed..."
cargo test -p distributed

echo "Testing wasm..."
cargo test -p wasm

echo "Testing monitor..."
cargo test -p monitor

echo ""
echo "All tests passed!"
