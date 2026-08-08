#![allow(unused_variables, dead_code)]
//! CTP Trader 专用示例（不含行情模块）
//!
//! 演示交易端完整链条：连接 → 客户端认证 → 登录 → 结算单确认
//! → 查询资金账户 → 查询持仓 → 查询报单 → 查询成交，全部完成后自动退出。
use std::{
    env::var,
    path::Path,
    sync::{
        mpsc::{self, SyncSender},
        Arc,
    },
    thread,
    time::Duration,
};

use ctp2rs::ffi::{gb18030_cstr_i8_to_str, SetString, WrapToString};
use ctp2rs::{print_rsp_info, v1alpha1::*};
use log::*;

/// 交易端回调事件，由 SPI 线程发送到主线程驱动查询链条
#[derive(Debug)]
enum TdEvent {
    FrontConnected,
    FrontDisconnected(i32),
    RspAuthenticate {
        rsp_info: Option<CThostFtdcRspInfoField>,
        is_last: bool,
    },
    RspUserLogin {
        rsp_user_login: Option<CThostFtdcRspUserLoginField>,
        rsp_info: Option<CThostFtdcRspInfoField>,
        is_last: bool,
    },
    RspSettlementInfoConfirm {
        rsp_info: Option<CThostFtdcRspInfoField>,
        is_last: bool,
    },
    RspQryTradingAccount {
        account: Option<CThostFtdcTradingAccountField>,
        rsp_info: Option<CThostFtdcRspInfoField>,
        is_last: bool,
    },
    RspQryInvestorPosition {
        position: Option<CThostFtdcInvestorPositionField>,
        rsp_info: Option<CThostFtdcRspInfoField>,
        is_last: bool,
    },
    RspQryOrder {
        order: Option<CThostFtdcOrderField>,
        rsp_info: Option<CThostFtdcRspInfoField>,
        is_last: bool,
    },
    RspQryTrade {
        trade: Option<CThostFtdcTradeField>,
        rsp_info: Option<CThostFtdcRspInfoField>,
        is_last: bool,
    },
}

struct ChannelTraderSpi {
    tx: SyncSender<TdEvent>,
}

impl TraderSpi for ChannelTraderSpi {
    fn on_front_connected(&mut self) {
        debug!("on_front_connected");
        self.tx.send(TdEvent::FrontConnected).unwrap();
    }

    fn on_front_disconnected(&mut self, n_reason: i32) {
        debug!("on_front_disconnected: {n_reason}");
        self.tx.send(TdEvent::FrontDisconnected(n_reason)).unwrap();
    }

    fn on_rsp_error(
        &mut self,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        n_request_id: i32,
        b_is_last: bool,
    ) {
        error!("on_rsp_error");
        print_rsp_info!(p_rsp_info);
    }

    fn on_rsp_authenticate(
        &mut self,
        p_rsp_authenticate_field: Option<&CThostFtdcRspAuthenticateField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        n_request_id: i32,
        b_is_last: bool,
    ) {
        debug!("on_rsp_authenticate");
        self.tx
            .send(TdEvent::RspAuthenticate {
                rsp_info: p_rsp_info.cloned(),
                is_last: b_is_last,
            })
            .unwrap();
    }

    fn on_rsp_user_login(
        &mut self,
        p_rsp_user_login: Option<&CThostFtdcRspUserLoginField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        n_request_id: i32,
        b_is_last: bool,
    ) {
        debug!("on_rsp_user_login");
        self.tx
            .send(TdEvent::RspUserLogin {
                rsp_user_login: p_rsp_user_login.cloned(),
                rsp_info: p_rsp_info.cloned(),
                is_last: b_is_last,
            })
            .unwrap();
    }

    fn on_rsp_settlement_info_confirm(
        &mut self,
        p_settlement_info_confirm: Option<&CThostFtdcSettlementInfoConfirmField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        n_request_id: i32,
        b_is_last: bool,
    ) {
        debug!("on_rsp_settlement_info_confirm");
        self.tx
            .send(TdEvent::RspSettlementInfoConfirm {
                rsp_info: p_rsp_info.cloned(),
                is_last: b_is_last,
            })
            .unwrap();
    }

    fn on_rsp_qry_trading_account(
        &mut self,
        p_trading_account: Option<&CThostFtdcTradingAccountField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        n_request_id: i32,
        b_is_last: bool,
    ) {
        debug!("on_rsp_qry_trading_account");
        self.tx
            .send(TdEvent::RspQryTradingAccount {
                account: p_trading_account.cloned(),
                rsp_info: p_rsp_info.cloned(),
                is_last: b_is_last,
            })
            .unwrap();
    }

