#![allow(non_upper_case_globals, unused_variables)]
#![allow(dead_code)]
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use futures::StreamExt;

use ctp2rs::ffi::{AssignFromString, WrapToString};
use ctp2rs::v1alpha1::{bindings::*, TraderApi, TraderSpiInner, TraderSpiStream};

use ctp2rs::v1alpha1::event::TraderSpiEvent::*;

use log::{debug, error, info};

use tokio::time;

pub fn init_logger() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug")
    }
    env_logger::init();
}

#[derive(Clone, Debug)]
pub struct CtpAccountConfig {
    pub broker_id: String,
    pub account: String,
    pub trade_fronts: Vec<String>,
    pub md_fronts: Vec<String>,
    pub name_servers: Vec<String>,
    pub auth_code: String,
    pub user_product_info: String,
    pub app_id: String,
    pub password: String,
    pub remark: String,
}

#[derive(Debug, Clone)]
pub struct FakeMarketQuote {
    pub instrument_id: String,
    pub bid_price: f64,
    pub ask_price: f64,
    pub quote_ref: String,
    pub last_price: String,
    pub settlement_price: String,
    pub upper_limit_price: String,
    pub lower_limit_price: String,
    pub business_unit: String,
    pub volume: i32,
}

async fn simulate_market_data(api: Arc<Box<LocalCTP>>) {
    let mut interval = time::interval(Duration::from_millis(500));
    // 使用全局 random / random_range 函数（避免线程局部 RNG Send 问题）
    loop {
        interval.tick().await;
        let market_quote = FakeMarketQuote {
            instrument_id: "ag2506".to_string(),
            bid_price: rand::random_range(1000.0..2000.0),
            ask_price: rand::random_range(1000.0..2000.0),
            quote_ref: format!("{:.1}", rand::random::<f64>() * 1000.0),
            last_price: format!("{:.2}", rand::random::<f64>() * 5000.0),
            settlement_price: format!("{:.2}", rand::random::<f64>() * 5000.0),
            upper_limit_price: format!("{:.2}", rand::random::<f64>() * 6000.0),
            lower_limit_price: format!("{:.2}", rand::random::<f64>() * 4000.0),
            business_unit: format!("{:08}", rand::random::<u32>()),
            volume: rand::random_range(1..100),
        };
        debug!(
            "{} -> {}",
            market_quote.instrument_id, market_quote.last_price
        );
        if let Err(e) = api.insert_market_quote(&market_quote) {
            eprintln!("Error inserting market quote: {:?}", e);
        }
    }
}

pub fn create_localctp_and_spi(
    dynlib: &str,
    flow_path: &str,
) -> (Box<LocalCTP>, Box<TraderSpiStream>) {
    let api = TraderApi::create_api(dynlib, flow_path);

    // Initialize the SPI and get the stream
    let stream = {
        let (stream, pp) = create_spi();
        api.register_spi(pp);
        stream
    };
    let localctp = LocalCTP {
        tdapi: Arc::new(api),
    };

    (Box::new(localctp), stream)
}

pub fn create_spi() -> (Box<TraderSpiStream>, *mut TraderSpiStream) {
    let inner = TraderSpiInner::new();
    let xspi = TraderSpiStream::new(inner);

    let myspi = Box::new(xspi);
    let pp = Box::into_raw(myspi);
    let pp2 = pp.clone();
    (unsafe { Box::from_raw(pp2) }, pp)
}

pub struct InputOrderField {
    pub direction: u8,
    pub offset: u8,
    pub price: f64,
    pub volume: i32,
}

pub struct LocalCTP {
    pub tdapi: Arc<TraderApi>,
}

impl LocalCTP {
    pub fn create<P: AsRef<Path>>(dynlib: P, flow_path: P) -> Self {
        let tdapi = TraderApi::create_api(dynlib, flow_path);
        let tdapi = Arc::new(tdapi);
        let localctp = LocalCTP { tdapi: tdapi };
        localctp
    }

