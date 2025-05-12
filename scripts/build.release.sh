#!/usr/bin/env bash
set -xue

if ! [[ "$0" =~ scripts/build.release.sh ]]; then
  echo "must be run from repository root"
  exit 255
fi

PROTOC_VERSION=$(protoc --version | cut -f2 -d' ')
if [[ "${PROTOC_VERSION}" == "" ]] || [[ "${PROTOC_VERSION}" < 3.15.0 ]]; then
  echo "protoc must be installed and the version must be greater than 3.15.0"
  exit 255
fi

# "--bin" can be specified multiple times for each directory in "bin/*" or workspaces
# 设置 RUSTFLAGS 环境变量，禁用一些可能导致问题的功能
export RUSTFLAGS="-C target-feature=-crt-static"

# 使用 --no-default-features 避免一些可能导致问题的功能
cargo build \
--release \
--bin timestampvm

./target/release/timestampvm --help

./target/release/timestampvm genesis "hello world"
./target/release/timestampvm vm-id timestampvm