    fn on_rsp_qry_investor_position(
        &mut self,
        p_investor_position: Option<&CThostFtdcInvestorPositionField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        n_request_id: i32,
        b_is_last: bool,
    ) {
        debug!("on_rsp_qry_investor_position");
        self.tx
            .send(TdEvent::RspQryInvestorPosition {
                position: p_investor_position.cloned(),
                rsp_info: p_rsp_info.cloned(),
                is_last: b_is_last,
            })
            .unwrap();
    }

    fn on_rsp_qry_order(
        &mut self,
        p_order: Option<&CThostFtdcOrderField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        n_request_id: i32,
        b_is_last: bool,
    ) {
        debug!("on_rsp_qry_order");
        self.tx
            .send(TdEvent::RspQryOrder {
                order: p_order.cloned(),
                rsp_info: p_rsp_info.cloned(),
                is_last: b_is_last,
            })
            .unwrap();
    }

    fn on_rsp_qry_trade(
        &mut self,
        p_trade: Option<&CThostFtdcTradeField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        n_request_id: i32,
        b_is_last: bool,
    ) {
        debug!("on_rsp_qry_trade");
        self.tx
            .send(TdEvent::RspQryTrade {
                trade: p_trade.cloned(),
                rsp_info: p_rsp_info.cloned(),
                is_last: b_is_last,
            })
            .unwrap();
    }
}

struct TraderConfig {
    broker_id: String,
    user_id: String,
    password: String,
    app_id: String,
    auth_code: String,
    front_address: String,
}

impl TraderConfig {
    fn from_env() -> Self {
        Self {
            broker_id: var("CTP_BROKER_ID").unwrap_or_else(|_| "9999".to_string()),
            user_id: var("SIMNOW_USER_ID").expect("`SIMNOW_USER_ID` is required"),
            password: var("SIMNOW_PASS").expect("`SIMNOW_PASS` is required"),
            app_id: var("CTP_APP_ID").unwrap_or_else(|_| "simnow_client_test".to_string()),
            auth_code: var("CTP_AUTH_CODE").unwrap_or_else(|_| "0000000000000000".to_string()),
            // simnow 7x24 td
            front_address: var("CTP_TD_FRONT")
                .unwrap_or_else(|_| "tcp://182.254.243.31:40001".to_string()),
        }
    }
}

fn dynlib_path() -> std::path::PathBuf {
    let base_dir = var("CARGO_MANIFEST_DIR").unwrap();

    #[cfg(all(target_os = "macos", feature = "ctp_v6_7_13"))]
    let path = "../../ctp-dyn/api/ctp/v6.7.13/v6.7.13_MacOS_20260529/thosttraderapi_se.framework/thosttraderapi_se";
    #[cfg(all(target_os = "linux", feature = "ctp_v6_7_13"))]
    let path = "../../ctp-dyn/api/ctp/v6.7.13/v6.7.13_20260225_api_traderapi_se_linux64/thosttraderapi_se.so";
    #[cfg(all(target_os = "windows", feature = "ctp_v6_7_13"))]
    let path = "../../ctp-dyn/api/ctp/v6.7.13/v6.7.13_20260225_winApi/thosttraderapi_se.dll";

    #[cfg(all(target_os = "macos", feature = "ctp_v6_7_2", not(feature = "ctp_v6_7_13")))]
    let path = "../../ctp-dyn/api/ctp/v6.7.2/v6.7.2_MacOS_20231016/thosttraderapi_se.framework/thosttraderapi_se";
    #[cfg(all(target_os = "linux", feature = "ctp_v6_7_2", not(feature = "ctp_v6_7_13")))]
    let path = "../../ctp-dyn/api/ctp/v6.7.2/v6.7.2_20230913_api_traderapi_se_linux64/thosttraderapi_se.so";
    #[cfg(all(
        target_os = "windows",
        feature = "ctp_v6_7_2",
        not(feature = "ctp_v6_7_13")
    ))]
    let path =
        "../../ctp-dyn/api/ctp/v6.7.2/v6.7.2_20230913_api_traderapi64_se_windows/thosttraderapi_se.dll";

    Path::new(&base_dir).join(path)
}

/// CTP 查询接口有 1 秒流控，每次查询前等待
fn query_throttle() {
    thread::sleep(Duration::from_secs(1));
}