    pub fn insert_market_quote(&self, fq: &FakeMarketQuote) -> Result<(), std::ffi::NulError> {
        let mut quote = CThostFtdcInputQuoteField::default();
        quote
            .InstrumentID
            .assign_from_str(fq.instrument_id.as_str());
        quote.BidPrice = fq.bid_price;
        quote.AskPrice = fq.ask_price;
        quote.QuoteRef.assign_from_str(&fq.quote_ref);
        quote.UserID.assign_from_str(&fq.last_price);
        quote.ForQuoteSysID.assign_from_str(&fq.settlement_price);
        quote.BidOrderRef.assign_from_str(&fq.upper_limit_price);
        quote.AskOrderRef.assign_from_str(&fq.lower_limit_price);
        quote.BusinessUnit.assign_from_str(&fq.business_unit);

        self.tdapi.req_quote_insert(&mut quote, fq.volume);

        Ok(())
    }

    fn req_order_insert(
        &self,
        broker_id: &str,
        account: &str,
        exchange: &str,
        symbol: &str,
        order: &InputOrderField,
        order_ref: i32,
        n_request_id: i32,
    ) -> Result<(), String> {
        let mut input = CThostFtdcInputOrderField::default();

        input.BrokerID.assign_from_str(broker_id);
        input.UserID.assign_from_str(account);
        input.InvestorID.assign_from_str(account);
        input.ExchangeID.assign_from_str(exchange);
        input.InstrumentID.assign_from_str(symbol);
        input.Direction = order.direction as i8;
        input.CombOffsetFlag[0] = order.offset as i8;
        input.CombHedgeFlag[0] = THOST_FTDC_HF_Speculation as i8;
        input.LimitPrice = order.price;
        input.VolumeTotalOriginal = order.volume;
        input.OrderPriceType = THOST_FTDC_OPT_LimitPrice as i8;
        if input.LimitPrice == 0.0 {
            input.OrderPriceType = THOST_FTDC_OPT_AnyPrice as i8;
        }
        input.TimeCondition = THOST_FTDC_TC_GFD as i8;
        input.VolumeCondition = THOST_FTDC_VC_AV as i8;
        input.MinVolume = 1;
        input.ContingentCondition = THOST_FTDC_CC_Immediately as i8;
        input.ForceCloseReason = THOST_FTDC_FCC_NotForceClose as i8;
        input.IsAutoSuspend = 0;
        input.UserForceClose = 0;
        input.OrderRef.assign_from_str(&format!("{order_ref}"));
        let ret = self.tdapi.req_order_insert(&mut input, n_request_id);
        if ret != 0 {
            error!("td.order_insert {}", ret);
            return Err(format!("req_order_insert error: {}", ret));
        }
        Ok(())
    }
}

async fn insert_limit_order(api: Arc<Box<LocalCTP>>, account_config: &CtpAccountConfig) {
    // Define the necessary parameters for a limit order
    let broker_id = &account_config.broker_id;
    let account = &account_config.account;
    let exchange = "SHFE"; // Shanghai Futures Exchange, adjust as needed
    let symbol = "ag2506"; // Example futures contract for copper, adjust as needed
    let price = 50000.0; // Example price, adjust as needed
    let volume = 1; // Example volume, adjust as needed
    let order_ref = 123; // This should be a unique reference for the order, possibly incrementing
    let n_request_id = 1; // Request ID, unique per request

    // Initialize the order input with default values
    let order_input = InputOrderField {
        direction: THOST_FTDC_D_Buy, // Buying, adjust as needed (Long or Short)
        offset: THOST_FTDC_OF_Open,  // Order is to open a position, adjust as needed
        price,
        volume,
    };

    debug!("insert_limit_order: {} -> {}", symbol, price);
    let tdapi = api;
    // Call the req_order_insert method on the API object
    let result = tdapi.req_order_insert(
        broker_id,
        account,
        exchange,
        symbol,
        &order_input,
        order_ref,
        n_request_id,
    );

    // Check the result of the order insertion
    match result {
        Ok(_) => debug!("Order successfully inserted."),
        Err(e) => error!("Error inserting order: {:?}", e),
    }
}

