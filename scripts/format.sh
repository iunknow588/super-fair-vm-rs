#!/usr/bin/env bash
set -xue

# 先自动格式化
cargo fmt --all

# 再检查格式
cargo fmt --all -- --check

echo "Format check passed!"