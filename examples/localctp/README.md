# LocalCTP Sample 

加载 LocalCTP 动态库的例子
源样例 ctp_query 来自 [localctp-sys](https://github.com/WhisperCapital/localctp-sys/blob/main/crates/examples/ctp_query.rs)

## Run
```sh
> cd examples/localctp/
> cargo run 
```

## Output

```log
[2024-12-11T07:01:36Z INFO  localctp] Start!
OpenSqlDB done!~
LocalCTP version: LocalCTP V1.0.0 By QiuShui(Aura) QQ1005018695
[2024-12-11T07:01:36Z DEBUG localctp::localctpx] register name_server ""
[2024-12-11T07:01:36Z INFO  localctp::localctpx] register front tcp://182.254.243.31:30001
[2024-12-11T07:01:36Z DEBUG localctp::localctpx] register done
[2024-12-11T07:01:36Z DEBUG localctp::localctpx] subscribe topic done
[2024-12-11T07:01:36Z DEBUG localctp::localctpx] init done
[2024-12-11T07:01:36Z INFO  localctp::localctpx] 前端连接成功回报 OnFrontConnected
[2024-12-11T07:01:36Z INFO  localctp::localctpx] call req_authenticate done
[2024-12-11T07:01:36Z INFO  localctp::localctpx] 认证成功回报 OnRspAuthenticate
[2024-12-11T07:01:36Z INFO  localctp::localctpx] 完成认证
[2024-12-11T07:01:36Z INFO  localctp::localctpx] 开始输入行情
[2024-12-11T07:01:36Z DEBUG localctp::localctpx] ag2406 -> 2645.64
[2024-12-11T07:01:36Z DEBUG localctp::localctpx] ag2406 -> 3360.10
[2024-12-11T07:01:37Z DEBUG localctp::localctpx] ag2406 -> 3854.10
[2024-12-11T07:01:37Z DEBUG localctp::localctpx] insert_limit_order: ag2406 -> 50000
Order successfully inserted.
[2024-12-11T07:01:37Z DEBUG localctp::localctpx] ag2406 -> 1255.42
[2024-12-11T07:01:38Z DEBUG localctp::localctpx] ag2406 -> 4064.46
[2024-12-11T07:01:38Z DEBUG localctp::localctpx] ag2406 -> 211.31
[2024-12-11T07:01:39Z DEBUG localctp::localctpx] ag2406 -> 1541.29
[2024-12-11T07:01:39Z INFO  localctp::localctpx] 其它回报
[2024-12-11T07:01:39Z INFO  localctp::localctpx] 报单成功回报 Order Return: BrokerID: 9999, InvestorID: 111111, ExchangeID: SHFE, OrderRef: 123, OrderStatus: 未知状态, InstrumentID: ag2406
[2024-12-11T07:01:39Z INFO  localctp::localctpx] 报单成功回报 Order Return: BrokerID: 9999, InvestorID: 111111, ExchangeID: SHFE, OrderRef: 123, OrderStatus: 未成交还在队列中, InstrumentID: ag2406
[2024-12-11T07:01:39Z INFO  localctp::localctpx] 成交回报 OnRtnTrade: OrderRef: 123, BrokerID: 9999, InvestorID: 111111, ExchangeID: SHFE, TradeID: 1733900496312, InstrumentID: ag2406, Price: 1019.79, Volume: 1
Quit the CSqliteHandler, let's sync for the last time...!
```