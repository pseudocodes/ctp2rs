# Changelog

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



