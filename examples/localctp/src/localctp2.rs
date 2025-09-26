use std::{
    sync::{atomic::AtomicBool, Arc},
    vec,
};

use crate::{as_bytes_unsafe, MarketPublisher};
use ctp2rs::{
    ffi::{SetString, WrapToString},
    v1alpha1::{
        CThostFtdcDepthMarketDataField, CThostFtdcInputOrderField, CThostFtdcReqAuthenticateField,
        CThostFtdcReqUserLoginField, CThostFtdcRspInfoField, CThostFtdcRspUserLoginField,
        CThostFtdcSpecificInstrumentField, MdApi, MdSpi, THOST_FTDC_CC_Immediately,
        THOST_FTDC_FCC_NotForceClose, THOST_FTDC_HF_Speculation, THOST_FTDC_OPT_AnyPrice,
        THOST_FTDC_OPT_LimitPrice, THOST_FTDC_OST_AllTraded, THOST_FTDC_OST_Canceled,
        THOST_FTDC_OST_NoTradeNotQueueing, THOST_FTDC_OST_NoTradeQueueing,
        THOST_FTDC_OST_NotTouched, THOST_FTDC_OST_PartTradedQueueing, THOST_FTDC_OST_Touched,
        THOST_FTDC_OST_Unknown, TraderApi, TraderSpi, THOST_FTDC_TC_GFD, THOST_FTDC_VC_AV,
    },
};
use log::{debug, error, info, warn};

#[derive(Clone, Debug)]
pub struct CtpAccountConfig {
    pub broker_id: String,
    pub account: String,
    pub auth_code: String,
    pub user_product_info: String,
    pub app_id: String,
    pub password: String,
    pub remark: String,
}

pub struct MyTraderSpi {
    tdapi: Option<TraderApi>,
    cfg: CtpAccountConfig,
}

impl MyTraderSpi {
    pub fn new(cfg: CtpAccountConfig) -> Arc<Self> {
        #[cfg(target_os = "macos")]
        let dynlib_path = "./lib/libthosttraderapi_se_v6.7.2.dylib";
        #[cfg(target_os = "linux")]
        let dynlib_path = "./lib/libthosttraderapi_se_v6.7.2.so";

        let tdapi = TraderApi::create_api(dynlib_path, "");
        let mut arc = Arc::new(Self {
            tdapi: Some(tdapi),
            cfg,
        });
        {
            let spi: &mut MyTraderSpi =
                Arc::get_mut(&mut arc).expect("Failed to get mutable reference to Arc");
            let spi_ptr: *mut dyn TraderSpi = spi as *mut dyn TraderSpi;
            if let Some(ref inner) = spi.tdapi {
                inner.register_spi(spi_ptr);
            }
        }
        arc
    }

    pub fn connect(&self) {
        if let Some(ref api) = self.tdapi {
            api.init();
            info!("Trader API initialized and connecting to front.");
        } else {
            error!("Trader API is not initialized.");
        }
    }

    pub fn req_confirm_settlement(&self) {}

    pub fn insert_limit_order(
        &self,
        symbol: &str,
        exchange: &str,
        price: f64,
        volume: i32,
        direction: u8,
        offset: u8,
    ) -> Result<(), String> {
        let broker_id = "9999";
        let account = self.cfg.account.as_str();
        let order_ref = 123; // This should be a unique reference for the order, possibly incrementing
        let n_request_id = 1; // Request ID, unique per request

        debug!("insert_limit_order: {} -> {}", symbol, price);

        let mut input = CThostFtdcInputOrderField::default();

        input.BrokerID.set_str(broker_id);
        input.UserID.set_str(account);
        input.InvestorID.set_str(account);
        input.ExchangeID.set_str(exchange);
        input.InstrumentID.set_str(symbol);
        input.Direction = direction as i8;
        input.CombOffsetFlag[0] = offset as i8;
        input.CombHedgeFlag[0] = THOST_FTDC_HF_Speculation as i8;
        input.LimitPrice = price;
        input.VolumeTotalOriginal = volume;
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
        input.OrderRef.set_str(&format!("{order_ref}"));
        let ret = self
            .tdapi
            .as_ref()
            .unwrap()
            .req_order_insert(&mut input, n_request_id);
        if ret != 0 {
            error!("td.order_insert {}", ret);
            return Err(format!("req_order_insert error: {}", ret));
        }
        debug!("Order successfully inserted.");
        Ok(())
    }
}

#[allow(unused_variables)]
impl TraderSpi for MyTraderSpi {
    fn on_front_connected(&mut self) {
        info!("Trader front connected.");
        let user_id = std::env::var("SIMNOW_USER_ID").unwrap_or_else(|_| "027011".to_string());
        let mut req = CThostFtdcReqAuthenticateField::default();
        req.BrokerID.set_str("9999");
        req.UserID.set_str(&user_id);
        req.AuthCode.set_str("0000000000000000");
        req.AppID.set_str("ctp_test");
        self.tdapi.as_ref().unwrap().req_authenticate(&mut req, 0);
    }

