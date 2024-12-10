#![allow(unused_variables)]
use std::{env::var, path::Path, sync::Arc, thread, time::Duration};

use ctp2rs::{
    ffi::{gb18030_cstr_i8_to_str, AssignFromString, WrapToString},
    print_rsp_info,
    v1alpha1::{
        CThostFtdcDepthMarketDataField, CThostFtdcReqUserLoginField, CThostFtdcRspInfoField,
        CThostFtdcRspUserLoginField, CThostFtdcSpecificInstrumentField, MdApi, MdSpi,
    },
};

pub struct BaseMdSpi {
    pub(crate) mdapi: Arc<MdApi>,
}

impl MdSpi for BaseMdSpi {
    fn on_front_connected(&mut self) {
        let mut req = CThostFtdcReqUserLoginField::default();
        println!("mdspi.on_front_connected");
        let user_id = var("OPENCTP_USER_ID").unwrap();
        req.UserID.assign_from_str(&user_id);
        self.mdapi.req_user_login(&mut req, 1);
    }

    fn on_rsp_user_login(
        &mut self,
        rsp_user_login: Option<&CThostFtdcRspUserLoginField>,
        rsp_info: Option<&CThostFtdcRspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        print_rsp_info!(rsp_info);
        println!("on_rsp_user_login!");

        if is_last {
            let instrument_ids = vec!["ag2502".to_string(), "fu2505".to_string()];
            self.mdapi.subscribe_market_data(&instrument_ids);
        }
    }

    fn on_rsp_sub_market_data(
        &mut self,
        specific_instrument: Option<&CThostFtdcSpecificInstrumentField>,
        rsp_info: Option<&CThostFtdcRspInfoField>,
        request_id: i32,
        is_last: bool,
    ) {
        print_rsp_info!(rsp_info);
        println!(
            "on_rsp_sub_market_data: instrument_id[{:?}], {:?}, {:?}",
            specific_instrument.unwrap().InstrumentID.to_string(),
            request_id,
            is_last
        );
    }

    fn on_rtn_depth_market_data(
        &mut self,
        depth_market_data: Option<&CThostFtdcDepthMarketDataField>,
    ) {
        println!("OnRtnDepthMarketData!");

        if let Some(q) = depth_market_data {
            println!(
                "[{} {} {}] {} last_price: {}",
                q.ActionDay.to_string(),
                q.UpdateTime.to_string(),
                q.UpdateMillisec,
                q.InstrumentID.to_string(),
                q.LastPrice,
            )
        } else {
            panic!("tick null!");
        };
    }
}

pub fn run_md() {
    println!("mdapi start here!");

    let base_dir = var("CARGO_MANIFEST_DIR").unwrap();
    println!("base_dir: {base_dir}");
    #[cfg(target_os = "macos")]
    let dynlib_path = "./tts/v6_7_2/mac_arm64/thostmduserapi_se.dylib";

    #[cfg(target_os = "linux")]
    let dynlib_path = "./tts/v6_7_2/lin64/thostmduserapi_se.so";

    let dynlib_path = Path::new(&base_dir).join(dynlib_path);
    println!("dynlib_path: {}", dynlib_path.as_path().to_string_lossy());

    let mdapi = MdApi::create_api(dynlib_path, "./md_", false, false);

    let mdapi = Arc::new(mdapi);

    let base_mdspi: BaseMdSpi = BaseMdSpi {
        mdapi: Arc::clone(&mdapi),
    };
    let mdspi_box = Box::new(base_mdspi);
    println!("get_api_version: {}", mdapi.get_api_version());

    mdapi.register_front("tcp://121.37.80.177:20004"); // tts 7x24 md

    let mdspi_ptr = Box::into_raw(mdspi_box) as *mut dyn MdSpi;
    let mdspi_ptr2 = mdspi_ptr.clone();

    mdapi.register_spi(mdspi_ptr);

    mdapi.init();

    println!("mdapi init");

    loop {
        println!("md loop");
        thread::sleep(Duration::from_secs(10));
    }
    // let _ = unsafe { Box::from_raw(mdspi_ptr2) };
}
