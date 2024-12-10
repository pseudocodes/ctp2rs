# OpenCTP Sample 

加载 OpenCTP 动态库的例子

## Run
```sh
> cd examples/openctp/
> OPENCTP_USER_ID='your_openctp_account_id' OPENCTP_PASS='your_openctp_accout_password' cargo run 
```

## Output

```log
tdapi start here!
mdapi start here!
base_dir: /Users/neuron/git/pseudo/ctp2rs/examples/openctp
base_dir: /Users/neuron/git/pseudo/ctp2rs/examples/openctp
dynlib_path: /Users/neuron/git/pseudo/ctp2rs/examples/openctp/./tts/v6_7_2/mac_arm64/thostmduserapi_se.dylib
get_api_version: V6_7_2
get_api_version: V6_6_9
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
on_rsp_sub_market_data: instrument_id["ag2502"], 0, true
ErrorID[0] Message[成功]
req_user_login result: 0
ErrorID[0] Message[]
OnRtnDepthMarketData!
[20241210 09:56:13 0] ag2502 last_price: 7944
ErrorID[0] Message[]
on_rsp_sub_market_data: instrument_id["fu2505"], 0, true
OnRtnDepthMarketData!
[20241210 09:56:12 0] fu2505 last_price: 3002
OnRtnDepthMarketData!
[20241210 09:56:14 0] ag2502 last_price: 7944
OnRtnDepthMarketData!
[20241210 09:56:14 0] ag2502 last_price: 7944
OnRtnDepthMarketData!
[20241210 09:56:15 0] ag2502 last_price: 7943
req_qry_investor_position result: 0
ErrorID[0] Message[成功]
position hold None
on_rsp_qry_investor_position finish!
```