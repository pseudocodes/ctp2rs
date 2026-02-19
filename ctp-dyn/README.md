
[![GitHub Site]][github] [![Crates.io Version]][crates.io] [![Ask DeepWiki]][deepwiki] [![Rust DOC]][rust doc link]

[Crates.io Version]: https://img.shields.io/crates/v/ctp2rs
[crates.io]: https://crates.io/crates/ctp2rs
[Github Site]: https://img.shields.io/badge/Github-pseudocodes/ctp2rs-green&logo=github
[github]: https://github.com/pseudocodes/ctp2rs
[Ask DeepWiki]: https://deepwiki.com/badge.svg
[deepwiki]: https://deepwiki.com/pseudocodes/ctp2rs
[Rust Doc]: https://img.shields.io/docsrs/ctp2rs/latest
[rust doc link]: https://docs.rs/ctp2rs


# Ctp2rs

采用 `libloading` 加载 `ctp/ctp-mini/ctp-sopt` 动态库的 CTP API 的 Rust 绑定封装
历史项目 `ctp-dyn`


## Usage

* **项目中添加相关库**
```shell
cargo add ctp2rs
```
或者在 `Cargo.toml` 中添加
```toml
[dependencies]
ctp2rs = "0.1"
# or below
ctp2rs = { git = "https://github.com/pseudocodes/ctp2rs", package = "ctp2rs" } 
```

* **切换 CTP API 版本**

已加入项目的 CTP 柜台版本
| version         | feature      | Linux | macOS | Windows |
| :-------------- | ------------ | ----- | ----- | ------- |
| ctp v6.7.2      | ctp_v6_7_2   | x     | x     |   x     |
| ctp v6.7.7      | ctp_v6_7_7   | x     | x     |   x     |
| ctp v6.7.8      | ctp_v6_7_8   | x     |       |   x     |
| ctp v6.7.9      | ctp_v6_7_9   | x     |       |   x     |
| ctp v6.7.11     | ctp_v6_7_11  | x     |       |   x     |
| ctp-mini v1.6.9 | mini_v1_6_9  | x     |       |   x     |
| ctp-mini v1.7.0 | mini_v1_7_0  | x     |       |   x     |
| ctp-sopt v3.7.3 | sopt_v3_7_3  | x     |       |   x     |


实际支持版本请查看 *[Cargo.toml](./Cargo.toml)* 中 `[features]` 字段，或开发者可以采用环境变量来指定具体绑定的 CTP API 版本
`0.1.7` 版本后将不再在项目中跟踪保存 CTP 各版本头文件，除遇各平台主要接口变更

默认采用的 ctp sdk 版本停留在 `v6.7.2`，便于各个架构进行测试开发，`v6.7.11` 版本由于合并了生产和评测版本并在主创建实例接口添加了 `bIsProductionMode` 参数，所以需要在开发时用户自行指定;

```toml
[dependencies]
ctp2rs = { version = "0.1", features = ["ctp_v6_7_7"] } 
```
或者
```toml
[dependencies]
ctp2rs = { version = "0.1.8-alpha1", features = ["ctp_v6_7_11"] } 
```


当前已实现动态链接库的无版本依赖加载方案, Demo 可查看 [example/insecure](../examples/insecure/)，这可能是跨版本兼容封装的极限解决方案 

* **环境变量动态依赖构建**

通过环境变量 `CTP_API_INCLUDE_DIR` 可以在构建项目时指定所依赖的 CTP API 版本，有以下约定
  * 环境变量一般设定为绝对路径, 或者以当前构建目录为基准的相对路径
  * 指定的依赖目录里的头文件或者 `error.xml` 需要转码至 `UTF8`
  
```shell
> CTP_API_INCLUDE_DIR=/absolute/path/to/your/ctp/api/ cargo build 
```

* **基本样例**

