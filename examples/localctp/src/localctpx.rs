#![allow(non_upper_case_globals, unused_variables)]
#![allow(dead_code)]
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use ctp2rs::ffi::{AssignFromString, WrapToString};
use ctp2rs::v1alpha1::{bindings::*, TraderApi, TraderSpi};

use log::{debug, error, info};

use tokio::sync::mpsc;
use tokio::time;

pub fn init_logger() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug")
    }
    env_logger::init();
}

/// 自定义交易事件枚举，替代已移除的 TraderSpiEvent
#[derive(Debug)]
pub enum TraderEvent {
    FrontConnected,
    FrontDisconnected(i32),
    RspAuthenticate {
        rsp_info: Option<CThostFtdcRspInfoField>,
    },
    RspUserLogin {
        rsp_user_login: Option<CThostFtdcRspUserLoginField>,
        rsp_info: Option<CThostFtdcRspInfoField>,
    },
    RtnOrder {
        order: Option<CThostFtdcOrderField>,
    },
    RspOrderInsert {
        rsp_info: Option<CThostFtdcRspInfoField>,
    },
    RtnTrade {
        trade: Option<CThostFtdcTradeField>,
    },
    Other(String),
}

/// 通过 channel 转发 SPI 回调的实现
pub struct ChannelTraderSpi {
    tx: std::sync::mpsc::SyncSender<TraderEvent>,
}

impl TraderSpi for ChannelTraderSpi {
    fn on_front_connected(&mut self) {
        let _ = self.tx.send(TraderEvent::FrontConnected);
    }

    fn on_front_disconnected(&mut self, n_reason: i32) {
        let _ = self.tx.send(TraderEvent::FrontDisconnected(n_reason));
    }

    fn on_rsp_authenticate(
        &mut self,
        _p_rsp_authenticate_field: Option<&CThostFtdcRspAuthenticateField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        _n_request_id: i32,
        _b_is_last: bool,
    ) {
        let _ = self.tx.send(TraderEvent::RspAuthenticate {
            rsp_info: p_rsp_info.cloned(),
        });
    }

    fn on_rsp_user_login(
        &mut self,
        p_rsp_user_login: Option<&CThostFtdcRspUserLoginField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        _n_request_id: i32,
        _b_is_last: bool,
    ) {
        let _ = self.tx.send(TraderEvent::RspUserLogin {
            rsp_user_login: p_rsp_user_login.cloned(),
            rsp_info: p_rsp_info.cloned(),
        });
    }

    fn on_rtn_order(&mut self, p_order: Option<&CThostFtdcOrderField>) {
        let _ = self.tx.send(TraderEvent::RtnOrder {
            order: p_order.cloned(),
        });
    }

    fn on_rsp_order_insert(
        &mut self,
        _p_input_order: Option<&CThostFtdcInputOrderField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        _n_request_id: i32,
        _b_is_last: bool,
    ) {
        let _ = self.tx.send(TraderEvent::RspOrderInsert {
            rsp_info: p_rsp_info.cloned(),
        });
    }

    fn on_rtn_trade(&mut self, p_trade: Option<&CThostFtdcTradeField>) {
        let _ = self.tx.send(TraderEvent::RtnTrade {
            trade: p_trade.cloned(),
        });
    }
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

/// 创建 LocalCTP 和 channel 接收端
pub fn create_localctp_and_spi(
    dynlib: &str,
    flow_path: &str,
) -> (Box<LocalCTP>, std::sync::mpsc::Receiver<TraderEvent>) {
    let api = TraderApi::create_api(dynlib, flow_path);

    let (tx, rx) = std::sync::mpsc::sync_channel(1024);
    let spi = ChannelTraderSpi { tx };
    let spi_box = Box::new(spi);
    let spi_ptr = Box::into_raw(spi_box) as *mut dyn TraderSpi;
    api.register_spi(spi_ptr);

    let localctp = LocalCTP {
        tdapi: Arc::new(api),
    };

    (Box::new(localctp), rx)
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
        LocalCTP { tdapi }
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
    let broker_id = &account_config.broker_id;
    let account = &account_config.account;
    let exchange = "SHFE";
    let symbol = "ag2506";
    let price = 50000.0;
    let volume = 1;
    let order_ref = 123;
    let n_request_id = 1;

    let order_input = InputOrderField {
        direction: THOST_FTDC_D_Buy,
        offset: THOST_FTDC_OF_Open,
        price,
        volume,
    };

    debug!("insert_limit_order: {} -> {}", symbol, price);
    let result = api.req_order_insert(
        broker_id,
        account,
        exchange,
        symbol,
        &order_input,
        order_ref,
        n_request_id,
    );

    match result {
        Ok(_) => debug!("Order successfully inserted."),
        Err(e) => error!("Error inserting order: {:?}", e),
    }
}

#[derive(Clone, Debug, Default)]
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
    let mut request_id: i32 = 0;
    let mut get_request_id = || {
        request_id += 1;
        request_id
    };
    #[cfg(target_os = "linux")]
    let dynlib_path = "./lib/libthosttraderapi_se_v6.7.2.so";
    #[cfg(target_os = "macos")]
    let dynlib_path = "./lib/libthosttraderapi_se_v6.7.2.dylib";

    let (localctp, spi_rx) = create_localctp_and_spi(dynlib_path, "");
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

    // 将同步 Receiver 桥接到 tokio 异步任务
    let (async_tx, mut async_rx) = mpsc::unbounded_channel();
    tokio::task::spawn_blocking(move || {
        while let Ok(event) = spi_rx.recv() {
            if async_tx.send(event).is_err() {
                break;
            }
        }
    });

    // 处理登陆初始化查询
    while let Some(spi_msg) = async_rx.recv().await {
        match spi_msg {
            TraderEvent::FrontConnected => {
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
            TraderEvent::RspAuthenticate { .. } => {
                info!("认证成功回报 OnRspAuthenticate");
                let mut req = CThostFtdcReqUserLoginField::default();
                req.BrokerID.assign_from_str(broker_id);
                req.UserID.assign_from_str(account);
                req.Password.assign_from_str(&ctp_account.password);
                localctp.tdapi.req_user_login(&mut req, get_request_id());
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
    let shared_api = Arc::new(localctp);
    let api_clone: Arc<Box<LocalCTP>> = shared_api.clone();
    tokio::spawn(async move {
        simulate_market_data(api_clone).await;
    });

    time::sleep(Duration::from_millis(1000)).await;
    insert_limit_order(shared_api.clone(), ctp_account).await;

    time::sleep(Duration::from_secs(2)).await;

    while let Some(spi_msg) = async_rx.recv().await {
        match spi_msg {
            TraderEvent::RtnOrder { order } => {
                let order = order.unwrap();
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
            TraderEvent::RspOrderInsert { rsp_info } => {
                let rsp_info = rsp_info.unwrap();
                info!(
                    "报单失败回报 OnRspOrderInsert {}: {}",
                    rsp_info.ErrorID,
                    rsp_info.ErrorMsg.to_string(),
                );
                break;
            }
            TraderEvent::RtnTrade { trade } => {
                let trade = trade.unwrap();

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
                break;
            }
            _ => {
                info!("其它回报");
            }
        }
    }
}
