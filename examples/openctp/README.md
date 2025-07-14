# OpenCTP Sample 

加载 OpenCTP 动态库的例子

## OPENCTP 仿真环境
行情连接实盘，交易连接 `OPENCTP` 柜台

### Run
```bash
> cd examples/openctp/
> cargo run -- --environment sim --user-id 'your_openctp_account_id' --password 'your_openctp_accout_password'
```

### Output
```log
Running with environment: Sim
mdapi start here!
md dynlib_path: ./git/pseudo/ctp2rs/examples/openctp/../../../ctp-dyn/api/ctp/v6.7.2/v6.7.2_MacOS_20231016/thostmduserapi_se.framework/thostmduserapi_se
tdapi start here!
td dynlib_path: ./git/pseudo/ctp2rs/examples/openctp/tts/v6_7_2/mac_arm64/thosttraderapi_se.dylib
md get_api_version: v6.7.2_MacOS_20231016 15:00:00
mdapi init
md loop
td get_api_version: V6_7_2
tdapi init
td loop
tdspi.on_front_connected !!!
on_rsp_authenticate
ErrorID[0] Message[]
req_user_login result: 0
ErrorID[0] Message[成功]
req_user_login result: 0
ErrorID[0] Message[]
mdspi.on_front_connected
ErrorID[0] Message[CTP:No Error]
on_rsp_user_login!
ErrorID[0] Message[CTP:No Error]
on_rsp_sub_market_data: instrument_id["ag2510"], 0, false
ErrorID[0] Message[CTP:No Error]
on_rsp_sub_market_data: instrument_id["au2510"], 0, true
OnRtnDepthMarketData!
[20250714 11:29:57 0] ag2510 last_price: 9198
OnRtnDepthMarketData!
[20250714 11:29:57 0] au2510 last_price: 778.3000000000001
req_qry_investor_position result: 0
ErrorID[0] Message[成功]
position hold None
```

## TTS 环境
行情以及交易均连接 `OPENCTP` 柜台

### Run
```bash
> cd examples/openctp/
> cargo run -- --environment tts --user-id 'your_openctp_account_id' --password 'your_openctp_accout_password'
```

### Output

```log
Running with environment: Tts
mdapi start here!
md dynlib_path: ./git/pseudo/ctp2rs/examples/openctp/tts/v6_7_2/mac_arm64/thostmduserapi_se.dylib
tdapi start here!
td dynlib_path: ./git/pseudo/ctp2rs/examples/openctp/tts/v6_7_2/mac_arm64/thosttraderapi_se.dylib
md get_api_version: V6_6_9
td get_api_version: V6_7_2
mdapi init
md loop
tdapi init
td loop
mdspi.on_front_connected
tdspi.on_front_connected !!!
on_rsp_authenticate
ErrorID[0] Message[]
req_user_login result: 0
ErrorID[0] Message[成功]
on_rsp_user_login!
ErrorID[0] Message[]
on_rsp_sub_market_data: instrument_id["ag2510"], 0, true
OnRtnDepthMarketData!
[20250714 10:36:32 0] ag2510 last_price: 9216
ErrorID[0] Message[]
on_rsp_sub_market_data: instrument_id["au2510"], 0, true
OnRtnDepthMarketData!
[20250714 10:36:32 0] au2510 last_price: 778.1200000000001
ErrorID[0] Message[成功]
req_user_login result: 0
ErrorID[0] Message[]
OnRtnDepthMarketData!
[20250714 10:36:32 0] ag2510 last_price: 9216
OnRtnDepthMarketData!
[20250714 10:36:32 0] au2510 last_price: 778.1
OnRtnDepthMarketData!
[20250714 10:36:33 0] ag2510 last_price: 9216
OnRtnDepthMarketData!
[20250714 10:36:33 0] au2510 last_price: 778.1200000000001
req_qry_investor_position result: 0
ErrorID[0] Message[成功]
position hold None
on_rsp_qry_investor_position finish!
OnRtnDepthMarketData!
[20250714 10:36:33 0] ag2510 last_price: 9215
OnRtnDepthMarketData!
[20250714 10:36:33 0] au2510 last_price: 778.1
OnRtnDepthMarketData!
[20250714 10:36:34 0] ag2510 last_price: 9215
OnRtnDepthMarketData!
[20250714 10:36:34 0] au2510 last_price: 778.1400000000001
OnRtnDepthMarketData!
[20250714 10:36:34 0] ag2510 last_price: 9216
```

