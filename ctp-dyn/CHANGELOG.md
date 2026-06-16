# Changelog

## [Unreleased] - 2026-06-17

### Added
- 添加 CTP-Mini v1.7.5 SDK 头文件和预编译库（Linux / Windows），新增 `mini_v1_7_5` feature 及对应 `build.rs` SDK 路径分支
- 添加 CTP-Sopt v3.7.5 SDK 头文件和预编译库（Linux / Windows），新增 `sopt_v3_7_5` feature 及对应 `build.rs` SDK 路径分支
- 新增 codegen 分析、重构计划和静态代码生成设计文档

### Changed
- 默认 feature 调整为仅启用 `v1alpha1`，SDK 版本 feature 由用户或示例显式选择；未选择版本时仍由 `build.rs` fallback 到 v6.7.7
- `build.rs` 新增 API 层 feature 校验，要求 `v1alpha1` / `v1alpha2` 二选一，避免关闭默认 feature 后生成目录版本缺失
- `examples/*` 统一关闭 `ctp2rs` 默认 feature，并显式转发 `ctp2rs/v1alpha1` 与目标 SDK 版本 feature
- `openctp` 示例补齐 `ctp_v6_7_13` feature、动态库路径和 `create_api` 新签名分支
- `tts_sopt` 示例动态库路径切换到 v3.7.3 目录，并更新 OpenCTP 前置地址
- codegen 指针参数分类兼容 `CharU` / `SChar` / `UChar`，避免部分平台头文件中的字符指针被误判为结构体指针
- 更新 README 版本支持表格和 feature 使用说明，反映当前可用版本及显式版本选择方式
- 版本号升级至 **0.1.10-alpha4**

### Removed
- 移除 CTP-Mini v1.6.9 feature 与 SDK 资产，改由 v1.7.5 接替


## [Unreleased] - 2026-05-02

### Added
- 添加 CTP v6.7.13 SDK 头文件和预编译库（Linux / Windows）
- `Cargo.toml` 新增 `ctp_v6_7_13` feature（启用 `dynlib` + `union`）
- `build.rs` 新增 `ctp_v6_7_13` 分支及互斥检查条目
- `openctp` 示例适配 v6.7.13 的 `subscribe_private_topic` 新增参数（`#[cfg]` 条件编译）

### Changed
- 默认 feature 调整为 `ctp_v6_7_7`（原先无指定版本，依赖 fallback）
- 更新 README 版本支持表格，反映当前可用版本
- 版本号升级至 **0.1.10-alpha3**

### Removed
- 移除 `ctp_v6_7_8`、`ctp_v6_7_9` feature 及对应 `build.rs` 分支（SDK 不再随仓库分发）
- `openctp` 示例移除 `ctp_v6_7_8` / `ctp_v6_7_9` feature 选项


## [Unreleased] - 2026-03-10

### Changed
- default feature 不再包含版本选择（移除 `ctp_v6_7_7`），`build.rs` 在无版本 feature 时自动 fallback 到 v6.7.7
- `build.rs` 版本匹配优先级调整：显式指定的版本优先匹配，fallback 版本（v6.7.7）降至最后，缓解 Cargo workspace feature unification 导致的签名冲突
- `ctp_v6_7_8` / `ctp_v6_7_9` 新增 openctp 支持，macOS 下使用对应版本的 Linux 头文件编译
- macOS fallback 路径新增 openctp 判断，启用时使用 Linux 头文件
- `localctp` 示例锁定 `ctp_v6_7_2` 版本
- `openctp` 示例新增 `ctp_v6_7_8` / `ctp_v6_7_9` feature 选项


## [Unreleased] - 2026-03-04

### Changed
- **codegen 架构重构**：从 handler 注册表 + 多 Pass 遍历架构重构为 IR 管线（AST → MethodInfo → Rust code），codegen 源码从约 1920 行精简至约 1080 行（减少约 44%）
- **codegen 模块重组**：删除 `setting.rs`（Config/Context/Handler）和原 `parser.rs`（1029 行单体文件），拆分为职责清晰的三个模块：
  - `parser.rs` — IR 数据结构定义 + C++ AST 提取
  - `generator.rs` — IR → Rust 代码生成
  - `naming.rs` — XML 错误 ID → Rust 枚举名转换
- **移除 event/stream 代码生成**：构建期不再生成 `event.rs` 和 `stream.rs`（`MdSpiEvent`/`TraderSpiEvent` 枚举及 `futures::Stream` 异步流封装）；用户可在应用层通过 `std::sync::mpsc` 自行实现等效功能（参见 `examples/channel`）
- **示例适配**：`examples/channel` 改用本地 `MdEvent` 枚举 + `std::sync::mpsc::sync_channel`；`examples/localctp` 改用本地 `ChannelTraderSpi` + `TraderEvent` 枚举，通过 `mpsc::unbounded_channel` 桥接 tokio
- **生成代码展示方式变更**：`generated/` 目录下的 `.rs` 文件替换为 `README.md`，以代码块引用方式展示各生成文件节选，重点标注跨平台签名差异（如 macOS `ReqUserLogin` 额外的 `length`/`systemInfo` 参数），避免误导 AI agent 或开发者将展示文件当作编译源码

