# Channel Sample 

使用 channel 来实现行情获取的 sample 

## Run
```shell
> cd examples/channel/
> SIMNOW_USER_ID={your_simnow_account_id} RUST_LOG=debug cargo run
```

## Output

```log
[2024-12-10T03:05:09Z DEBUG channel] on_front_connected
[2024-12-10T03:05:09Z DEBUG channel] on_rsp_user_login
[2024-12-10T03:05:09Z DEBUG channel] on_rsp_sub_market_data
[2024-12-10T03:05:09Z DEBUG channel] on_rsp_sub_market_data
ErrorID[0] Message[CTP:No Error]
[2024-12-10T03:05:09Z INFO  channel] on_rsp_sub_market_data: ag2502
ErrorID[0] Message[CTP:No Error]
[2024-12-10T03:05:09Z INFO  channel] on_rsp_sub_market_data: fu2505
[2024-12-10T03:05:09Z INFO  channel] 20241210 11:05:09 ag2502 last_price: 7942
[2024-12-10T03:05:09Z INFO  channel] 20241210 11:05:08 fu2505 last_price: 3000
[2024-12-10T03:05:09Z INFO  channel] 20241210 11:05:09 ag2502 last_price: 7941
[2024-12-10T03:05:09Z INFO  channel] 20241210 11:05:10 ag2502 last_price: 7941
[2024-12-10T03:05:10Z INFO  channel] 20241210 11:05:10 ag2502 last_price: 7941
[2024-12-10T03:05:10Z INFO  channel] 20241210 11:05:11 ag2502 last_price: 7940
[2024-12-10T03:05:11Z INFO  channel] 20241210 11:05:11 fu2505 last_price: 3000
```