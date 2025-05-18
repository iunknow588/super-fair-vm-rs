#!/bin/bash

# 构建 fairvm 二进制文件
cargo build --release

# 运行端到端测试
# 使用方法：
# VM_PLUGIN_PATH=$(pwd)/target/release/fairvm ./scripts/tests.e2e.sh

# 或者保持网络运行：
# NETWORK_RUNNER_ENABLE_SHUTDOWN=1 VM_PLUGIN_PATH=$(pwd)/target/release/fairvm ./scripts/tests.e2e.sh

# 或者使用特定版本的 AvalancheGo：
# VM_PLUGIN_PATH=$(pwd)/target/release/fairvm ./scripts/tests.e2e.sh 1.11.1

set -e

# 检查环境变量
if [ -z "$VM_PLUGIN_PATH" ]; then
    echo "错误: 未设置 VM_PLUGIN_PATH 环境变量"
    exit 1
fi

# 获取 AvalancheGo 版本
AVALANCHEGO_VERSION=${1:-"latest"}

# 设置网络运行器端点
NETWORK_RUNNER_GRPC_ENDPOINT=${NETWORK_RUNNER_GRPC_ENDPOINT:-"127.0.0.1:8080"}

# 设置是否在测试后关闭网络
NETWORK_RUNNER_ENABLE_SHUTDOWN=${NETWORK_RUNNER_ENABLE_SHUTDOWN:-"false"}

# 运行测试
cargo test --package e2e --lib -- tests::e2e --exact --nocapture

# 如果需要，关闭网络
if [ "$NETWORK_RUNNER_ENABLE_SHUTDOWN" = "true" ]; then
    echo "关闭网络..."
    # 这里添加关闭网络的命令
fi