### Removed
- `codegen/setting.rs`、`codegen/stream.rs`
- `futures`、`parking_lot` 运行时依赖
- `examples/localctp` 的 `futures` 依赖
- `errors.rs` 中未使用的编码检测函数（`read_file_with_encoding_detection`、`extract_encoding_from_xml_declaration`、`detect_encoding`）
- `generated/` 目录下的 `.rs` 展示文件


## [Unreleased] - 2026-02-11

### Fixed
- 修复 `MdApi::default()` / `TraderApi::default()` 零初始化 UB，改为安全的字段级初始化
- `Cell<*mut T>` 替换为 `AtomicPtr<T>`，消除 `unsafe impl Sync` 下的数据竞争风险
- 修复 Drop 中 Release 与 SPI 释放的竞态风险，调整为先 Release 再释放 SPI 内存
- Stream 的 `std::sync::Mutex` 替换为 `parking_lot::Mutex`，消除 poison panic 风险
- 修复 `MdApiBuilder::build()` 在 union 分支中未保存 dynlib 导致动态库提前卸载的问题

### Changed
- Subscribe/Unsubscribe 系列方法参数类型从 `&[String]` 改为 `&[impl AsRef<str>]`
- Stream 缓冲区新增有界队列背压控制（默认容量 65536，满时丢弃最旧事件）
- `build.rs` 新增 CTP 版本 feature 互斥检测，同时启用多个版本时发出编译警告
- `MdApiBuilder` / `TraderApiBuilder` 的 `with_dynlib` 方法参数改为 `AsRef<Path>`
- `MdApiBuilder::Default` 手动实现，确保 `use_production_mode` 默认为 `true`
- `.gitignore` 新增 `*.dylib` 及项目分析文档的忽略规则

### Added
- `register_spi` 方法新增 `/// # Safety` 文档注释，明确调用者的生命周期责任
- `ffi.rs` 新增 `DecodeString` trait（`decode()` / `try_decode()`），不与 `std::toString` 冲突
- `ffi.rs` 新增 `DynLibKind` 枚举和 `resolve_dynlib_path()` 工具函数，统一跨平台动态库路径解析
- 新增 `TraderApiBuilder`，与 `MdApiBuilder` 对称的交易 API 构建器
- `builder.rs` 全部公开方法和结构体新增文档注释及示例
- 新增 `parking_lot` 依赖

## [Unreleased] - 2025-09-26
### Changed
-  Readme 更新
-  LocalCTP 样例更新，支持 MacOS 环境，添加共享内存行情导入 case
-  版本号升级至 **0.1.8**


## [Unreleased] - 2025-09-19
### Changed
- Readme 更新
- 更新 MdApi 订阅相关接口的参数
- 修复 Api Drop 可能产生的重复 Release 问题
- 版本号升级至 **0.1.8-alpha3**


## [Unreleased] - 2025-08-13
### Changed
- Readme 更新
- 添加对 ctp 6.7.11 版本的支持，ctp 实例创建接口新增 `production_mode` 参数
- 版本号升级至 **0.1.8-alpha1**


## [Unreleased] - 2025-07-14
### Changed
- Readme 更新
- openctp sample 更新，支持 openctp 仿真环境

## [Unreleased] - 2025-05-22

### Changed

- 适配 Windows 动态库
- 更新 build.rs 
- Readme 更新
- 版本号升级至 **0.1.7-alpha3**


## [Unreleased] - 2025-05-19

### Changed

- 增加对环境变量 `CTP_API_INCLUDE_DIR` 的支持，方便开发者构建项目时替换不同版本的 CTP 头文件
- 更新 build.rs 
- 若 API 依赖路径不存在 `error.xml`, 则不会提供 `CtpError` 支持
- Readme 更新
- 版本号升级至 **0.1.7-alpha1**

## [Unreleased] - 2025-05-11

### Changed

- 添加对 `error.xml` 的解析，生成 `CtpError` 封装
- 更新 build.rs 
- Readme 更新
- 版本号升级至 **0.1.6**


## [Unreleased] - 2025-05-01

### Changed

- 更新添加 6.7.9 对应官方 CTP 库 Linux 版本
- 更新 build.rs 
- Readme 更新
- 版本号升级至 **0.1.5**


## [Unreleased] - 2025-02-19

### Changed

- 更新支持 ctp-mini v1.7.0 Linux 版
- 更新支持 ctp-sopt v3.7.3 Linux 版
- 更新 build.rs 
- Readme 更新
- 版本号升级至 **0.1.4** 

## [Unreleased] - 2024-12-13

### Changed

- 更新支持 CTP-Mini v1.6.9 Linux 版
- 更新 build.rs 
- 自动生成代码移入 `OUT_DIR` 目录, 项目中保留的文件实际不参与编译构建，仅作为学习展示


## [Unreleased] - 2024-12-12

### Changed

- 更新添加 6.7.8 对应官方 CTP 库 Linux 版本

## [Unreleased] - 2024-12-11

### Changed

- 更新 Codegen stream 模块代码，增加构造函数
- 增加 localctp 样例代码

## [Unreleased] - 2024-12-10

- 公开 Repo, 发布 ctp2rs v0.1.0 版本
- 添加 OpenCTP 样例
- 添加 Channel 样例



