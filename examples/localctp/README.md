# LocalCTP Sample 

## Realtime Sim
通过共享内存队列将实时行情数据导入 LocalCTP 模组，已实现实时交易模拟
LocalCTP 库实现来自 [gitub.com/pseudocodes/LocalCTP](https://github.com/pseudocodes/LocalCTP/tree/ndev)
共享内存队列采用 `boost/interprocess/message_queue` 封装
目前测试支持 `Linux/MacOS` 系统
### Run
```sh
> cd examples/localctp/
> cargo run -p localctp --bin sim
```

### Output
```log
[LocalCTP] Load local config file, running_mode:0(REALTIME_MODE), backtest_startdate:20250228, exit_after_settlement:0, settlement_time:17:00:00
OpenSqlDB done!~
Message queue initialized at path: 
MdReceiver thread started
[2025-09-26T14:20:52Z INFO  localctp::localctp2] Trader front connected.
[2025-09-26T14:20:52Z INFO  localctp::localctp2] Trader API initialized and connecting to front.
[2025-09-26T14:20:52Z INFO  localctp::localctp2] Authenticate successful.
[LocalCTP] tradingDay is empty, let's init it!
[2025-09-26T14:20:52Z INFO  localctp::localctp2] Login successful: SysVersion=, SystemName=LocalCTP
[INFO] C API: Creating MarketDataPublisher with name='md_queue', max_msg=1024, msg_size=1048576
[INFO] Creating MarketDataPublisher with queue='md_queue', max_msg=1024, msg_size=1048576
[INFO] Opened existing message queue: md_queue
[INFO] MarketDataPublisher initialized successfully for queue: md_queue
[INFO] C API: MarketDataPublisher wrapper created successfully
[2025-09-26T14:20:52Z INFO  localctp::localctp2] MD API initialized and connecting to front.
[2025-09-26T14:20:53Z INFO  localctp::localctp2] Login successful: SysVersion=, SystemName=
[2025-09-26T14:20:53Z INFO  localctp::localctp2] Subscribed to market data for instrument: "au2512"
[2025-09-26T14:20:53Z INFO  localctp::localctp2] Subscribed to market data for instrument: "ag2512"
[2025-09-26T14:20:53Z INFO  localctp::localctp2] Market Data - Instrument: au2512, LastPrice: 854.24, Volume: 248144
[2025-09-26T14:20:53Z INFO  localctp::localctp2] Market Data - Instrument: ag2512, LastPrice: 10405, Volume: 654462
[2025-09-26T14:20:53Z INFO  localctp::localctp2] Market Data - Instrument: au2512, LastPrice: 854.24, Volume: 248144
[2025-09-26T14:20:53Z INFO  localctp::localctp2] Market Data - Instrument: ag2512, LastPrice: 10404, Volume: 654590
[2025-09-26T14:20:54Z INFO  localctp::localctp2] Market Data - Instrument: au2512, LastPrice: 854.22, Volume: 248145
[2025-09-26T14:20:54Z INFO  localctp::localctp2] Market Data - Instrument: ag2512, LastPrice: 10404, Volume: 654596
[2025-09-26T14:20:54Z INFO  localctp::localctp2] Market Data - Instrument: au2512, LastPrice: 854.22, Volume: 248146
[2025-09-26T14:20:54Z INFO  localctp::localctp2] Market Data - Instrument: ag2512, LastPrice: 10406, Volume: 654598
[2025-09-26T14:20:55Z INFO  localctp::localctp2] Market Data - Instrument: au2512, LastPrice: 854.22, Volume: 248147
[2025-09-26T14:20:55Z INFO  localctp::localctp2] Market Data - Instrument: ag2512, LastPrice: 10406, Volume: 654605
[LocalCTP] tradingDay is 20250929
[LocalCTP] next Settlement Time is 2025-09-29 17:00:00
[2025-09-26T14:20:55Z INFO  localctp::localctp2] Market Data - Instrument: au2512, LastPrice: 854.22, Volume: 248147
[2025-09-26T14:20:55Z INFO  localctp::localctp2] Market Data - Instrument: ag2512, LastPrice: 10405, Volume: 654615
[2025-09-26T14:20:56Z INFO  localctp::localctp2] Market Data - Instrument: au2512, LastPrice: 854.28, Volume: 248161
[2025-09-26T14:20:56Z INFO  localctp::localctp2] Market Data - Instrument: ag2512, LastPrice: 10405, Volume: 654628
[2025-09-26T14:20:56Z INFO  localctp::localctp2] Market Data - Instrument: au2512, LastPrice: 854.26, Volume: 248166
[2025-09-26T14:20:56Z INFO  localctp::localctp2] Market Data - Instrument: ag2512, LastPrice: 10405, Volume: 654637
[2025-09-26T14:20:57Z INFO  localctp::localctp2] Market Data - Instrument: au2512, LastPrice: 854.32, Volume: 248174
[2025-09-26T14:20:57Z INFO  localctp::localctp2] Market Data - Instrument: ag2512, LastPrice: 10406, Volume: 654638
[2025-09-26T14:20:57Z INFO  localctp::localctp2] Market Data - Instrument: au2512, LastPrice: 854.32, Volume: 248176
[2025-09-26T14:20:57Z INFO  localctp::localctp2] Market Data - Instrument: ag2512, LastPrice: 10406, Volume: 654641
[2025-09-26T14:20:58Z INFO  sim] Inserted limit order successfully.
[2025-09-26T14:20:58Z INFO  localctp::localctp2] 报单成功回报 Order Return: BrokerID: 9999, InvestorID: 027011, ExchangeID: SHFE, OrderRef: 123, OrderStatus: 部分成交不在队列中, InstrumentID: ag2512
[2025-09-26T14:20:58Z INFO  localctp::localctp2] 报单成功回报 Order Return: BrokerID: 9999, InvestorID: 027011, ExchangeID: SHFE, OrderRef: 123, OrderStatus: 部分成交不在队列中, InstrumentID: ag2512
[2025-09-26T14:20:58Z INFO  localctp::localctp2] 成交回报 OnRtnTrade: OrderRef: 123, BrokerID: 9999, InvestorID: 027011, ExchangeID: SHFE, TradeID: 1758896453815, InstrumentID: ag2512, Price: 10406.00, Volume: 1
[2025-09-26T14:20:58Z INFO  localctp::localctp2] 报单成功回报 Order Return: BrokerID: 9999, InvestorID: 027011, ExchangeID: SHFE, OrderRef: 123, OrderStatus: 全部成交, InstrumentID: ag2512
[2025-09-26T14:20:58Z INFO  localctp::localctp2] Market Data - Instrument: au2512, LastPrice: 854.32, Volume: 248176
[2025-09-26T14:20:58Z INFO  localctp::localctp2] Market Data - Instrument: ag2512, LastPrice: 10406, Volume: 654641
[2025-09-26T14:20:58Z INFO  localctp::localctp2] Market Data - Instrument: au2512, LastPrice: 854.3, Volume: 248177
[2025-09-26T14:20:58Z INFO  localctp::localctp2] Market Data - Instrument: ag2512, LastPrice: 10406, Volume: 654641
```

## CTP Query
加载 LocalCTP 动态库的例子
源样例 ctp_query 来自 [localctp-sys](https://github.com/WhisperCapital/localctp-sys/blob/main/crates/examples/ctp_query.rs)
测试支持 `Linux/MacOS` 系统

### Run
```sh
> cd examples/localctp/
> cargo run -p localctp --bin query
```

### Output

```log
[LocalCTP] Load local config file, running_mode:0(REALTIME_MODE), backtest_startdate:20250228, exit_after_settlement:0, settlement_time:17:00:00
OpenSqlDB done!~
[2025-09-26T14:19:13Z INFO  query] Start query binary!
Message queue initialized at path: 
LocalCTP version: LocalCTP V1.0.0 By QiuShui(Aura) QQ1005018695
[2025-09-26T14:19:13Z DEBUG localctp::localctpx] register name_server ""
[2025-09-26T14:19:13Z INFO  localctp::localctpx] register front tcp://182.254.243.31:30001
[2025-09-26T14:19:13Z DEBUG localctp::localctpx] register done
[2025-09-26T14:19:13Z DEBUG localctp::localctpx] subscribe topic done
[2025-09-26T14:19:13Z DEBUG localctp::localctpx] init done
MdReceiver thread started
[2025-09-26T14:19:13Z INFO  localctp::localctpx] 前端连接成功回报 OnFrontConnected
[2025-09-26T14:19:13Z INFO  localctp::localctpx] call req_authenticate done
[2025-09-26T14:19:13Z INFO  localctp::localctpx] 认证成功回报 OnRspAuthenticate
[LocalCTP] tradingDay is empty, let's init it!
[2025-09-26T14:19:13Z INFO  localctp::localctpx] 完成认证
[2025-09-26T14:19:13Z INFO  localctp::localctpx] 开始输入行情
[2025-09-26T14:19:13Z DEBUG localctp::localctpx] ag2506 -> 894.05
[2025-09-26T14:19:13Z DEBUG localctp::localctpx] ag2506 -> 2853.48
No message received in timeout period
[2025-09-26T14:19:14Z DEBUG localctp::localctpx] insert_limit_order: ag2506 -> 50000
[2025-09-26T14:19:14Z DEBUG localctp::localctpx] ag2506 -> 398.69
[2025-09-26T14:19:14Z DEBUG localctp::localctpx] Order successfully inserted.
[2025-09-26T14:19:14Z DEBUG localctp::localctpx] ag2506 -> 3061.96
No message received in timeout period
[2025-09-26T14:19:15Z DEBUG localctp::localctpx] ag2506 -> 3389.04
[2025-09-26T14:19:15Z DEBUG localctp::localctpx] ag2506 -> 100.67
[LocalCTP] tradingDay is 20250929
[LocalCTP] next Settlement Time is 2025-09-29 17:00:00
[2025-09-26T14:19:16Z DEBUG localctp::localctpx] ag2506 -> 2584.06
No message received in timeout period
[2025-09-26T14:19:16Z INFO  localctp::localctpx] 其它回报
[2025-09-26T14:19:16Z INFO  localctp::localctpx] 报单成功回报 Order Return: BrokerID: 9999, InvestorID: 111111, ExchangeID: SHFE, OrderRef: 123, OrderStatus: 未知状态, InstrumentID: ag2506
[2025-09-26T14:19:16Z INFO  localctp::localctpx] 报单成功回报 Order Return: BrokerID: 9999, InvestorID: 111111, ExchangeID: SHFE, OrderRef: 123, OrderStatus: 未成交还在队列中, InstrumentID: ag2506
[2025-09-26T14:19:16Z INFO  localctp::localctpx] 成交回报 OnRtnTrade: OrderRef: 123, BrokerID: 9999, InvestorID: 111111, ExchangeID: SHFE, TradeID: 1758896353996, InstrumentID: ag2506, Price: 1774.65, Volume: 1
No message received in timeout period
Quit the CSqliteHandler, let's sync for the last time...!
```

