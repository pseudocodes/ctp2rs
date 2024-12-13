# Ctp2rs

采用 `libloading` 加载 `ctp/ctp-mini` 动态库的 CTP API 的 Rust 绑定封装
历史项目 `ctp-dyn`


## Usage

* **项目中添加相关库**
```shell
cargo add ctp2rs
```
或者在 `Cargo.toml` 中添加
```toml
[dependencies]
ctp2rs = "0.1.0"
# or below
ctp2rs = { git = "https://github.com/pseudocodes/ctp2rs", package = "ctp2rs" } 
```

* **切换 CTP API 版本**

已加入项目的 CTP 柜台版本
| version         | feature     | Linux | macOS | Windows |
| :-------------- | ----------- | ----- | ----- | :------ |
| ctp v6.7.2      | ctp_v6_7_2  | x     | x     |         |
| ctp v6.7.7      | ctp_v6_7_7  | x     | x     |         |
| ctp v6.7.8      | ctp_v6_7_8  | x     |       |         |
| ctp-mini v1.6.9 | mini_v1_6_9 | x     |       |         |


实际支持版本请查看 *[Cargo.toml](./Cargo.toml)* 中 `[features]` 字段
```toml
[dependencies]
ctp2rs = { version = "0.1.1", features = ["ctp_v6_7_7"] }
```



* **基本样例**

```rust
#[cfg(target_os = "macos")]
let dynlib_path = "../../ctp-dyn/api/v6.7.2/v6.7.2_MacOS_20231016/thostmduserapi_se.framework/thostmduserapi_se";

#[cfg(target_os = "linux")]
let dynlib_path = "../../ctp-dyn/api/v6.7.2/v6.7.2_20230913_api_traderapi_se_linux64/thostmduserapi_se.so";

let mdapi = MdApi::create_api(dynlib_path, "./md_", false, false);
...    
```


## Examples
See [/examples](../examples) for various uses of *`Ctp2rs`*. You can run them with:

```sh
cargo run --example <example>
```
- [channel](../examples/channel/): 采用 channel 来获取行情，支持 MacOS 以及 Linux 
- [localctp](../examples/localctp/): 运行 localctp 的一个样例，支持 Linux
- [openctp](../examples/openctp): 连接 openctp 模拟平台的行情以及交易样例，支持 MacOS 以及 Linux 
  



## Related Projects
|| **项目名称**| **链接**| **crate**| **支持平台** |**关系/特点**           |
|---| :--------------: | :-----|:------------------: | :------------:|------------------- |
|1| WiSaGaN/ctp-rs | [github.com/WiSaGaN/ctp-rs](https://github.com/WiSaGaN/ctp-rs) | N/A | Win64/Linux | 采用 vtable 映射实现 C++ 类的方法调用,  |
|2| SheldonNico/ctp-rs | [github.com/SheldonNico/ctp-rs](https://github.com/SheldonNico/ctp-rs)| N/A | Win64/Linux | 采用 C++ wrapper 代码作为桥接辅助封装 |
|3| rust-share | [github.com/mineralres/rust-share](https://github.com/mineralres/rust-share) | N/A| Win64/Linux | 首个提供了构建时解析 CTP 头文件并生成 Rust 封装代码的功能的项目，同时提供了异步 Stream 调用接口功能 |
|4| gqf2008/ctp-rs| [github.com/gqf2008/ctp-rs](https://github.com/gqf2008/ctp-rs) |N/A | Win64/Linux| 类似`2`, 采用 C++ 代码辅助封装 |
|5| libctp-sys| [github.com/unknown-marketwizards/libctp-sys](https://github.com/unknown-marketwizards/libctp-sys)| [libctp-sys](https://crates.io/crates/libctp-sys)| Win64/Linux | 类似`2`, 采用 C++ 代码辅助封装 |
|6| ctp-rust| [github.com/kozyan/ctp-rust](https://github.com/kozyan/ctp-rust)| N/A |Win64/Linux   | `2` 的分支项目 |
|7| ctp-alone | [github.com/calebx/ctp-alone](https://github.com/calebx/ctp-alone)| N/A |Win64/Linux | `1` 的分支项目 |
|8| ctp-sys | [https://github.com/daedalus2022/ctp-sys](https://github.com/daedalus2022/ctp-sys) |[ctp-sys](https://crates.io/crates/ctp-sys)| Win64/Linux| rust-share 的分支实现 |
|9| ctp-futures|[github.com/baiguoname/ctp-futures](https://github.com/baiguoname/ctp-futures)|[ctp-futures](https://crates.io/crates/ctp-futures)| Win64/Linux |rust-share 的分支实现 |
|10| localctp-sys| [github.com/WhisperCapital/localctp-sys](https://github.com/WhisperCapital/localctp-sys)|[localctp-sys](https://crates.io/crates/localctp-sys)| Win64/Linux| rust-share 分支实现，作者重写了封装代码生成模块，扩展性较强，仅适配 LocalCTP, 不支持官方版本|


### Potential Issues
* 以上项目均为静态编译项目，构建过程中需要二进制库参与编译，项目运行时需要通过环境变量指定动态库加载位置
* 当 CTP 升级版本时，开发者需要自行重新构建项目，用以升级依赖的 Cpp 头文件和依赖库
* 同上，当需要切换 CTP API 柜台时，部署时需要通过环境变量替换动态库，无法在同一个进程中访问多种柜台
* 基本不支持 MacOS 平台
  
  
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

## Notice
自动生成代码在构建中移入 `OUT_DIR` 目录, 项目中保留的文件实际不参与编译构建，仅作为参考展示

## Reference 

【1】*[OpenCTP](http://openctp.cn/download.html): 类上期技术 `Simnow` 开放模拟环境*

【2】*[LocalCTP](https://github.com/dearleeyoung/LocalCTP): 部署于本地的仿 CTP 项目*

【3】*[Go2CTP](https://github.com/pseudocodes/go2ctp): Go 语言加载动态库 CTP 接口封装*  

