use ctp2rs::v1alpha1::{THOST_FTDC_D_Buy, THOST_FTDC_OF_Open};
use localctp::localctp2::{CtpAccountConfig, MyMdSpi, MyTraderSpi};
use log::info;

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let cfg = CtpAccountConfig {
        broker_id: "9999".to_string(),
        account: std::env::var("SIMNOW_USER_ID").unwrap_or_else(|_| "027011".to_string()),
        auth_code: "0000000000000000".to_string(),
        user_product_info: "ctp_test".to_string(),
        app_id: "ctp_test".to_string(),
        password: "123456".to_string(),
        remark: "".to_string(),
    };
    let trader_api = MyTraderSpi::new(cfg);
    trader_api.connect();

    let md_spi = MyMdSpi::new();
    md_spi.connect();

    std::thread::sleep(std::time::Duration::from_secs(5));
    let result = trader_api.insert_limit_order(
        "ag2512",
        "SHFE",
        50000.0,
        1,
        THOST_FTDC_D_Buy,
        THOST_FTDC_OF_Open,
    );
    match result {
        Ok(()) => info!("Inserted limit order successfully."),
        Err(e) => info!("Failed to insert limit order: {}", e),
    }
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
