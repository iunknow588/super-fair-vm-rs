#!/usr/bin/env bash
set -xue

if ! [[ "$0" =~ scripts/tests.lint.sh ]]; then
  echo "must be run from repository root"
  exit 255
fi

# 运行格式化检查脚本
echo "Running format check..."
bash ./scripts/format.sh

# 运行质量检查脚本
echo "Running clippy check..."
bash ./scripts/clippy.sh

# 清理构建产物
cargo clean

echo "ALL SUCCESS!"