    fn on_front_disconnected(&mut self, n_reason: i32) {
        error!("Trader front disconnected with reason code: {}", n_reason);
    }

    fn on_heart_beat_warning(&mut self, n_time_lapse: i32) {
        warn!("Trader Heartbeat warning, time lapse: {}", n_time_lapse);
    }

    fn on_rsp_authenticate(
        &mut self,
        _p_rsp_authenticate_field: Option<&ctp2rs::v1alpha1::CThostFtdcRspAuthenticateField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        _n_request_id: i32,
        _b_is_last: bool,
    ) {
        if let Some(rsp_info) = p_rsp_info {
            if rsp_info.ErrorID != 0 {
                error!(
                    "Authenticate failed: code={}, msg={}",
                    rsp_info.ErrorID,
                    rsp_info.ErrorMsg.to_string()
                );
                return;
            }
        }
        info!("Authenticate successful.");
        let user_id = std::env::var("SIMNOW_USER_ID").unwrap_or_else(|_| "027011".to_string());
        let mut req = CThostFtdcReqUserLoginField::default();
        req.BrokerID.set_str("9999");
        req.UserID.set_str(&user_id);
        req.Password.set_str("123456");
        self.tdapi.as_ref().unwrap().req_user_login(&mut req, 1);
    }

    fn on_rsp_user_login(
        &mut self,
        p_rsp_user_login: Option<&CThostFtdcRspUserLoginField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        _n_request_id: i32,
        _b_is_last: bool,
    ) {
        if let Some(rsp_info) = p_rsp_info {
            if rsp_info.ErrorID != 0 {
                error!(
                    "Login failed: code={}, msg={}",
                    rsp_info.ErrorID,
                    rsp_info.ErrorMsg.to_string()
                );
                return;
            }
        }
        if let Some(rsp_user_login) = p_rsp_user_login {
            info!(
                "Login successful: SysVersion={}, SystemName={}",
                rsp_user_login.SysVersion.to_string(),
                rsp_user_login.SystemName.to_string(),
            );
        }
        self.req_confirm_settlement();
    }

    fn on_rsp_settlement_info_confirm(
        &mut self,
        p_settlement_info_confirm: Option<&ctp2rs::v1alpha1::CThostFtdcSettlementInfoConfirmField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        n_request_id: i32,
        b_is_last: bool,
    ) {
    }

    fn on_rsp_qry_trade(
        &mut self,
        p_trade: Option<&ctp2rs::v1alpha1::CThostFtdcTradeField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        n_request_id: i32,
        b_is_last: bool,
    ) {
    }

    fn on_rsp_qry_order(
        &mut self,
        p_order: Option<&ctp2rs::v1alpha1::CThostFtdcOrderField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        n_request_id: i32,
        b_is_last: bool,
    ) {
    }

    fn on_rsp_order_insert(
        &mut self,
        p_input_order: Option<&ctp2rs::v1alpha1::CThostFtdcInputOrderField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        n_request_id: i32,
        b_is_last: bool,
    ) {
        if let Some(rsp_info) = p_rsp_info {
            if rsp_info.ErrorID != 0 {
                error!(
                    "Order Insert failed: code={}, msg={}",
                    rsp_info.ErrorID,
                    rsp_info.ErrorMsg.to_string()
                );
                return;
            }
        }
        if let Some(input_order) = p_input_order {
            info!(
                "Order Insert successful: InstrumentID={}, LimitPrice={}, VolumeTotalOriginal={}",
                input_order.InstrumentID.to_string(),
                input_order.LimitPrice,
                input_order.VolumeTotalOriginal
            );
        }
    }

    #[allow(non_snake_case, non_upper_case_globals, unreachable_patterns)]
    fn on_rtn_order(&mut self, p_order: Option<&ctp2rs::v1alpha1::CThostFtdcOrderField>) {
        let order = p_order.unwrap();
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

    fn on_rtn_trade(&mut self, p_trade: Option<&ctp2rs::v1alpha1::CThostFtdcTradeField>) {
        let trade = p_trade.unwrap();

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
    }

    fn on_rsp_qry_investor_position(
        &mut self,
        p_investor_position: Option<&ctp2rs::v1alpha1::CThostFtdcInvestorPositionField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        n_request_id: i32,
        b_is_last: bool,
    ) {
    }

    fn on_rsp_error(
        &mut self,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        n_request_id: i32,
        b_is_last: bool,
    ) {
        if let Some(rsp_info) = p_rsp_info {
            error!(
                "Trader Error: code={}, msg={}",
                rsp_info.ErrorID,
                rsp_info.ErrorMsg.to_string()
            );
        }
    }
}

pub struct MyMdSpi {
    #[allow(dead_code)]
    mdapi: Option<MdApi>,
    #[allow(dead_code)]
    running: AtomicBool,
    #[allow(dead_code)]
    mq: MarketPublisher,
}

impl MyMdSpi {
    pub fn new() -> Arc<Self> {
        #[cfg(target_os = "macos")]
        let dynlib_path = "../../ctp-dyn/api/ctp/v6.7.2/v6.7.2_MacOS_20231016/thostmduserapi_se.framework/thostmduserapi_se";
        #[cfg(target_os = "linux")]
        let dynlib_path = "../../ctp-dyn/api/ctp/v6.7.2/v6.7.2_20230913_api_traderapi_se_linux64/thostmduserapi_se.so";

        let mdapi = MdApi::create_api(dynlib_path, "./md_", false, false);
        let mq = MarketPublisher::create("md_queue", 1024, 1024 * 1024)
            .expect("failed to create market publisher");
        let mut arc = Arc::new(Self {
            mdapi: Some(mdapi),
            running: AtomicBool::new(true),
            mq,
        });
        {
            let spi: &mut MyMdSpi =
                Arc::get_mut(&mut arc).expect("Failed to get mutable reference to Arc");
            let spi_ptr: *mut dyn MdSpi = spi as *mut dyn MdSpi;
            if let Some(ref inner) = spi.mdapi {
                inner.register_spi(spi_ptr);
                // inner.register_front("tcp://182.254.243.31:40011");
                inner.register_front("tcp://182.254.243.31:40011");
            }
        }
        arc
    }

