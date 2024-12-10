mod localctpx;
use localctpx::*;
use log::info;

#[tokio::main]
async fn main() {
    init_logger();
    info!("Start!");

    // let trade_front = "tcp://180.168.146.187:10130"; // 7*24

    let account = CtpAccountConfig {
        broker_id: "9999".to_string(),
        account: "111111".to_string(),
        trade_fronts: vec!["tcp://180.168.146.187:10201".to_string()],
        md_fronts: vec!["180.168.146.187:10211".to_string()],
        name_servers: vec!["".to_string()],
        auth_code: "0000000000000000".to_string(),
        user_product_info: "".to_string(),
        app_id: "simnow_client_test".to_string(),
        password: "just test".to_string(),
        remark: "".into(),
    };
    query(&account).await;
}
