# Agent Guide — ctp2rs

## 项目概述

ctp2rs 是上海期货信息技术公司 CTP/CTP-Mini/CTP-Sopt 接口的 Rust 原生封装。采用 `libloading` 在运行时动态加载 CTP 动态库（.so/.dll/.framework），通过 feature flag 切换不同柜台版本。

## Workspace 结构

```
ctp2rs/
├── ctp-dyn/          # 核心 crate（发布名 ctp2rs），包含 codegen、build.rs、API 头文件和动态库
│   ├── build.rs      # 编译期代码生成入口
│   ├── codegen/      # codegen 模块（parser、stream、errors、setting）
│   ├── api/          # 各版本 CTP/Mini/Sopt 的 C++ 头文件和预编译动态库
│   └── src/
│       ├── ffi.rs    # 公共 FFI 工具函数（GB18030 编解码、字符串转换等）
│       └── v1alpha1/ # 版本化接口模块
│           ├── mod.rs       # 通过 include!(concat!(env!("OUT_DIR"), ...)) 引入编译期生成的代码
│           ├── builder.rs   # 手写的 builder 辅助代码
│           └── generated/   # ⚠️ 仅供展示，不参与实际编译（见下方说明）
├── ctp-vt/           # 空占位 crate，暂无实现，可忽略
├── examples/         # 使用示例（多个独立 crate）
│   ├── channel/      # 基于 channel 的行情/交易示例
│   ├── insecure/     # 非安全连接示例
│   ├── localctp/     # LocalCTP 本地模拟示例
│   ├── openctp/      # OpenCTP 开放平台示例
│   └── tts_sopt/     # TTS 股票期权示例
└── Cargo.toml        # workspace 根配置
```

## ⚠️ 关键：代码生成机制

**所有与 CTP 接口相关的 Rust 绑定代码都是在编译期由 `build.rs` 动态生成的，不在源码树中。**

- 生成的代码位于编译产物路径：`target/<target-triple>/<profile>/build/ctp2rs-<hash>/out/`
- `ctp-dyn/src/v1alpha1/mod.rs` 通过 `include!(concat!(env!("OUT_DIR"), "/mdapi.rs"))` 等宏引入这些生成文件
- `build.rs` 使用 `libclang` 解析 C++ 头文件，再通过 `codegen/` 模块生成 Rust wrapper 代码，同时使用 `bindgen` 生成 FFI bindings
- 生成的文件包括：`bindings.rs`、`mdapi.rs`、`mdspi.rs`、`traderapi.rs`、`traderspi.rs`、`event.rs`、`stream.rs`、`mod.rs`

**`ctp-dyn/src/v1alpha1/generated/` 目录下的文件仅作为展示参考，不参与实际生产编译。** 如需查看真实生成的代码，请查阅 `build.rs` 和 `codegen/` 模块，或查看 `target/.../out/` 下的编译中间产物。

## 不同版本/平台的接口差异

每个 CTP 版本（v6.7.2、v6.7.7、v6.7.8、v6.7.9、v6.7.11）以及 Mini、Sopt 变体，在不同平台（Linux、Windows、macOS）上：
- 生成的接口数量不同
- 接口的参数签名各有差异
- 通过 Cargo feature flag 选择版本：`ctp_v6_7_2`（默认）、`ctp_v6_7_7`、`ctp_v6_7_11`、`mini_v1_6_9`、`sopt_v3_7_3` 等

`build.rs` 中的 `get_sdk_path()` 函数根据启用的 feature 和目标平台选择对应的 SDK 头文件路径。

## 如何了解使用方式

**请优先查阅 `examples/` 目录下的示例项目。** 每个示例都是独立的 crate，展示了不同场景下的用法：
- `examples/channel/` — 基于 channel 模式的行情订阅
- `examples/openctp/` — 对接 OpenCTP 开放平台
- `examples/localctp/` — 使用 LocalCTP 进行本地模拟
- `examples/insecure/` — 非安全连接场景
- `examples/tts_sopt/` — TTS 股票期权