    pub fn connect(&self) {
        if let Some(ref api) = self.mdapi {
            api.init();
            info!("MD API initialized and connecting to front.");
        } else {
            error!("MD API is not initialized.");
        }
    }

    pub fn subscribe_market_data(&self, instruments: &[String]) {
        self.mdapi
            .as_ref()
            .unwrap()
            .subscribe_market_data(instruments);
    }
}

impl MdSpi for MyMdSpi {
    fn on_front_connected(&mut self) {
        let user_id = std::env::var("SIMNOW_USER_ID").unwrap_or_else(|_| "027011".to_string());
        let mut req = CThostFtdcReqUserLoginField::default();
        req.BrokerID.set_str("9999");
        req.UserID.set_str(&user_id);

        let ret = self.mdapi.as_ref().unwrap().req_user_login(&mut req, 1);
        if ret != 0 {
            error!("Failed to send login request, error code: {}", ret);
        }
    }

    fn on_front_disconnected(&mut self, n_reason: i32) {
        error!("Disconnected from front with reason code: {}", n_reason);
    }

    fn on_heart_beat_warning(&mut self, n_time_lapse: i32) {
        warn!("Heartbeat warning, time lapse: {}", n_time_lapse);
    }

    #[doc = " 登录请求响应"]
    fn on_rsp_user_login(
        &mut self,
        p_rsp_user_login: Option<&CThostFtdcRspUserLoginField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        _n_request_id: i32,
        _b_is_last: bool,
    ) {
        if let Some(rsp_info) = p_rsp_info {
            if rsp_info.ErrorID != 0 {
                error!(
                    "Login failed: code={}, msg={}",
                    rsp_info.ErrorID,
                    rsp_info.ErrorMsg.to_string()
                );
                return;
            }
        }
        if let Some(rsp_user_login) = p_rsp_user_login {
            info!(
                "Login successful: SysVersion={}, SystemName={}",
                rsp_user_login.SysVersion.to_string(),
                rsp_user_login.SystemName.to_string(),
            );
        }
        let instruments = vec!["au2512".to_string(), "ag2512".to_string()];

        self.subscribe_market_data(&instruments);
    }

    #[doc = " 错误应答"]
    fn on_rsp_error(
        &mut self,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        _n_request_id: i32,
        _b_is_last: bool,
    ) {
        if let Some(rsp_info) = p_rsp_info {
            error!(
                "Error: code={}, msg={}",
                rsp_info.ErrorID,
                rsp_info.ErrorMsg.to_string()
            );
        }
    }

    #[doc = " 订阅行情应答"]
    fn on_rsp_sub_market_data(
        &mut self,
        p_specific_instrument: Option<&CThostFtdcSpecificInstrumentField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        _n_request_id: i32,
        _b_is_last: bool,
    ) {
        if let Some(rsp_info) = p_rsp_info {
            if rsp_info.ErrorID != 0 {
                error!(
                    "Subscribe Market Data Response: code={}, msg={}",
                    rsp_info.ErrorID,
                    rsp_info.ErrorMsg.to_string()
                );
            }
        }
        info!(
            "Subscribed to market data for instrument: {:?}",
            p_specific_instrument
                .map(|inst| inst.InstrumentID.to_string())
                .unwrap_or_else(|| "Unknown".to_string())
        );
    }

    #[doc = " 深度行情通知"]
    fn on_rtn_depth_market_data(
        &mut self,
        p_depth_market_data: Option<&CThostFtdcDepthMarketDataField>,
    ) {
        if let Some(depth) = p_depth_market_data {
            info!(
                "Market Data - Instrument: {}, LastPrice: {}, Volume: {}",
                depth.InstrumentID.to_string(),
                depth.LastPrice,
                depth.Volume
            );
            let depth_bytes = as_bytes_unsafe(depth);
            if let Err(e) = self.mq.publish(depth_bytes) {
                error!("publish failed: {}", e);
            }
        }
    }
}
