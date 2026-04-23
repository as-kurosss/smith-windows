#!/bin/bash
# Build and test script for smith-windows
# This script is called via bash/sh

cd /d D:/Alexey/rust/smith-windows || exit 1
echo "=== Running cargo check ==="
C:/Users/Kurosss/.cargo/bin/cargo.exe check 2>&1
if [ $? -ne 0 ]; then
    echo "Cargo check failed!"
    exit 1
fi

echo "=== Running cargo test ==="
C:/Users/Kurosss/.cargo/bin/cargo.exe test 2>&1
if [ $? -ne 0 ]; then
    echo "Cargo test failed!"
    exit 1
fi

echo "=== Running cargo clippy ==="
C:/Users/Kurosss/.cargo/bin/cargo.exe clippy -- -D warnings 2>&1
if [ $? -ne 0 ]; then
    echo "Cargo clippy failed!"
    exit 1
fi

echo "=== Running cargo fmt check ==="
C:/Users/Kurosss/.cargo/bin/cargo.exe fmt --check 2>&1
if [ $? -ne 0 ]; then
    echo "Cargo fmt check failed!"
    exit 1
fi

echo "=== All checks passed! ==="