```rust
#[cfg(target_os = "macos")]
let dynlib_path = "../../ctp-dyn/api/ctp/v6.7.2/v6.7.2_MacOS_20231016/thostmduserapi_se.framework/thostmduserapi_se";

#[cfg(target_os = "linux")]
let dynlib_path = "../../ctp-dyn/api/ctp/v6.7.2/v6.7.2_20230913_api_traderapi_se_linux64/thostmduserapi_se.so";

#[cfg(target_os = "windows")]
let dynlib_path = "../../ctp-dyn/api/ctp/v6.7.2/v6.7.2_20230913_api_traderapi64_se_windows/thostmduserapi_se.dll"

let mdapi = MdApi::create_api(dynlib_path, "./md_", false, false);
...    
```


## Examples
See [/examples](../examples) for various uses of *`Ctp2rs`*. You can run them with:

```sh
cargo run --example <example>
```
- [channel](../examples/channel/): 采用 channel 来获取行情，支持 MacOS 以及 Linux 
- [localctp](../examples/localctp/): 运行 localctp 的一个样例，支持 Linux，localctp 库提供特殊定制，支持共享内存导入行情
- [openctp](../examples/openctp): 连接 openctp 模拟平台的行情以及交易样例，支持 MacOS 以及 Linux 
- [insecure](../examples/insecure): 可跨版本加载动态方案 POC Demo, 适用 Linux
- [tts_sopt](../examples/tts_sopt): ctp-sopt 连接 openctp 股票期权仿真环境 demo 
  
### open-md-gateway

