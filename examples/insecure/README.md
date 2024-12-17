# Insecure

这是一个 **POC (Proof of Concept) Demo** ，用来探索展示如何利用 `CTP API` 的向后兼容性进行无视版本的动态库加载封装方法

**原理:**

* Linux 下直接采用动态库的内部符号加载，idea 来自 [WiSaGaN/ctp-rs](https://github.com/WiSaGaN/ctp-rs/blob/master/ctp-trader/src/lib.rs#L18)
* MacOS 的动态库属于 `Mach-O` 格式，并且上面 Linux 采用的动态库符号均属于未导出符号，无法采用 `dlsym` 获取
  * MacOS 环境下需要在加载 `framework` 动态库时解析提取并记录相关 Symbol 在动态库中的偏移量
  * 程序运行时通过 dyld 相关接口获得加载的动态库基址，计算出对应 Symbol 在内存中的位置
  * 通过上面计算出的 Symbol 内存位置，映射并实例化函数即可调用
  * 以上方法同样适用 Linux 环境

**可能产生的问题:**

*   获得的函数可能没有并发保护

Linux 以及 MacOS 完整的封装脚本以及最终封装解决方案将不再公开在本项目


## Run
```shell
> cd examples/insecure/
> SIMNOW_USER_ID={your_simnow_account_id} SIMNOW_PASS={your_simow_account_password} cargo run
```

## Output

```log
tdapi start here!
base_dir: 
get_api_version: v6.7.2_20230913 10:48:10.4926
tdapi init
tdspi.on_front_connected !!!
on_rsp_authenticate
ErrorID[0] Message[正确]
req_user_login result: 0
ErrorID[0] Message[正确]
```