#!/usr/bin/env bash
set -xue

# 运行rustfmt进行代码格式化检查
cargo fmt --all -- --check

echo "Format check passed!"
