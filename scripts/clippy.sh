#!/usr/bin/env bash
set -xue

# 运行clippy进行代码质量检查
cargo clippy --all-targets --all-features -- -D warnings

echo "Clippy check passed!"
