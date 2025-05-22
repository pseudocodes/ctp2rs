# TTS-SOPT Sample 

股票期权 openctp 环境的样例

## Run
```sh
> cd examples/tts_sopt/
> OPENCTP_USER_ID='your_openctp_account_id' OPENCTP_PASS='your_openctp_accout_password' cargo run 
```

## Output

```log
tdapi start here!
mdapi start here!
base dir:D:lproject\ctp2rs\examples\sopt
md dynlib path: D: \project\ctp2rs\examples\sopt\./tts/v3.7.3/20248918_traderapi64_windows_se/soptthostmduserapi_se.dll
base dir:D:projectictp2rslexamples\sopt
td dynlib path: D: \project\ctp2rs\examples\sopt\./tts/v3.7.3/20248918_traderapi64_windows_se/soptthosttraderapi_se.dll
get api version: openctp-ttsopt v3_7_0
get api version: openctp-ttsopt v3.7.8
mdapi init
md looptdapi init
td loop
mdspi.on_front_connected
tdspi.on_front_connected !!!
on_rsp_authenticate
ErrorID[0] Message[]
req_user_login result: 0
ErrorID[0] Message[成功]
on_rsp_user_login!
ErrorID[0] Message[]
on_rsp_sub_market_data: instrument_id["18888261"]，0, true
ErrorID[0] Message[]
reg_user_login_result: 0
ErrorID[0] Message[成功] 
OnRtnDepthMarketData.
26250522 10:11:16 6110008261 last price:6.2471
ErrorID[0] Message[]
on_rsp_sub_market_data: instrument id["10088265"],6,true
OnRtnDepthMarketData!
[20250522 11:13:53 0]10008265 last price: 8.0776
OnRtnDepthMarketData!
[20250522 11:13:53 010008265 last price: 6.8776
OnRtnDepthMarketData!
[20250522 10:11:16 6]10008261 last price: 0.2471
OnRtnDepthMarketData!
10088261 last price: 0.2471[20250522 10:11:16 6]
OnRtnDepthMarketData!
[26250522 11:13:53 6110008265 last price: 8.0776
OnRtnDepthMarketData!
[20250522 10:11:10 0] 10008261 last price: 0.2471
``` 