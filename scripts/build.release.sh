#!/bin/bash

# 确保脚本从项目根目录运行
if ! [[ "$0" =~ scripts/build.release.sh ]]; then
    echo "错误: 必须从项目根目录运行此脚本"
    exit 1
fi

# 获取项目根目录的绝对路径
PROJECT_ROOT=$(pwd)

set -e

echo "开始构建发布版本..."

# 构建项目
cargo build --release

# 输出构建的二进制文件路径
echo "构建完成。二进制文件位置："
echo "${PROJECT_ROOT}/target/release/fair-vm-cli"

# 显示帮助信息
echo "帮助信息："
"${PROJECT_ROOT}/target/release/fair-vm-cli" --help

# 显示版本信息
echo "版本信息："
"${PROJECT_ROOT}/target/release/fair-vm-cli" --version
