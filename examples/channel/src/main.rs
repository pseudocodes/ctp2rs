#![allow(unused_variables)]
use std::{
    env::var,
    path::Path,
    sync::mpsc::{self, SyncSender},
};

use ctp2rs::ffi::{gb18030_cstr_i8_to_str, AssignFromString, WrapToString};
use ctp2rs::{print_rsp_info, v1alpha1::*};
use log::*;

pub struct ChannelSpi {
    tx: SyncSender<MdSpiEvent>,
}

impl MdSpi for ChannelSpi {
    fn on_front_connected(&mut self) {
        debug!("on_front_connected");
        self.tx
            .send(MdSpiEvent::OnFrontConnected(MdSpiOnFrontConnectedEvent {}))
            .unwrap();
    }

    fn on_front_disconnected(&mut self, reason: i32) {
        debug!("on_front_disconnected {}", reason);
        self.tx
            .send(MdSpiEvent::OnFrontDisconnected(
                MdSpiOnFrontDisconnectedEvent { reason: reason },
            ))
            .unwrap();
    }

    fn on_heart_beat_warning(&mut self, time_lapse: i32) {
        debug!("on_heart_beating {}", time_lapse);
    }

    fn on_rsp_user_login(
        &mut self,
        rsp_user_login: Option<&CThostFtdcRspUserLoginField>,
        rsp_info: Option<&CThostFtdcRspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        debug!("on_rsp_user_login");
        self.tx
            .send(MdSpiEvent::OnRspUserLogin(MdSpiOnRspUserLoginEvent {
                rsp_user_login: rsp_user_login.cloned(),
                rsp_info: rsp_info.cloned(),
                request_id,
                is_last,
            }))
            .unwrap();
    }

    fn on_rsp_error(
        &mut self,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        n_request_id: i32,
        b_is_last: bool,
    ) {
        debug!("rsp_error");
        print_rsp_info!(p_rsp_info);
    }

    fn on_rsp_sub_market_data(
        &mut self,
        specific_instrument: Option<&CThostFtdcSpecificInstrumentField>,
        rsp_info: Option<&CThostFtdcRspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        debug!("on_rsp_sub_market_data");
        self.tx
            .send(MdSpiEvent::OnRspSubMarketData(
                MdSpiOnRspSubMarketDataEvent {
                    specific_instrument: specific_instrument.cloned(),
                    rsp_info: rsp_info.cloned(),
                    request_id: request_id,
                    is_last: is_last,
                },
            ))
            .unwrap();
    }

    fn on_rtn_depth_market_data(
        &mut self,
        depth_market_data: Option<&CThostFtdcDepthMarketDataField>,
    ) {
        self.tx
            .send(MdSpiEvent::OnRtnDepthMarketData(
                MdSpiOnRtnDepthMarketDataEvent {
                    depth_market_data: depth_market_data.cloned(),
                },
            ))
            .unwrap();
    }
}

pub fn run_channel_md() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    dotenv::dotenv().ok();
    env_logger::init();
    println!("mdapi start here!");

    let base_dir = var("CARGO_MANIFEST_DIR").unwrap();
    println!("base_dir: {base_dir}");
    #[cfg(target_os = "macos")]
    let dynlib_path =
        "../../ctp-dyn/api/ctp/v6.7.2/v6.7.2_MacOS_20231016/thostmduserapi_se.framework/thostmduserapi_se";

    #[cfg(all(target_os = "linux", not(feature = "ctp_v6_7_11")))]
    let dynlib_path =
        "../../ctp-dyn/api/ctp/v6.7.2/v6.7.2_20230913_api_traderapi_se_linux64/thostmduserapi_se.so";

    #[cfg(all(target_os = "linux", feature = "ctp_v6_7_11"))]
    let dynlib_path = "../../ctp-dyn/api/ctp/v6.7.11/v6.7.11_20250617_api_traderapi_se_linux64/thostmduserapi_se.so";

    #[cfg(all(target_os = "windows", not(feature = "ctp_v6_7_11")))]
    let dynlib_path =
        "../../ctp-dyn/api/ctp/v6.7.2/v6.7.2_20230913_api_traderapi64_se_windows/thostmduserapi_se.dll";

    #[cfg(all(target_os = "windows", feature = "ctp_v6_7_11"))]
    let dynlib_path =
        "../../ctp-dyn/api/ctp/v6.7.11/v6.7.11_20250617_traderapi64_se_windows/thostmduserapi_se.dll";

    let dynlib_path = Path::new(&base_dir).join(dynlib_path);
    println!("{}", dynlib_path.as_os_str().to_string_lossy());

    #[cfg(not(feature = "ctp_v6_7_11"))]
    let mdapi = MdApi::create_api(dynlib_path, "./md_", false, false);

    #[cfg(feature = "ctp_v6_7_11")]
    let mdapi = MdApi::create_api(dynlib_path, "./md_", false, false, true);

    let (tx, rx) = mpsc::sync_channel(1024);
    let mdspi = ChannelSpi { tx: tx };
    let mdspi_box = Box::new(mdspi);

    println!("get_api_version: {}", mdapi.get_api_version());

    mdapi.register_front("tcp://182.254.243.31:30011");

    let mdspi_ptr = Box::into_raw(mdspi_box) as *mut dyn MdSpi;
    let mdspi_ptr2 = mdspi_ptr.clone();

    mdapi.register_spi(mdspi_ptr);

    mdapi.init();

    println!("mdapi init");

    match rx.recv_timeout(std::time::Duration::from_secs(5)) {
        Err(_) => error!("Timeout try recv `req_init`"),
        Ok(MdSpiEvent::OnFrontConnected(_)) => {}
        Ok(event) => {
            error!("invalid event: {:?}", event);
            return;
        }
    }
    let mut req = CThostFtdcReqUserLoginField::default();
    req.BrokerID.assign_from_str("9999");
    let user_id = var("SIMNOW_USER_ID"). unwrap();
    req.UserID.assign_from_str(&user_id);

    mdapi.req_user_login(&mut req, 1);

    match rx.recv_timeout(std::time::Duration::from_secs(5)) {
        Err(_) => error!("Timeout try recv `req_user_login`"),
        Ok(MdSpiEvent::OnRspUserLogin(rsp)) => {
            let instrument_ids = vec!["ag2512".to_string(), "fu2601".to_string()];
            mdapi.subscribe_market_data(&instrument_ids);
        }
        Ok(event) => {
            error!("invalid event: {:?}", event);
            return;
        }
    }

    while let Ok(event) = rx.recv() {
        match event {
            MdSpiEvent::OnRtnDepthMarketData(event) => {
                let q = event.depth_market_data.unwrap();
                info!(
                    "{} {} {} last_price: {}",
                    q.ActionDay.to_string(),
                    q.UpdateTime.to_string(),
                    q.InstrumentID.to_string(),
                    q.LastPrice,
                )
            }
            MdSpiEvent::OnRspSubMarketData(event) => {
                print_rsp_info!(event.rsp_info);
                info!(
                    "on_rsp_sub_market_data: {}",
                    event.specific_instrument.unwrap().InstrumentID.to_string()
                )
            }
            _ => debug!("Got event: {:?}", event),
        }
    }
    mdapi.join();
    let _ = unsafe { Box::from_raw(mdspi_ptr2) };

    // Ok(())
}

fn main() {
    run_channel_md()
}
