#![allow(unused_variables)]
use std::{sync::Arc, thread, time::Duration};

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

use crate::CtpAccountConfig;

pub struct BaseTraderSpi {
    pub tdapi: Arc<TraderApi>,
    pub request_id: i32,
    pub config: CtpAccountConfig,
}

impl TraderSpi for BaseTraderSpi {
    fn on_front_connected(&mut self) {
        println!("tdspi.on_front_connected !!!");
        let mut req = CThostFtdcReqAuthenticateField::default();
        req.BrokerID.assign_from_str("9999");
        req.UserID.assign_from_str(&self.config.td_user_id);
        req.AppID.assign_from_str(&self.config.td_app_id);
        req.AuthCode.assign_from_str(&self.config.td_auth_code);

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
            req.BrokerID.assign_from_str("9999");
            req.UserID.assign_from_str(&self.config.td_user_id);
            req.Password.assign_from_str(&self.config.td_password);

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
            req.InvestorID.assign_from_str(&self.config.td_user_id);

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

pub fn run_td(config: CtpAccountConfig) {
    println!("tdapi start here!");
    println!(
        "td dynlib_path: {}",
        config.td_dynlib_path.to_string_lossy()
    );

    #[cfg(not(feature = "ctp_v6_7_11"))]
    let tdapi = TraderApi::create_api(&config.td_dynlib_path, "./td_");

    #[cfg(feature = "ctp_v6_7_11")]
    let tdapi = TraderApi::create_api(&config.td_dynlib_path, "./td_", true);

    let tdapi = Arc::new(tdapi);

    // 先获取 front_address，避免 move 后的借用问题
    let front_address = config.td_front_address.clone();

    let base_tdspi = BaseTraderSpi {
        tdapi: Arc::clone(&tdapi),
        request_id: 0,
        config,
    };
    let tdspi_box = Box::new(base_tdspi);
    let tdspi_ptr = Box::into_raw(tdspi_box);
    let tdspi_ptr2 = tdspi_ptr.clone();
    println!("td get_api_version: {}", tdapi.get_api_version());

    tdapi.register_front(&front_address);

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
