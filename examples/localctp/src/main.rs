mod localctpx;
use localctpx::*;
use log::info;

#[tokio::main]
async fn main() {
    init_logger();
    info!("Start!");

    // let trade_front = "tcp://182.254.243.31:40001"; // 7*24

    let account = CtpAccountConfig {
        broker_id: "9999".to_string(),
        account: "111111".to_string(),
        trade_fronts: vec!["tcp://182.254.243.31:30001".to_string()],
        md_fronts: vec!["182.254.243.31:30011".to_string()],
        name_servers: vec!["".to_string()],
        auth_code: "0000000000000000".to_string(),
        user_product_info: "".to_string(),
        app_id: "simnow_client_test".to_string(),
        password: "just test".to_string(),
        remark: "".into(),
    };
    query(&account).await;
}
