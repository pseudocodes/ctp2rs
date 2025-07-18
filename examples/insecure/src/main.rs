/* generated by handle_module_dynlib_symbol_api */

use std::{
    env::var,
    ffi::{CStr, CString},
    os::raw::{c_char, c_void},
    path::Path,
    sync::Arc,
};

use libloading::os::unix as imp;
use libloading::Library;

use ctp2rs::ffi::gb18030_cstr_i8_to_str;
use ctp2rs::{ffi::AssignFromString, print_rsp_info, v1alpha1::bindings::*};

type TraderApiCreator = unsafe extern "C" fn(*const c_char) -> *mut c_void;
type GetApiVersion = unsafe extern "C" fn() -> *const c_char;
type UnaryFn = unsafe extern "C" fn(*mut c_void);
type JoinFn = unsafe extern "C" fn(*mut c_void) -> ::std::os::raw::c_int;
// type GetString = unsafe extern "C" fn(*mut c_void) -> *const ::std::os::raw::c_char;
type RegisterStringFn = unsafe extern "C" fn(*mut c_void, *mut ::std::os::raw::c_char);
type RegisterStructFn = unsafe extern "C" fn(*mut c_void, *mut ::std::os::raw::c_void);
type SubscribeTopicFn = unsafe extern "C" fn(*mut c_void, ::std::os::raw::c_int);
type ReqFn = unsafe extern "C" fn(
    *mut c_void,
    *mut ::std::os::raw::c_void,
    ::std::os::raw::c_int,
) -> ::std::os::raw::c_int;

fn fetch_func_ptr<T>(dynlib: &Library, symbol: &[u8]) -> Option<imp::Symbol<T>> {
    let r = if let Some(f) = unsafe { dynlib.get::<T>(symbol).ok() } {
        Some(unsafe { f.into_raw() })
    } else {
        None
    };
    // println!("{}: {:?}", String::from_utf8_lossy(symbol), r);
    r
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Default)]
pub struct TraderApiFnTable {
    pub CreateFtdcTraderApi: Option<imp::Symbol<TraderApiCreator>>,
    pub GetApiVersion: Option<imp::Symbol<GetApiVersion>>,
    pub Init: Option<imp::Symbol<UnaryFn>>,
    pub Join: Option<imp::Symbol<JoinFn>>,
    pub RegisterFront: Option<imp::Symbol<RegisterStringFn>>,
    pub RegisterSpi: Option<imp::Symbol<RegisterStructFn>>,
    pub SubscribePrivateTopic: Option<imp::Symbol<SubscribeTopicFn>>,
    pub SubscribePublicTopic: Option<imp::Symbol<SubscribeTopicFn>>,
    pub ReqAuthenticate: Option<imp::Symbol<ReqFn>>,
    pub ReqUserLogin: Option<imp::Symbol<ReqFn>>,
}

