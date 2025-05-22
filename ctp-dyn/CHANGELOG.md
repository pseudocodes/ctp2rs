# Changelog

## [Unreleased] - 2025-05-22

### Changed

- 适配 Windows 动态库
- 更新 build.rs 
- Readme 更新
- 版本号升级至 0.1.7-alpha3


## [Unreleased] - 2025-05-19

### Changed

- 增加对环境变量 `CTP_API_INCLUDE_DIR` 的支持，方便开发者构建项目时替换不同版本的 CTP 头文件
- 更新 build.rs 
- 若 API 依赖路径不存在 `error.xml`, 则不会提供 `CtpError` 支持
- Readme 更新
- 版本号升级至 0.1.7-alpha1

## [Unreleased] - 2025-05-11

### Changed

- 添加对 `error.xml` 的解析，生成 `CtpError` 封装
- 更新 build.rs 
- Readme 更新
- 版本号升级至 0.1.6


## [Unreleased] - 2025-05-01

### Changed

- 更新添加 6.7.9 对应官方 CTP 库 Linux 版本
- 更新 build.rs 
- Readme 更新
- 版本号升级至 0.1.5


## [Unreleased] - 2025-02-19

### Changed

- 更新支持 ctp-mini v1.7.0 Linux 版
- 更新支持 ctp-sopt v3.7.3 Linux 版
- 更新 build.rs 
- Readme 更新
- 版本号升级至 0.1.4 

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



