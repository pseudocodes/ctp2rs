# Ctp2rs
上海期货信息技术公司 CTP/CTP-Mini/CTP-Sopt 接口 rust 语言原生封装


## Features
* 支持生产，测评，CTP-Mini, CTP-Sopt 股票期权, OpenCTP, LocalCTP 等兼容 CTP 接口的动态库版本, 适配 Linux, MacOS
* 采用 libloading 加载动态库，通过程序配置可以轻松切换不同柜台环境
* 编译指令自动切换 ctp 版本
* 保持与原生 ctp 接口一致的调用方式，更方便跨语言的项目转写
* 较为灵活的 codegen 脚本，方便开发者生成自己的封装接口

## Installation
To add the crate to an existing project, run the following command:
```sh
cargo add ctp2rs
```

## Usage 
```rust
#[cfg(target_os = "macos")]
let dynlib_path = "../../ctp-dyn/api/v6.7.2/v6.7.2_MacOS_20231016/thostmduserapi_se.framework/thostmduserapi_se";

#[cfg(target_os = "linux")]
let dynlib_path = "../../ctp-dyn/api/v6.7.2/v6.7.2_20230913_api_traderapi_se_linux64/thostmduserapi_se.so";

let mdapi = MdApi::create_api(dynlib_path, "./md_", false, false);
...    
```
更具体的使用内容请参考 `ctp-dyn` 目录下[文档](https://github.com/pseudocodes/ctp2rs/blob/master/ctp-dyn/README.md) 

## Examples

See [/examples](./examples) for various uses of Ctp2rs. You can run them with:

```sh
cargo run --example <example>
```

## Associative Project

对应 Go 语言封装项目 **[Go2ctp](https://github.com/pseudocodes/go2ctp)**

## 免责声明
本项目明确拒绝对产品做任何明示或暗示的担保。由于软件系统开发本身的复杂性，无法保证本项目完全没有错误。如选择使用本项目表示您同意错误和/或遗漏的存在，在任何情况下本项目对于直接、间接、特殊的、偶然的、或间接产生的、使用或无法使用本项目进行交易和投资造成的盈亏、直接或间接引起的赔偿、损失、债务或是任何交易中止均不承担责任和义务。