#[derive(Clone, Debug, Default /* , Encode, Decode */)]
pub struct CtpQueryResult {
    broker_id: String,
    account: String,
    trading_day: i32,
    timestamp: i64,
    dmd_list: Vec<CThostFtdcDepthMarketDataField>,
    icr_list: Vec<CThostFtdcInstrumentCommissionRateField>,
    instrument_list: Vec<CThostFtdcInstrumentField>,
    position_list: Vec<CThostFtdcInvestorPositionField>,
    position_detail_list: Vec<CThostFtdcInvestorPositionDetailField>,
    trading_account: CThostFtdcTradingAccountField,
    product_list: Vec<CThostFtdcProductField>,
    order_list: Vec<CThostFtdcOrderField>,
    trade_list: Vec<CThostFtdcTradeField>,
}

pub async fn query(ctp_account: &CtpAccountConfig) {
    let broker_id = ctp_account.broker_id.as_str();
    let account = ctp_account.account.as_str();
    let trade_front = ctp_account.trade_fronts[0].as_str();
    let name_server = ctp_account.name_servers[0].as_str();
    let auth_code = ctp_account.auth_code.as_str();
    let user_product_info = ctp_account.user_product_info.as_str();
    let app_id = ctp_account.app_id.as_str();
    // let password = ca.password.as_str();
    let mut request_id: i32 = 0;
    let mut get_request_id = || {
        request_id += 1;
        request_id
    };
    #[cfg(target_os = "linux")]
    let dynlib_path = "./lib/libthosttraderapi_se_v6.7.2.so";
    #[cfg(target_os = "macos")]
    let dynlib_path = "./lib/libthosttraderapi_se_v6.7.2.dylib";
    // let dynlib_path = "./lib/libthosttraderapi_se.dylib";

    let (localctp, mut spi_stream) = create_localctp_and_spi(dynlib_path, "");
    // let mut localctp = LocalCTP::create(dynlib_path, "./td_con_");
    println!("LocalCTP version: {}", localctp.tdapi.get_api_version());
    debug!("register name_server {:#?}", name_server);
    localctp.tdapi.register_front(trade_front);
    info!("register front {}", trade_front);
    debug!("register done");
    localctp
        .tdapi
        .subscribe_public_topic(THOST_TE_RESUME_TYPE::THOST_TERT_QUICK);
    localctp
        .tdapi
        .subscribe_private_topic(THOST_TE_RESUME_TYPE::THOST_TERT_QUICK);
    debug!("subscribe topic done");
    localctp.tdapi.init();
    debug!("init done");
    let mut result = CtpQueryResult::default();
    result.broker_id = broker_id.to_string();
    result.account = account.to_string();
    // 处理登陆初始化查询
    // 登录后才能发单

    // let mut login_req = CThostFtdcReqUserLoginField::default();
    // set_cstr_from_str_truncate_i8(&mut login_req.BrokerID, &ctp_account.broker_id);
    // set_cstr_from_str_truncate_i8(&mut login_req.UserID, &ctp_account.account);
    // set_cstr_from_str_truncate_i8(&mut login_req.Password, &ctp_account.password);
    // api_box.req_user_login(&mut login_req, 1);

    while let Some(spi_msg) = spi_stream.next().await {
        match spi_msg {
            OnFrontConnected(p) => {
                info!("前端连接成功回报 OnFrontConnected");
                let mut req = CThostFtdcReqAuthenticateField::default();
                req.BrokerID.assign_from_str(broker_id);
                req.UserID.assign_from_str(account);
                req.AuthCode.assign_from_str(auth_code);
                req.UserProductInfo.assign_from_str(user_product_info);
                req.AppID.assign_from_str(app_id);
                localctp.tdapi.req_authenticate(&mut req, get_request_id());
                info!("call req_authenticate done");
            }
            OnRspAuthenticate(p) => {
                info!("认证成功回报 OnRspAuthenticate");
                // 认证后才能登录
                let mut req = CThostFtdcReqUserLoginField::default();
                req.BrokerID.assign_from_str(broker_id);
                req.UserID.assign_from_str(account);
                req.Password.assign_from_str(&ctp_account.password);
                // 登录后才能下单
                localctp.tdapi.req_user_login(&mut req, get_request_id());
                // 这里有个 break，之后这个 while match 不再接收信息。（推荐将 SPI 放到单独线程）
                break;
            }
            _ => {
                info!("其它回报");
            }
        }
    }
    info!("完成认证");
    result.timestamp = chrono::Local::now().timestamp();
    info!("开始输入行情");
    // Wrap the API in Arc and Mutex for shared ownership and thread safety
    let shared_api = Arc::new(localctp);
    // Clone the API handle for the spawned task
    let api_clone: Arc<Box<LocalCTP>> = shared_api.clone();
    tokio::spawn(async move {
        simulate_market_data(api_clone).await;
    });

    // Now instead of trying to use `api_box`, use `shared_api`
    // If you need a mutable reference from the original `shared_api`:

    time::sleep(Duration::from_millis(1000)).await;
    insert_limit_order(shared_api.clone(), ctp_account).await;

    // Wait for 2 seconds after inserting the limit order
    time::sleep(Duration::from_secs(2)).await;

    while let Some(spi_msg) = spi_stream.next().await {
        match spi_msg {
            OnRtnOrder(p) => {
                let order = p.order.unwrap();
                let broker_id = order.BrokerID.to_string();
                let investor_id = order.InvestorID.to_string();
                let exchange_id = order.ExchangeID.to_string();
                let order_ref = order.OrderRef.to_string();
                let instrument_id = order.InstrumentID.to_string();

                let order_status = match order.OrderStatus as u8 {
                    THOST_FTDC_OST_AllTraded => "全部成交",
                    THOST_FTDC_OST_PartTradedQueueing => "部分成交还在队列中",
                    THOST_FTDC_OST_PartTradedNotQueueing => "部分成交不在队列中",
                    THOST_FTDC_OST_NoTradeQueueing => "未成交还在队列中",
                    THOST_FTDC_OST_NoTradeNotQueueing => "未成交不在队列中",
                    THOST_FTDC_OST_Canceled => "已撤销",
                    THOST_FTDC_OST_Unknown => "未知状态",
                    THOST_FTDC_OST_NotTouched => "尚未触发",
                    THOST_FTDC_OST_Touched => "已触发",
                    _ => "其他状态",
                };

                info!("报单成功回报 Order Return: BrokerID: {}, InvestorID: {}, ExchangeID: {}, OrderRef: {}, OrderStatus: {}, InstrumentID: {}", 
                          broker_id, investor_id, exchange_id, order_ref, order_status, instrument_id);
            }
            OnRspOrderInsert(p) => {
                let rsp_info = p.rsp_info.unwrap();
                info!(
                    "报单失败回报 OnRspOrderInsert {}: {}",
                    rsp_info.ErrorID,
                    rsp_info.ErrorMsg.to_string(),
                );

                break;
            }
            OnRtnTrade(p) => {
                let trade = p.trade.unwrap();

                let broker_id = trade.BrokerID.to_string();
                let investor_id = trade.InvestorID.to_string();
                let exchange_id = trade.ExchangeID.to_string();
                let trade_id = trade.TradeID.to_string();
                let order_ref = trade.OrderRef.to_string();
                let instrument_id = trade.InstrumentID.to_string();
                let price = trade.Price;
                let volume = trade.Volume;

                info!("成交回报 OnRtnTrade: OrderRef: {}, BrokerID: {}, InvestorID: {}, ExchangeID: {}, TradeID: {}, InstrumentID: {}, Price: {:.2}, Volume: {}",
                          order_ref, broker_id, investor_id, exchange_id, trade_id, instrument_id, price, volume);

                // 这里有个 break，之后这个 while match 不再接收信息。（推荐将 SPI 放到单独线程）
                break;
            }

            _ => {
                info!("其它回报");
            }
        }
    }
}