impl TraderApiFnTable {
    pub fn new(dynlib: &Library) -> Self {
        Self {
            CreateFtdcTraderApi: fetch_func_ptr::<TraderApiCreator>(
                dynlib,
                b"_ZN19CThostFtdcTraderApi19CreateFtdcTraderApiEPKc",
            ),
            GetApiVersion: fetch_func_ptr::<GetApiVersion>(
                dynlib,
                b"_ZN19CThostFtdcTraderApi13GetApiVersionEv",
            ),

            Init: fetch_func_ptr::<UnaryFn>(dynlib, b"_ZN18CFtdcTraderApiImpl4InitEv"),
            Join: fetch_func_ptr::<JoinFn>(dynlib, b"_ZN18CFtdcTraderApiImpl4JoinEv"),
            RegisterFront: fetch_func_ptr::<RegisterStringFn>(
                dynlib,
                b"_ZN18CFtdcTraderApiImpl13RegisterFrontEPc",
            ),
            RegisterSpi: fetch_func_ptr::<RegisterStructFn>(
                dynlib,
                b"_ZN18CFtdcTraderApiImpl11RegisterSpiEP19CThostFtdcTraderSpi",
            ),
            SubscribePrivateTopic: fetch_func_ptr::<SubscribeTopicFn>(
                dynlib,
                b"_ZN18CFtdcTraderApiImpl21SubscribePrivateTopicE20THOST_TE_RESUME_TYPE",
            ),
            SubscribePublicTopic: fetch_func_ptr::<SubscribeTopicFn>(
                dynlib,
                b"_ZN18CFtdcTraderApiImpl20SubscribePublicTopicE20THOST_TE_RESUME_TYPE",
            ),
            ReqAuthenticate: fetch_func_ptr::<ReqFn>(
                dynlib,
                b"_ZN18CFtdcTraderApiImpl15ReqAuthenticateEP30CThostFtdcReqAuthenticateFieldi",
            ),
            ReqUserLogin: fetch_func_ptr::<ReqFn>(
                dynlib,
                b"_ZN18CFtdcTraderApiImpl12ReqUserLoginEP27CThostFtdcReqUserLoginFieldi",
            ),
        }
    }
}

use ctp2rs::v1alpha1::traderspi::*;

#[derive(Debug)]
pub struct TraderApi {
    pub api_ptr: *mut c_void,
    pub dynlib: Option<Library>,
    pub vt_: TraderApiFnTable,
}

unsafe impl Sync for TraderApi {}
unsafe impl Send for TraderApi {}

impl TraderApi {
    pub fn create_api<P, F>(dynlib_path: P, flow_path: F) -> Self
    where
        P: AsRef<Path>,
        F: AsRef<Path>,
    {
        // CThostFtdcMdApi_CreateFtdcMdApi(flow_path, is_using_udp, is_multicast)
        let dynlib =
            unsafe { libloading::Library::new(dynlib_path.as_ref()).expect("failed to open") };
        let vt_ = TraderApiFnTable::new(&dynlib);
        let create_api = vt_.CreateFtdcTraderApi.as_ref().unwrap();
        let cflow_path = CString::new(flow_path.as_ref().to_str().unwrap()).expect("fail to new");
        let api_ptr: *mut c_void = unsafe { create_api(cflow_path.as_ptr()) };

        Self {
            api_ptr: api_ptr,
            dynlib: Some(dynlib),
            vt_: vt_,
        }
    }

    pub fn get_api_version(&self) -> String {
        unsafe {
            let get_api_version = self.vt_.GetApiVersion.as_ref().unwrap();
            let cstr_ptr = get_api_version();
            let c_str: &CStr = CStr::from_ptr(cstr_ptr);
            c_str.to_string_lossy().to_string()
        }
    }

    pub fn init(&self) {
        unsafe { self.vt_.Init.as_ref().unwrap()(self.api_ptr) }
    }

    pub fn join(&self) -> i32 {
        unsafe { self.vt_.Join.as_ref().unwrap()(self.api_ptr) }
    }

    pub fn register_front(&self, psz_front_address: &str) {
        unsafe {
            let psz_front_address = CString::new(psz_front_address).unwrap();
            self.vt_.RegisterFront.as_ref().unwrap()(
                self.api_ptr,
                psz_front_address.as_ptr() as *mut _,
            )
        }
    }

    pub fn register_spi(&self, p_spi: *mut dyn TraderSpi) {
        let spi_ptr = Box::into_raw(Box::new(CThostFtdcTraderSpiExt::new(p_spi)));
        unsafe { self.vt_.RegisterSpi.as_ref().unwrap()(self.api_ptr, spi_ptr as _) }
    }

    pub fn subscribe_private_topic(&self, n_resume_type: THOST_TE_RESUME_TYPE) {
        unsafe {
            self.vt_.SubscribePrivateTopic.as_ref().unwrap()(self.api_ptr, n_resume_type as _)
        }
    }