一个多源行情网关的实现 [open-md-gateway](https://github.com/pseudocodes/open-md-gateway)

通过对 [QAMD Gateway](https://github.com/QUANTAXIS/qautlra-rs/tree/master/qamdgateway) 进行重构，简化对 `ctp` 库的依赖和构建


## Related Projects
|| **项目名称**| **链接**| **crate**| **支持平台** |**关系/特点**           |
|---| :--------------: | :-----|:------------------: | :------------:|------------------- |
|1| WiSaGaN/ctp-rs | [github.com/WiSaGaN/ctp-rs](https://github.com/WiSaGaN/ctp-rs) | N/A | Win64/Linux | 采用 vtable 映射实现 C++ 类的方法调用, 已归档不在 github 更新 |
|2| SheldonNico/ctp-rs | [github.com/SheldonNico/ctp-rs](https://github.com/SheldonNico/ctp-rs)| N/A | Win64/Linux | 采用 C++ wrapper 代码作为桥接辅助封装; 20250630 后参考 `ctp2rs` 构建方式，支持了环境变量头文件编译，由 rust 重写 python 构建代码|
|3| rust-share | [github.com/mineralres/rust-share](https://github.com/mineralres/rust-share) | N/A| Win64/Linux | 首个提供了构建时解析 CTP 头文件并生成 Rust 封装代码的功能的项目，同时提供了异步 Stream 调用接口功能 |
|4| gqf2008/ctp-rs| [github.com/gqf2008/ctp-rs](https://github.com/gqf2008/ctp-rs) |N/A | Win64/Linux| 类似`2`, 采用 C++ 代码辅助封装 |
|5| libctp-sys| [github.com/unknown-marketwizards/libctp-sys](https://github.com/unknown-marketwizards/libctp-sys)| [libctp-sys](https://crates.io/crates/libctp-sys)| Win64/Linux | 类似`2`, 采用 C++ 代码辅助封装 |
|6| kozyn/ctp-rust| [github.com/kozyan/ctp-rust](https://github.com/kozyan/ctp-rust)| N/A |Win64/Linux   | `2` 的分支项目 |
|7| ctp-alone | [github.com/calebx/ctp-alone](https://github.com/calebx/ctp-alone)| N/A |Win64/Linux | `1` 的分支项目 |
|8| ctp-sys | [https://github.com/daedalus2022/ctp-sys](https://github.com/daedalus2022/ctp-sys) |[ctp-sys](https://crates.io/crates/ctp-sys)| Win64/Linux| rust-share 的分支实现 |
|9| ctp-futures|[github.com/baiguoname/ctp-futures](https://github.com/baiguoname/ctp-futures)|[ctp-futures](https://crates.io/crates/ctp-futures)| Win64/Linux |rust-share 的分支实现 |
|10| localctp-sys| [github.com/WhisperCapital/localctp-sys](https://github.com/WhisperCapital/localctp-sys)|[localctp-sys](https://crates.io/crates/localctp-sys)| Win64/Linux| rust-share 分支实现，作者重写了封装代码生成模块，扩展性较强，仅适配 LocalCTP, 不支持官方版本|
|11| RTP| [github.com/glacierx/RTP](https://github.com/glacierx/RTP)|[rptx](https://crates.io/crates/rtpx)| Linux| `1` 分支实现, 仅实现了 TraderApi 绑定|
|12| rn7s2/ctp-rs| [https://github.com/rn7s2/ctp-rs](https://github.com/rn7s2/ctp-rs)|[ctp-rs](https://crates.io/crates/ctp-rs)| Win64/Linux| 原名 `ctp4rs` 后占据 `ctp-rs` crate, 高度定制化的静态 C++ 代码辅助封装，采用了 `cxx-build` 作为 C++ 代码桥接工具, 仅支持 ctp 最新版本|
|13| myctp-rs  | [https://github.com/shawn666liu/myctp-rs](https://github.com/shawn666liu/myctp-rs)| N/A | Win64/Linux| 基于作者的纯 C 封装项目 [ftdc2c_ctp](https://github.com/shawn666liu/ftdc2c_ctp) 之上的 Rust 封装，项目尝试提供多种高度抽象的封装范式，编译构建较为繁琐，建议采用大模型解读|
|14| deepissue/ctp-rust| [https://github.com/deepissue/ctp-rust](https://github.com/deepissue/ctp-rust) | [ctp-rust](https://crates.io/crates/ctp-rust)|Linux/MacOS | 项目封装方案估计由大模型提供，未清晰理解 CTP 具体环境和业务状态，仅从 rust 语言层面提供了技术封装方案，接口 api 函数未全部实现封装，代码存有 TODO, MacOS 接口封装无法通过穿透式认证|


### Potential Issues
* 以上项目均为静态编译项目，构建过程中需要二进制库参与编译，项目运行时需要通过环境变量指定动态库加载位置
* 当 CTP 升级版本时，开发者需要自行重新构建项目，用以升级依赖的 Cpp 头文件和依赖库
* 同上，当需要切换 CTP API 柜台时，部署时需要通过环境变量替换动态库，无法在同一个进程中访问多种柜台
* 较少项目支持 MacOS 平台
  
  
## Development Environment
以下开发环境编译成功
* **MacOS (Apple Silicon)**
```shell
> clang --version 
Apple clang version 16.0.0 (clang-1600.0.26.4)
Target: arm64-apple-darwin23.6.0
> rustc --version
rustc 1.81.0 (eeb90cda1 2024-09-04)
```
* **Ubuntu 22.04.1 LTS (Aliyun ecs)**
```shell
> clang --version
Ubuntu clang version 14.0.0-1ubuntu1
Target: x86_64-pc-linux-gnu
> g++ --version
g++ (Ubuntu 11.2.0-19ubuntu1) 11.2.0
> rustc --version
rustc 1.77.2 (25ef9e3d8 2024-04-09)
```
* **Windows**
```shell
> clang -v
clang version 18.1.8
Target: x86_64-pc-windows-msvc
Thread model: posix
> rustc --version
rustc 1.87.0 (17067e9ac 2025-05-09)
```

## Notice
自动生成代码在构建中移入 `OUT_DIR` 目录, 项目中保留的文件实际不参与编译构建，仅作为参考展示

## Acknowledgments 
[@ZerounNet](https://github.com/ZerounNet): Windows 开发环境适配

## Reference 

【1】*[OpenCTP](http://openctp.cn/download.html): 类上期技术 `Simnow` 开放模拟环境*

【2】*[LocalCTP](https://github.com/dearleeyoung/LocalCTP): 部署于本地的仿 CTP 项目*

【3】*[Go2CTP](https://github.com/pseudocodes/go2ctp): Go 语言加载动态库 CTP 接口封装*  

