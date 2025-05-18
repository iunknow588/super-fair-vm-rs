#!/bin/bash

# 确保脚本从项目根目录运行
if ! [[ "$0" =~ scripts/format.sh ]]; then
    echo "错误: 必须从项目根目录运行此脚本"
    exit 1
fi

# 获取项目根目录的绝对路径
PROJECT_ROOT=$(pwd)

set -e

echo "格式化代码..."

# 运行 rustfmt
cargo fmt

echo "格式化完成"