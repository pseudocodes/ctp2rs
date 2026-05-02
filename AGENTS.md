# AGENTS.md — ctp2rs

## 项目概述

ctp2rs 是上海期货信息技术公司 CTP/CTP-Mini/CTP-Sopt 接口的 Rust 原生封装。采用 `libloading` 在运行时动态加载 CTP 动态库（.so/.dll/.framework），通过 feature flag 切换不同柜台版本。

## Agent 行为约束

### 禁止

- **不要修改** `ctp-dyn/src/v1alpha1/generated/` 下的任何文件（仅供展示参考，不参与编译）
- **不要修改** `ctp-dyn/api/` 下的 C++ 头文件或预编译动态库
- **不要编辑** `build.rs` 生成到 `OUT_DIR` 的产物
- **不要假设接口签名** — 不同 feature/平台生成的接口数量和参数各不相同
- **不要在手写代码中引入不必要的 unsafe** — unsafe 应集中在 `ffi.rs` 和 codegen 生成的代码中

### 必须

- 修改 codegen 逻辑时，改 `ctp-dyn/codegen/` 下的源码，而非生成产物
- 修改接口行为前，先判断是 codegen 问题还是手写代码（`builder.rs`/`ffi.rs`）问题
- 添加新功能时，参考 `examples/` 下的现有模式
- 变更后运行 `cargo build -p ctp2rs` 验证编译通过

## Workspace 结构

```
ctp2rs/
├── ctp-dyn/          # 核心 crate（发布名 ctp2rs）
│   ├── build.rs      # 编译期代码生成入口
│   ├── codegen/      # codegen 模块（parser、stream、errors、setting）
│   ├── api/          # 各版本 CTP/Mini/Sopt 的 C++ 头文件和预编译动态库
│   └── src/
│       ├── ffi.rs    # 公共 FFI 工具（GB18030 编解码、字符串转换等）
│       └── v1alpha1/ # 版本化接口模块
│           ├── mod.rs       # 通过 include!(concat!(env!("OUT_DIR"), ...)) 引入生成代码
│           ├── builder.rs   # 手写的 builder 辅助代码
│           └── generated/   # ⚠️ 仅供展示，不参与实际编译
├── ctp-vt/           # 空占位 crate，暂无实现，可忽略
├── examples/         # 使用示例（多个独立 crate）
│   ├── channel/      # 基于 channel 的行情/交易示例
│   ├── insecure/     # 非安全连接示例
│   ├── localctp/     # LocalCTP 本地模拟示例
│   ├── openctp/      # OpenCTP 开放平台示例
│   └── tts_sopt/     # TTS 股票期权示例
└── Cargo.toml        # workspace 根配置
```

## 代码生成机制（关键）

**所有 CTP 接口的 Rust 绑定代码都由 `build.rs` 在编译期生成，不在源码树中。**

- 生成产物路径：`target/<target-triple>/<profile>/build/ctp2rs-<hash>/out/`
- `build.rs` 使用 `libclang` 解析 C++ 头文件，通过 `codegen/` 模块生成 Rust wrapper，同时用 `bindgen` 生成 FFI bindings
- `v1alpha1/mod.rs` 通过 `include!()` 宏引入生成文件
- 生成文件清单：`bindings.rs`、`mdapi.rs`、`mdspi.rs`、`traderapi.rs`、`traderspi.rs`、`event.rs`、`stream.rs`、`mod.rs`

## 版本与 Feature Flag

通过 Cargo feature 选择 CTP 版本：`ctp_v6_7_2`（默认）、`ctp_v6_7_7`、`ctp_v6_7_8`、`ctp_v6_7_9`、`ctp_v6_7_11`、`mini_v1_6_9`、`mini_v1_7_0`、`sopt_v3_7_3`。

不同版本/平台下生成的接口数量和参数签名各有差异。`build.rs` 中的 `get_sdk_path()` 根据 feature + 目标平台选择对应的 SDK 路径。

## 编码约定

- Rust 标准 `snake_case` 命名
- FFI unsafe 代码集中在 `ffi.rs` 和 codegen 生成的代码中，手写代码通过 safe wrapper 暴露接口
- 字符串编码统一使用 `ffi.rs` 中的 GB18030 工具函数
- 优先查阅 `examples/` 了解使用模式

## 测试与验证

- `cargo build -p ctp2rs` — 验证默认版本编译通过
- `cargo build -p ctp2rs --features ctp_v6_7_11 --no-default-features` — 交叉验证其他版本
- 运行示例需要连接 CTP 前置服务器，本地无法直接跑集成测试
- codegen 变更后建议对比 `OUT_DIR` 下生成文件的 diff
- 构建依赖 `libclang`，请确保系统已安装 LLVM/Clang

## 常见任务路径

| 任务 | 起点 |
|------|------|
| 修改 API wrapper 行为 | `codegen/stream.rs` 或 `codegen/setting.rs` |
| 添加新 CTP 版本支持 | `api/` 添加头文件和库 → `build.rs` 更新 `get_sdk_path()` → `Cargo.toml` 添加 feature |
| 修改 FFI 字符串处理 | `ctp-dyn/src/ffi.rs` |
| 添加使用示例 | `examples/` 下新建 crate，参考 `channel/` 结构 |
| 修改 codegen 解析逻辑 | `codegen/parser.rs` |
| 修改错误码生成 | `codegen/errors.rs` |

## 生成代码概览

> 以下为 ctp_v6_7_2 版本的结构概览，实际接口因版本和平台而异。

| 文件 | 内容 |
|------|------|
| `bindings.rs` | bindgen 生成的 FFI 类型（常量、`#[repr(C)]` 结构体、VTable） |
| `mdapi.rs` | `struct MdApi` — 行情 API（init/join/register_front/subscribe 等） |
| `mdspi.rs` | `trait MdSpi: Send` — 行情回调（on_front_connected/on_rtn_depth_market_data 等，约 13 个） |
| `traderapi.rs` | `struct TraderApi` — 交易 API（认证、报单、查询、银期转账等，约 90+ 个方法） |
| `traderspi.rs` | `trait TraderSpi: Send` — 交易回调（约 100+ 个，与 TraderApi 一一对应） |
| `event.rs` | `enum MdSpiEvent` / `enum TraderSpiEvent` — 回调事件枚举，用于 Stream 模式 |
| `stream.rs` | `MdSpiStream` / `TraderSpiStream` — 实现 `futures::Stream`，将 SPI 回调转为异步事件流 |
| `error.rs` | `enum CtpError` — CTP 错误码枚举（`from_code`/`code`/`message`） |

## 构建命令

```sh
# 默认构建（ctp_v6_7_2）
cargo build -p ctp2rs

# 指定版本构建
cargo build -p ctp2rs --features ctp_v6_7_11 --no-default-features

# 运行示例
cargo run -p channel
```