## 生成代码接口摘要（基于 ctp_v6_7_2 版本示例）

> 以下签名仅供参考，实际接口因版本和平台而异。真实代码由 `build.rs` 在编译期生成到 `OUT_DIR`。

### bindings.rs（bindgen 生成）
由 `bindgen` 从 C++ 头文件生成的 FFI 类型定义，约 18000 行，包含：
- CTP 常量定义（`THOST_FTDC_*` 系列 `u8` 常量）
- C 结构体映射（`CThostFtdcReqUserLoginField`、`CThostFtdcDepthMarketDataField` 等数百个 `#[repr(C)]` 结构体）
- VTable 结构体（`CThostFtdcMdApi`、`CThostFtdcTraderApi` 的虚函数表）

### mdapi.rs — `struct MdApi`
行情 API 封装，核心方法：
```rust
pub fn release(&self)
pub fn init(&self)
pub fn join(&self) -> i32
pub fn get_trading_day(&self) -> String
pub fn register_front(&self, psz_front_address: &str)
pub fn register_spi(&self, p_spi: *mut dyn MdSpi)
pub fn subscribe_market_data(&self, pp_instrument_id: &[impl AsRef<str>]) -> i32
pub fn unsubscribe_market_data(&self, pp_instrument_id: &[impl AsRef<str>]) -> i32
pub fn req_user_login(&self, p_req_user_login_field: &mut CThostFtdcReqUserLoginField, n_request_id: i32) -> i32
pub fn req_user_logout(&self, p_user_logout: &mut CThostFtdcUserLogoutField, n_request_id: i32) -> i32
```

### mdspi.rs — `trait MdSpi: Send`
行情回调 trait，约 13 个回调方法：
```rust
fn on_front_connected(&mut self)
fn on_front_disconnected(&mut self, n_reason: i32)
fn on_rsp_user_login(&mut self, ..., n_request_id: i32, b_is_last: bool)
fn on_rtn_depth_market_data(&mut self, p_depth_market_data: Option<&CThostFtdcDepthMarketDataField>)
fn on_rtn_for_quote_rsp(&mut self, ...)
// ... 等
```

### traderapi.rs — `struct TraderApi`
交易 API 封装，约 90+ 个方法，涵盖：
- 连接管理：`release`, `init`, `join`, `register_front`, `register_spi`
- 认证登录：`req_authenticate`, `req_user_login`, `req_user_logout`
- 报单操作：`req_order_insert`, `req_order_action`, `req_batch_order_action`
- 查询接口：`req_qry_instrument`, `req_qry_trading_account`, `req_qry_investor_position` 等数十个
- 银期转账：`req_from_bank_to_future_by_future`, `req_from_future_to_bank_by_future`
- SPBM/RCAMS/RULE 保证金查询等

### traderspi.rs — `trait TraderSpi: Send`
交易回调 trait，约 100+ 个回调方法，与 TraderApi 的请求方法一一对应。

### event.rs
定义 `enum MdSpiEvent` 和 `enum TraderSpiEvent`，每个回调对应一个事件变体和事件结构体，用于 Stream 模式。

### stream.rs
提供 `MdSpiStream` 和 `TraderSpiStream`，实现 `futures::Stream` trait，将 SPI 回调转为异步事件流。

### error.rs — `enum CtpError`
CTP 错误码枚举，提供 `from_code(i32)`、`code() -> i32`、`message() -> &'static str` 方法。

## 构建与运行

```sh
# 默认构建（ctp_v6_7_2）
cargo build -p ctp2rs

# 指定版本构建
cargo build -p ctp2rs --features ctp_v6_7_11 --no-default-features

# 运行示例
cargo run -p channel
```

构建依赖 `libclang`，请确保系统已安装 LLVM/Clang。
