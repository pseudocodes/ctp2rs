# trader

CTP Trader 专用示例（不启用行情模块），演示交易端完整链条：

```
连接前置 → 客户端认证 → 用户登录 → 结算单确认
→ 查询资金账户 → 查询持仓 → 查询报单 → 查询成交 → 退出
```

SPI 回调通过 channel 转发到主线程，由主线程以事件驱动方式串联整个查询流程；
查询接口之间自带 1 秒等待以规避 CTP 查询流控。

## 配置

通过环境变量（支持 `.env` 文件）配置账户信息：

| 变量 | 必填 | 默认值 | 说明 |
|------|------|--------|------|
| `SIMNOW_USER_ID` | 是 | - | 投资者账号 |
| `SIMNOW_PASS` | 是 | - | 密码 |
| `CTP_BROKER_ID` | 否 | `9999` | 经纪商代码 |
| `CTP_APP_ID` | 否 | `simnow_client_test` | 客户端认证 AppID |
| `CTP_AUTH_CODE` | 否 | `0000000000000000` | 客户端认证授权码 |
| `CTP_TD_FRONT` | 否 | `tcp://182.254.243.31:40001` | 交易前置地址（SimNow 7x24） |

## 运行

```sh
# 默认使用 CTP v6.7.13（Linux / macOS / Windows）
cargo run -p trader

# 使用 v6.7.2
cargo run -p trader --no-default-features --features ctp_v6_7_2
```
