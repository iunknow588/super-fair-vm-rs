#!/bin/bash

# 确保脚本从项目根目录运行
if ! [[ "$0" =~ scripts/clippy.sh ]]; then
    echo "错误: 必须从项目根目录运行此脚本"
    exit 1
fi

# 获取项目根目录的绝对路径
PROJECT_ROOT=$(pwd)

set -e

echo "运行 clippy 检查..."

# 运行 clippy
cargo clippy -- -D warnings

echo "检查完成"
