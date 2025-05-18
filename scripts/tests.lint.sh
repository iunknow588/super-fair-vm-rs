#!/bin/bash

# 确保脚本从项目根目录运行
if ! [[ "$0" =~ scripts/tests.lint.sh ]]; then
    echo "错误: 必须从项目根目录运行此脚本"
    exit 1
fi

# 获取项目根目录的绝对路径
PROJECT_ROOT=$(pwd)

set -e

echo "运行代码格式检查..."

# 自动格式化所有 Rust 代码
cargo fmt

# 检查格式是否合规（可选）
cargo fmt -- --check

echo "运行 clippy 检查..."

# 运行 clippy
cargo clippy --all-targets --all-features -- -D warnings

echo "检查完成"
