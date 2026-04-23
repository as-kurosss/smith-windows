#!/bin/bash
cd /d D:\Alexey\rust\smith-windows
cargo check || exit 1
cargo test 2>&1 || exit 1
cargo clippy -- -D warnings 2>&1 || exit 1
cargo fmt --check 2>&1 || exit 1