fn run_trader() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    dotenv::dotenv().ok();
    env_logger::init();
    println!("tdapi start here!");

    let config = TraderConfig::from_env();
    let dynlib_path = dynlib_path();
    println!("td dynlib_path: {}", dynlib_path.to_string_lossy());

    #[cfg(feature = "ctp_v6_7_13")]
    let tdapi = TraderApi::create_api(&dynlib_path, "./td_", true);
    #[cfg(not(feature = "ctp_v6_7_13"))]
    let tdapi = TraderApi::create_api(&dynlib_path, "./td_");

    let tdapi = Arc::new(tdapi);
    println!("td get_api_version: {}", tdapi.get_api_version());

    let (tx, rx) = mpsc::sync_channel(1024);
    let tdspi_ptr = Box::into_raw(Box::new(ChannelTraderSpi { tx })) as *mut dyn TraderSpi;

    tdapi.register_spi(tdspi_ptr);
    tdapi.register_front(&config.front_address);

    #[cfg(feature = "ctp_v6_7_13")]
    tdapi.subscribe_private_topic(THOST_TE_RESUME_TYPE::THOST_TERT_QUICK, 1);
    #[cfg(not(feature = "ctp_v6_7_13"))]
    tdapi.subscribe_private_topic(THOST_TE_RESUME_TYPE::THOST_TERT_QUICK);
    tdapi.subscribe_public_topic(THOST_TE_RESUME_TYPE::THOST_TERT_QUICK);

    tdapi.init();
    println!("tdapi init, front: {}", config.front_address);

    let mut request_id = 0;
    let mut next_request_id = || {
        request_id += 1;
        request_id
    };

    while let Ok(event) = rx.recv_timeout(Duration::from_secs(30)) {
        match event {
            // 1. 连接成功 → 客户端认证
            TdEvent::FrontConnected => {
                info!("front connected, request authenticate");
                let mut req = CThostFtdcReqAuthenticateField::default();
                req.BrokerID.set_str(&config.broker_id);
                req.UserID.set_str(&config.user_id);
                req.AppID.set_str(&config.app_id);
                req.AuthCode.set_str(&config.auth_code);
                let ret = tdapi.req_authenticate(&mut req, next_request_id());
                info!("req_authenticate: {ret}");
            }
            TdEvent::FrontDisconnected(reason) => {
                error!("front disconnected, reason: {reason}");
            }
            // 2. 认证成功 → 登录
            TdEvent::RspAuthenticate { rsp_info, is_last } => {
                print_rsp_info!(&rsp_info);
                if rsp_info.as_ref().is_some_and(|r| r.ErrorID != 0) {
                    error!("authenticate failed, abort");
                    break;
                }
                if is_last {
                    info!("authenticate success, request user login");
                    let mut req = CThostFtdcReqUserLoginField::default();
                    req.BrokerID.set_str(&config.broker_id);
                    req.UserID.set_str(&config.user_id);
                    req.Password.set_str(&config.password);
                    // macOS v6.7.2 SDK 的 ReqUserLogin 额外携带终端采集信息参数
                    #[cfg(all(
                        target_os = "macos",
                        feature = "ctp_v6_7_2",
                        not(feature = "ctp_v6_7_13")
                    ))]
                    let ret = tdapi.req_user_login(&mut req, next_request_id(), 0, [0; 273]);
                    #[cfg(not(all(
                        target_os = "macos",
                        feature = "ctp_v6_7_2",
                        not(feature = "ctp_v6_7_13")
                    )))]
                    let ret = tdapi.req_user_login(&mut req, next_request_id());
                    info!("req_user_login: {ret}");
                }
            }
            // 3. 登录成功 → 结算单确认
            TdEvent::RspUserLogin {
                rsp_user_login,
                rsp_info,
                is_last,
            } => {
                print_rsp_info!(&rsp_info);
                if rsp_info.as_ref().is_some_and(|r| r.ErrorID != 0) {
                    error!("user login failed, abort");
                    break;
                }
                if let Some(login) = &rsp_user_login {
                    info!(
                        "login success, trading_day: {}, front_id: {}, session_id: {}, max_order_ref: {}",
                        login.TradingDay.to_string(),
                        login.FrontID,
                        login.SessionID,
                        login.MaxOrderRef.to_string(),
                    );
                }
                if is_last {
                    let mut req = CThostFtdcSettlementInfoConfirmField::default();
                    req.BrokerID.set_str(&config.broker_id);
                    req.InvestorID.set_str(&config.user_id);
                    let ret = tdapi.req_settlement_info_confirm(&mut req, next_request_id());
                    info!("req_settlement_info_confirm: {ret}");
                }
            }
            // 4. 结算单确认 → 查询资金账户
            TdEvent::RspSettlementInfoConfirm { rsp_info, is_last } => {
                print_rsp_info!(&rsp_info);
                if is_last {
                    query_throttle();
                    let mut req = CThostFtdcQryTradingAccountField::default();
                    req.BrokerID.set_str(&config.broker_id);
                    req.InvestorID.set_str(&config.user_id);
                    let ret = tdapi.req_qry_trading_account(&mut req, next_request_id());
                    info!("req_qry_trading_account: {ret}");
                }
            }
            // 5. 资金账户 → 查询持仓
            TdEvent::RspQryTradingAccount {
                account,
                rsp_info,
                is_last,
            } => {
                print_rsp_info!(&rsp_info);
                if let Some(a) = &account {
                    info!(
                        "account[{}] balance: {:.2}, available: {:.2}, margin: {:.2}, close_profit: {:.2}, position_profit: {:.2}",
                        a.AccountID.to_string(),
                        a.Balance,
                        a.Available,
                        a.CurrMargin,
                        a.CloseProfit,
                        a.PositionProfit,
                    );
                }
                if is_last {
                    query_throttle();
                    let mut req = CThostFtdcQryInvestorPositionField::default();
                    req.BrokerID.set_str(&config.broker_id);
                    req.InvestorID.set_str(&config.user_id);
                    let ret = tdapi.req_qry_investor_position(&mut req, next_request_id());
                    info!("req_qry_investor_position: {ret}");
                }
            }
            // 6. 持仓 → 查询报单
            TdEvent::RspQryInvestorPosition {
                position,
                rsp_info,
                is_last,
            } => {
                print_rsp_info!(&rsp_info);
                match &position {
                    Some(p) => info!(
                        "position[{}] direction: {}, total: {}, yd: {}, today: {}, cost: {:.2}",
                        p.InstrumentID.to_string(),
                        p.PosiDirection as u8 as char,
                        p.Position,
                        p.YdPosition,
                        p.TodayPosition,
                        p.PositionCost,
                    ),
                    None => info!("position: none"),
                }
                if is_last {
                    query_throttle();
                    let mut req = CThostFtdcQryOrderField::default();
                    req.BrokerID.set_str(&config.broker_id);
                    req.InvestorID.set_str(&config.user_id);
                    let ret = tdapi.req_qry_order(&mut req, next_request_id());
                    info!("req_qry_order: {ret}");
                }
            }
            // 7. 报单 → 查询成交
            TdEvent::RspQryOrder {
                order,
                rsp_info,
                is_last,
            } => {
                print_rsp_info!(&rsp_info);
                match &order {
                    Some(o) => info!(
                        "order[{}] ref: {}, sys_id: {}, direction: {}, price: {:.2}, volume: {}/{}, status: {}, msg: {}",
                        o.InstrumentID.to_string(),
                        o.OrderRef.to_string(),
                        o.OrderSysID.to_string(),
                        o.Direction as u8 as char,
                        o.LimitPrice,
                        o.VolumeTraded,
                        o.VolumeTotalOriginal,
                        o.OrderStatus as u8 as char,
                        gb18030_cstr_i8_to_str(&o.StatusMsg).unwrap_or_default(),
                    ),
                    None => info!("order: none"),
                }
                if is_last {
                    query_throttle();
                    let mut req = CThostFtdcQryTradeField::default();
                    req.BrokerID.set_str(&config.broker_id);
                    req.InvestorID.set_str(&config.user_id);
                    let ret = tdapi.req_qry_trade(&mut req, next_request_id());
                    info!("req_qry_trade: {ret}");
                }
            }
            // 8. 成交查询完成 → 链条结束
            TdEvent::RspQryTrade {
                trade,
                rsp_info,
                is_last,
            } => {
                print_rsp_info!(&rsp_info);
                match &trade {
                    Some(t) => info!(
                        "trade[{}] trade_id: {}, direction: {}, price: {:.2}, volume: {}, time: {} {}",
                        t.InstrumentID.to_string(),
                        t.TradeID.to_string(),
                        t.Direction as u8 as char,
                        t.Price,
                        t.Volume,
                        t.TradeDate.to_string(),
                        t.TradeTime.to_string(),
                    ),
                    None => info!("trade: none"),
                }
                if is_last {
                    info!("query chain finished, exit");
                    break;
                }
            }
        }
    }

    tdapi.release();
    // release() 后 CTP 内部线程已退出，回收自定义 SPI 对象安全
    let _ = unsafe { Box::from_raw(tdspi_ptr) };
}

fn main() {
    run_trader()
}
