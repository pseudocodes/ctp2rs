#![allow(unused_variables)]
use std::{env::var, path::Path, sync::Arc, thread, time::Duration};

use ctp2rs::{
    ffi::{gb18030_cstr_i8_to_str, AssignFromString, WrapToString},
    print_rsp_info,
    v1alpha1::{
        CThostFtdcInvestorPositionField, CThostFtdcQryInvestorPositionField,
        CThostFtdcReqAuthenticateField, CThostFtdcReqUserLoginField,
        CThostFtdcRspAuthenticateField, CThostFtdcRspInfoField, CThostFtdcRspUserLoginField,
        CThostFtdcSettlementInfoConfirmField, TraderApi, TraderSpi, THOST_TE_RESUME_TYPE,
    },
};
pub struct BaseTraderSpi {
    pub tdapi: Arc<TraderApi>,
    pub request_id: i32,
}

impl TraderSpi for BaseTraderSpi {
    fn on_front_connected(&mut self) {
        println!("tdspi.on_front_connected !!!");
        let mut req = CThostFtdcReqAuthenticateField::default();
        let user_id = var("OPENCTP_USER_ID").unwrap();
        req.BrokerID.assign_from_str("9999");
        req.UserID.assign_from_str(&user_id);
        req.AppID.assign_from_str("simnow_client_test");
        req.AuthCode.assign_from_str("0000000000000000");

        self.request_id += 1;
        self.tdapi.req_authenticate(&mut req, self.request_id);
    }

    fn on_front_disconnected(&mut self, n_reason: i32) {
        println!("on_front_disconnected: reason -> {n_reason}")
    }

    fn on_heart_beat_warning(&mut self, n_time_lapse: i32) {}

    fn on_rsp_authenticate(
        &mut self,
        p_rsp_authenticate_field: Option<&CThostFtdcRspAuthenticateField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        n_request_id: i32,
        b_is_last: bool,
    ) {
        println!("on_rsp_authenticate");
        print_rsp_info!(p_rsp_info);
        if let Some(p_rsp_info) = p_rsp_info {
            if p_rsp_info.ErrorID != 0 {
                return;
            }
        }

        if b_is_last {
            let mut req = CThostFtdcReqUserLoginField::default();
            let user_id = var("OPENCTP_USER_ID").unwrap();
            let password = var("OPENCTP_PASS").unwrap();

            req.BrokerID.assign_from_str("9999");
            req.UserID.assign_from_str(&user_id);
            req.Password.assign_from_str(&password);

            self.request_id += 1;
            let ret = self.tdapi.req_user_login(&mut req, self.request_id);
            println!("req_user_login result: {ret}");
        }
    }

    fn on_rsp_user_login(
        &mut self,
        p_rsp_user_login: Option<&CThostFtdcRspUserLoginField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        n_request_id: i32,
        b_is_last: bool,
    ) {
        print_rsp_info!(p_rsp_info);
        if b_is_last {
            let mut req = CThostFtdcSettlementInfoConfirmField::default();
            req.BrokerID.assign_from_str("9999");
            let user_id = var("OPENCTP_USER_ID").unwrap();
            req.InvestorID.assign_from_str(&user_id);

            self.request_id += 1;
            let ret = self
                .tdapi
                .req_settlement_info_confirm(&mut req, self.request_id);
            println!("req_user_login result: {ret}");
        }
    }

    fn on_rsp_settlement_info_confirm(
        &mut self,
        p_settlement_info_confirm: Option<&CThostFtdcSettlementInfoConfirmField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        n_request_id: i32,
        b_is_last: bool,
    ) {
        print_rsp_info!(p_rsp_info);
        if b_is_last {
            std::thread::sleep(std::time::Duration::from_secs(1));
            self.request_id += 1;
            let mut req = CThostFtdcQryInvestorPositionField::default();
            let ret = self
                .tdapi
                .req_qry_investor_position(&mut req, self.request_id);
            println!("req_qry_investor_position result: {ret}");
        }
    }

    fn on_rsp_qry_investor_position(
        &mut self,
        p_investor_position: Option<&CThostFtdcInvestorPositionField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        n_request_id: i32,
        b_is_last: bool,
    ) {
        print_rsp_info!(p_rsp_info);
        if let Some(p) = p_investor_position {
            let instrument_id = p.InstrumentID.to_string();
            let user_id = p.InvestorID.to_string();
            println!("{user_id} holds {instrument_id}");
        } else {
            println!("position hold None");
        }
        if b_is_last {
            println!("on_rsp_qry_investor_position finish!");
        }
    }
}

pub fn run_td() {
    println!("tdapi start here!");
    let base_dir = var("CARGO_MANIFEST_DIR").unwrap();
    println!("base_dir: {base_dir}");

    #[cfg(target_os = "macos")]
    let dynlib_path = "./tts/v6_7_2/mac_arm64/thosttraderapi_se.dylib";
    #[cfg(target_os = "linux")]
    let dynlib_path = "./tts/v6_7_2/lin64/thosttraderapi_se.so";
    #[cfg(target_os = "windows")]
    let dynlib_path = "./tts/v6_7_2/win64/thosttraderapi_se.dll";

    let dynlib_path = Path::new(&base_dir).join(dynlib_path);
    println!(
        "td dynlib_path: {}",
        dynlib_path.as_path().to_string_lossy()
    );

    let tdapi = TraderApi::create_api(dynlib_path.as_path(), "./td_");
    let tdapi = Arc::new(tdapi);
    let base_tdspi = BaseTraderSpi {
        tdapi: Arc::clone(&tdapi),
        request_id: 0,
    };
    let tdspi_box = Box::new(base_tdspi);
    let tdspi_ptr = Box::into_raw(tdspi_box);
    let tdspi_ptr2 = tdspi_ptr.clone();
    println!("get_api_version: {}", tdapi.get_api_version());

    tdapi.register_front("tcp://121.37.80.177:20004"); // tts 7x24 td

    tdapi.register_spi(tdspi_ptr);
    tdapi.subscribe_private_topic(THOST_TE_RESUME_TYPE::THOST_TERT_QUICK);
    tdapi.subscribe_public_topic(THOST_TE_RESUME_TYPE::THOST_TERT_QUICK);

    tdapi.init();

    println!("tdapi init");

    loop {
        println!("td loop");
        thread::sleep(Duration::from_secs(10));
    }
    // let _ = unsafe { Box::from_raw(tdspi_ptr2) };
}
