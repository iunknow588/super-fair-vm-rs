# bin 目录

本目录包含 timestampvm 的可执行文件实现。

## 目录结构

- `timestampvm/` - timestampvm 可执行文件的实现
  - `main.rs` - 主程序入口点
  - `genesis.rs` - 创世区块命令实现
  - `vm_id.rs` - VM ID 转换命令实现

## 功能概述

这个目录包含将 timestampvm 库编译为可执行文件所需的代码。可执行文件可以作为 AvalancheGo 的插件运行，也可以用于生成创世区块和 VM ID。