    pub fn subscribe_public_topic(&self, n_resume_type: THOST_TE_RESUME_TYPE) {
        unsafe { self.vt_.SubscribePublicTopic.as_ref().unwrap()(self.api_ptr, n_resume_type as _) }
    }

    pub fn req_authenticate(
        &self,
        p_req_authenticate_field: &mut CThostFtdcReqAuthenticateField,
        n_request_id: i32,
    ) -> i32 {
        unsafe {
            self.vt_.ReqAuthenticate.as_ref().unwrap()(
                self.api_ptr,
                p_req_authenticate_field as *mut CThostFtdcReqAuthenticateField as _,
                n_request_id,
            )
        }
    }

    pub fn req_user_login(
        &self,
        p_req_user_login_field: &mut CThostFtdcReqUserLoginField,
        n_request_id: i32,
    ) -> i32 {
        unsafe {
            self.vt_.ReqUserLogin.as_ref().unwrap()(
                self.api_ptr,
                p_req_user_login_field as *mut CThostFtdcReqUserLoginField as _,
                n_request_id,
            )
        }
    }
}

struct BaseTraderSpi {
    tdapi: Arc<TraderApi>,
    pub request_id: i32,
}

impl TraderSpi for BaseTraderSpi {
    fn on_front_connected(&mut self) {
        println!("tdspi.on_front_connected !!!");
        let mut req = CThostFtdcReqAuthenticateField::default();
        let user_id = var("SIMNOW_USER_ID").unwrap();
        req.BrokerID.assign_from_str("9999");
        req.UserID.assign_from_str(&user_id);
        req.AppID.assign_from_str("simnow_client_test");
        req.AuthCode.assign_from_str("0000000000000000");

        self.request_id += 1;
        self.tdapi.req_authenticate(&mut req, self.request_id);
    }

    fn on_front_disconnected(&mut self, n_reason: i32) {
        println!("on_front_disconnected: reason{n_reason}")
    }

    fn on_rsp_authenticate(
        &mut self,
        _p_rsp_authenticate_field: Option<&CThostFtdcRspAuthenticateField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        _n_request_id: i32,
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
            let user_id = var("SIMNOW_USER_ID").unwrap();
            let password = var("SIMNOW_PASS").unwrap();

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
        _p_rsp_user_login: Option<&CThostFtdcRspUserLoginField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        _n_request_id: i32,
        b_is_last: bool,
    ) {
        print_rsp_info!(p_rsp_info);
        if b_is_last {}
    }
}

fn main() {
    println!("tdapi start here!");
    let base_dir = var("CARGO_MANIFEST_DIR").unwrap();
    println!("base_dir: {base_dir}");

    if cfg!(not(target_os = "linux")) {
        panic!("Only support for Linux! ")
    }
    let dynlib_path =
        "../../ctp-dyn/api/ctp/v6.7.2/v6.7.2_20230913_api_traderapi_se_linux64/thosttraderapi_se.so";

    let dynlib_path = Path::new(&base_dir).join(dynlib_path);

    let tdapi = TraderApi::create_api(dynlib_path.as_path(), "./td_");
    let tdapi = Arc::new(tdapi);
    let base_tdspi = BaseTraderSpi {
        tdapi: Arc::clone(&tdapi),
        request_id: 0,
    };
    let tdspi_box = Box::new(base_tdspi);
    let tdspi_ptr = Box::into_raw(tdspi_box);
    let _tdspi_ptr2 = tdspi_ptr.clone();
    println!("get_api_version: {}", tdapi.get_api_version());

    tdapi.register_front("tcp://182.254.243.31:40001"); // simnow 7x24 td

    tdapi.register_spi(tdspi_ptr);
    tdapi.subscribe_private_topic(THOST_TE_RESUME_TYPE::THOST_TERT_QUICK);
    tdapi.subscribe_public_topic(THOST_TE_RESUME_TYPE::THOST_TERT_QUICK);

    tdapi.init();

    println!("tdapi init");

    tdapi.join();
}
