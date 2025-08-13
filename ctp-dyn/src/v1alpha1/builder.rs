use std::cell::Cell;
use std::error::Error;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::Path;
use std::ptr::null_mut;

use libloading::Library;

use crate::v1alpha1::{CThostFtdcMdApi, CThostFtdcTraderApi, MdApi, TraderApi};

mod symbols {
    // 合并相同的 GET_API_VERSION 符号
    #[cfg(not(target_os = "windows"))]
    pub const MDAPI_GET_API_VERSION_SYMBOL: &[u8] = if cfg!(feature = "sopt") {
        b"_ZN8ctp_sopt15CThostFtdcMdApi13GetApiVersionEv"
    } else {
        b"_ZN15CThostFtdcMdApi13GetApiVersionEv"
    };

    #[cfg(not(target_os = "windows"))]
    pub const TDAPI_GET_API_VERSION_SYMBOL: &[u8] = if cfg!(feature = "sopt") {
        b"_ZN8ctp_sopt19CThostFtdcTraderApi13GetApiVersionEv"
    } else {
        b"_ZN19CThostFtdcTraderApi13GetApiVersionEv"
    };

    #[cfg(target_os = "windows")]
    pub const MDAPI_GET_API_VERSION_SYMBOL: &[u8] = if cfg!(feature = "sopt") {
        b"?GetApiVersion@CThostFtdcMdApi@ctp_sopt@@SAPEBDXZ"
    } else {
        b"?GetApiVersion@CThostFtdcMdApi@@SAPEBDXZ"
    };

    #[cfg(target_os = "windows")]
    pub const TDAPI_GET_API_VERSION_SYMBOL: &[u8] = if cfg!(feature = "sopt") {
        b"?GetApiVersion@CThostFtdcTraderApi@ctp_sopt@@SAPEBDXZ"
    } else {
        b"?GetApiVersion@CThostFtdcTraderApi@@SAPEBDXZ"
    };

    // CREATE 符号需要按具体条件分别定义
    #[cfg(all(not(feature = "sopt"), not(target_os = "windows")))]
    pub const MDAPI_CREATE_API_SYMBOL: &[u8] = if cfg!(feature = "ctp_v6_7_11") {
        b"_ZN15CThostFtdcMdApi15CreateFtdcMdApiEPKcbbb"
    } else {
        b"_ZN15CThostFtdcMdApi15CreateFtdcMdApiEPKcbb"
    };

    #[cfg(all(not(feature = "sopt"), not(target_os = "windows")))]
    pub const TDAPI_CREATE_API_SYMBOL: &[u8] = if cfg!(feature = "ctp_v6_7_11") {
        b"_ZN19CThostFtdcTraderApi19CreateFtdcTraderApiEPKcb"
    } else {
        b"_ZN19CThostFtdcTraderApi19CreateFtdcTraderApiEPKc"
    };

    #[cfg(all(feature = "sopt", not(target_os = "windows")))]
    pub const MDAPI_CREATE_API_SYMBOL: &[u8] =
        b"_ZN8ctp_sopt15CThostFtdcMdApi15CreateFtdcMdApiEPKcbb";

    #[cfg(all(feature = "sopt", not(target_os = "windows")))]
    pub const TDAPI_CREATE_API_SYMBOL: &[u8] =
        b"_ZN8ctp_sopt19CThostFtdcTraderApi19CreateFtdcTraderApiEPKc";

    #[cfg(all(not(feature = "sopt"), target_os = "windows"))]
    pub const MDAPI_CREATE_API_SYMBOL: &[u8] = if cfg!(feature = "ctp_v6_7_11") {
        b"?CreateFtdcMdApi@CThostFtdcMdApi@@SAPEAV1@PEBD_N1_N@Z"
    } else {
        b"?CreateFtdcMdApi@CThostFtdcMdApi@@SAPEAV1@PEBD_N1@Z"
    };

    #[cfg(all(not(feature = "sopt"), target_os = "windows"))]
    pub const TDAPI_CREATE_API_SYMBOL: &[u8] = if cfg!(feature = "ctp_v6_7_11") {
        b"?CreateFtdcTraderApi@CThostFtdcTraderApi@@SAPEAV1@PEBD_N@Z"
    } else {
        b"?CreateFtdcTraderApi@CThostFtdcTraderApi@@SAPEAV1@PEBD@Z"
    };

    #[cfg(all(feature = "sopt", target_os = "windows"))]
    pub const MDAPI_CREATE_API_SYMBOL: &[u8] =
        b"?CreateFtdcMdApi@CThostFtdcMdApi@ctp_sopt@@SAPEAV12@PEBD_N1@Z";

    #[cfg(all(feature = "sopt", target_os = "windows"))]
    pub const TDAPI_CREATE_API_SYMBOL: &[u8] =
        b"?CreateFtdcTraderApi@CThostFtdcTraderApi@ctp_sopt@@SAPEAV12@PEBD@Z";
}
pub use symbols::*;

impl MdApi {
    #[cfg(not(feature = "ctp_v6_7_11"))]
    pub fn create_api<P: AsRef<Path>, F: AsRef<Path>>(
        dynlib_path: P,
        flow_path: F,
        is_using_udp: bool,
        is_multicast: bool,
    ) -> Self {
        let dynlib =
            unsafe { libloading::Library::new(dynlib_path.as_ref()).expect("failed to open") };
        type MdApiCreator = unsafe extern "C" fn(*const c_char, bool, bool) -> *mut CThostFtdcMdApi;
        let create_api: libloading::Symbol<MdApiCreator> =
            unsafe { dynlib.get(MDAPI_CREATE_API_SYMBOL).expect("failed to get") };
        let cflow_path = CString::new(flow_path.as_ref().to_str().unwrap()).expect("fail to new");
        let api_ptr: *mut CThostFtdcMdApi =
            unsafe { create_api(cflow_path.as_ptr(), is_using_udp, is_multicast) };

        Self {
            api_ptr: api_ptr,
            spi_ptr: Cell::new(null_mut()),
            dynlib: Some(dynlib),
        }
    }

    #[cfg(feature = "ctp_v6_7_11")]
    pub fn create_api<P: AsRef<Path>, F: AsRef<Path>>(
        dynlib_path: P,
        flow_path: F,
        is_using_udp: bool,
        is_multicast: bool,
        is_production_mode: bool,
    ) -> Self {
        // CThostFtdcMdApi_CreateFtdcMdApi(flow_path, is_using_udp, is_multicast)
        let dynlib =
            unsafe { libloading::Library::new(dynlib_path.as_ref()).expect("failed to open") };
        type MdApiCreator =
            unsafe extern "C" fn(*const c_char, bool, bool, bool) -> *mut CThostFtdcMdApi;
        let create_api: libloading::Symbol<MdApiCreator> =
            unsafe { dynlib.get(MDAPI_CREATE_API_SYMBOL).expect("failed to get") };
        let cflow_path = CString::new(flow_path.as_ref().to_str().unwrap()).expect("fail to new");
        let api_ptr: *mut CThostFtdcMdApi = unsafe {
            create_api(
                cflow_path.as_ptr(),
                is_using_udp,
                is_multicast,
                is_production_mode,
            )
        };

        Self {
            api_ptr: api_ptr,
            spi_ptr: Cell::new(null_mut()),
            dynlib: Some(dynlib),
        }
    }

    pub fn get_api_version(&self) -> String {
        unsafe {
            type MdGetApiVersion = unsafe extern "C" fn() -> *const c_char;
            let get_api_version: libloading::Symbol<MdGetApiVersion> = self
                .dynlib
                .as_ref()
                .unwrap()
                .get(MDAPI_GET_API_VERSION_SYMBOL)
                .unwrap();
            let cstr_ptr = get_api_version();
            let c_str: &CStr = CStr::from_ptr(cstr_ptr);
            c_str.to_string_lossy().to_string()
        }
    }
}

impl Default for MdApi {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

impl Drop for MdApi {
    fn drop(&mut self) {
        let spi_ptr = self.spi_ptr.get();
        if !spi_ptr.is_null() {
            unsafe {
                ((*(*self.api_ptr).vtable_).CThostFtdcMdApi_RegisterSpi)(
                    self.api_ptr,
                    std::ptr::null_mut(),
                );
                let last_spi = Box::from_raw(spi_ptr); // 释放动态分配的内存
                drop(last_spi);
            }
        }
        unsafe {
            if !self.api_ptr.is_null() {
                ((*(*self.api_ptr).vtable_).CThostFtdcMdApi_Release)(self.api_ptr);
            }
            let dynlib = self.dynlib.take();
            let _ = dynlib.unwrap().close();
        }
    }
}

#[derive(Debug, Default)]
pub struct MdApiBuilder {
    flow_path: Option<String>,
    use_udp: bool,
    use_multicast: bool,
    use_production_mode: bool,
    dynlib: Option<String>,
}

impl MdApiBuilder {
    pub fn new() -> Self {
        Self {
            flow_path: None,
            use_udp: false,
            use_multicast: false,
            use_production_mode: true, // 默认使用生产模式
            dynlib: None,
        }
    }

    pub fn from_dynlib(self, path: &str) -> Self {
        Self {
            dynlib: Some(path.to_string()),
            ..self
        }
    }

    pub fn flow_path<P: AsRef<Path>>(self, value: P) -> Self {
        Self {
            flow_path: Some(value.as_ref().as_os_str().to_str().unwrap().into()),
            ..self
        }
    }

    pub fn using_udp(self, value: bool) -> Self {
        Self {
            use_udp: value,
            ..self
        }
    }

    pub fn multicast(self, value: bool) -> Self {
        Self {
            use_multicast: value,
            ..self
        }
    }

    pub fn build(self) -> Result<MdApi, &'static str> {
        let flow_path = self.flow_path.unwrap().clone();
        let use_udp = self.use_udp;
        let use_multicast = self.use_multicast;
        let use_production_mode = self.use_production_mode;

        // utils::check_and_create_dir(flow_path.as_str())?;
        match self.dynlib {
            Some(dynlib_path) => {
                let mut mdapi = MdApi {
                    api_ptr: null_mut(),
                    spi_ptr: Cell::new(null_mut()),
                    dynlib: None,
                };
                println!("here?");
                unsafe {
                    let lib = libloading::Library::new(dynlib_path).expect("load dynlib error");
                    if cfg!(not(feature = "ctp_v6_7_11")) {
                        type MdApiCreator =
                            unsafe extern "C" fn(*const c_char, bool, bool) -> *mut CThostFtdcMdApi;
                        let create_api: libloading::Symbol<MdApiCreator> = lib
                            .get(MDAPI_CREATE_API_SYMBOL)
                            .expect("get md_create_api symbol error");
                        let cflow_path =
                            CString::new(flow_path.as_bytes()).expect("create cflow path error");
                        mdapi.api_ptr = create_api(cflow_path.as_ptr(), use_udp, use_multicast);
                        mdapi.dynlib = Some(lib);
                    } else {
                        type MdApiCreator = unsafe extern "C" fn(
                            *const c_char,
                            bool,
                            bool,
                            bool,
                        )
                            -> *mut CThostFtdcMdApi;
                        let create_api: libloading::Symbol<MdApiCreator> = lib
                            .get(MDAPI_CREATE_API_SYMBOL)
                            .expect("get md_create_api symbol error");
                        let cflow_path =
                            CString::new(flow_path.as_bytes()).expect("create cflow path error");
                        mdapi.api_ptr = create_api(
                            cflow_path.as_ptr(),
                            use_udp,
                            use_multicast,
                            use_production_mode,
                        );
                    }
                }
                Ok(mdapi)
            }
            None => Err("dynlib null"),
        }
    }
}

impl TraderApi {
    #[cfg(not(feature = "ctp_v6_7_11"))]
    pub fn create_api<P: AsRef<Path>, F: AsRef<Path>>(dynlib_path: P, flow_path: F) -> Self {
        let dynlib =
            unsafe { libloading::Library::new(dynlib_path.as_ref()).expect("failed to open") };
        type TraderApiCreator = unsafe extern "C" fn(*const c_char) -> *mut CThostFtdcTraderApi;
        let create_api: libloading::Symbol<TraderApiCreator> =
            unsafe { dynlib.get(TDAPI_CREATE_API_SYMBOL).expect("failed to get") };
        let cflow_path = CString::new(flow_path.as_ref().to_str().unwrap()).expect("fail to new");
        let api_ptr: *mut CThostFtdcTraderApi = unsafe { create_api(cflow_path.as_ptr()) };

        Self {
            api_ptr: api_ptr,
            spi_ptr: Cell::new(null_mut()),
            dynlib: Some(dynlib),
        }
    }

    #[cfg(feature = "ctp_v6_7_11")]
    pub fn create_api<P: AsRef<Path>, F: AsRef<Path>>(
        dynlib_path: P,
        flow_path: F,
        is_production_mode: bool,
    ) -> Self {
        // CThostFtdcMdApi_CreateFtdcMdApi(flow_path, is_using_udp, is_multicast)
        let dynlib =
            unsafe { libloading::Library::new(dynlib_path.as_ref()).expect("failed to open") };
        type TraderApiCreator =
            unsafe extern "C" fn(*const c_char, bool) -> *mut CThostFtdcTraderApi;
        let create_api: libloading::Symbol<TraderApiCreator> =
            unsafe { dynlib.get(TDAPI_CREATE_API_SYMBOL).expect("failed to get") };
        let cflow_path = CString::new(flow_path.as_ref().to_str().unwrap()).expect("fail to new");
        let api_ptr: *mut CThostFtdcTraderApi =
            unsafe { create_api(cflow_path.as_ptr(), is_production_mode) };

        Self {
            api_ptr: api_ptr,
            spi_ptr: Cell::new(null_mut()),
            dynlib: Some(dynlib),
        }
    }

    pub fn get_api_version(&self) -> String {
        unsafe {
            type TdGetApiVersion = unsafe extern "C" fn() -> *const c_char;
            let get_api_version: libloading::Symbol<TdGetApiVersion> = self
                .dynlib
                .as_ref()
                .unwrap()
                .get(TDAPI_GET_API_VERSION_SYMBOL)
                .unwrap();
            let cstr_ptr = get_api_version();
            let c_str: &CStr = CStr::from_ptr(cstr_ptr);
            c_str.to_string_lossy().to_string()
        }
    }
}

impl Default for TraderApi {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

impl Drop for TraderApi {
    fn drop(&mut self) {
        let spi_ptr = self.spi_ptr.get();
        if !spi_ptr.is_null() {
            unsafe {
                ((*(*self.api_ptr).vtable_).CThostFtdcTraderApi_RegisterSpi)(
                    self.api_ptr,
                    std::ptr::null_mut(),
                );
                let last_spi = Box::from_raw(spi_ptr); // 释放动态分配的内存
                drop(last_spi);
            }
        }
        unsafe {
            if !self.api_ptr.is_null() {
                ((*(*self.api_ptr).vtable_).CThostFtdcTraderApi_Release)(self.api_ptr);
            }
            let dynlib = self.dynlib.take();
            let _ = dynlib.unwrap().close();
        }
    }
}

pub fn get_api_version_symbol<P: AsRef<Path>>(
    dynlib_path: P,
    symbol: &[u8],
) -> Result<String, libloading::Error> {
    let dynlib = unsafe { libloading::Library::new(dynlib_path.as_ref())? };
    unsafe {
        type GetApiVersion = unsafe extern "C" fn() -> *const c_char;
        let get_api_version: libloading::Symbol<GetApiVersion> = dynlib.get(symbol)?;
        let cstr_ptr = get_api_version();
        let c_str: &CStr = CStr::from_ptr(cstr_ptr);
        Ok(c_str.to_string_lossy().to_string())
    }
}

pub fn get_api_version<P: AsRef<Path>>(dynlib_path: P) -> Result<String, libloading::Error> {
    let md_symbol = MDAPI_GET_API_VERSION_SYMBOL;
    let td_symbol = TDAPI_GET_API_VERSION_SYMBOL;

    let dynlib = unsafe { libloading::Library::new(dynlib_path.as_ref())? };

    // 尝试加载 MDAPI 符号
    let _get_api_version = unsafe {
        type GetApiVersion = unsafe extern "C" fn() -> *const c_char;

        if let Ok(symbol) = dynlib.get::<GetApiVersion>(md_symbol) {
            symbol
        } else if let Ok(symbol) = dynlib.get::<GetApiVersion>(td_symbol) {
            symbol
        } else {
            // 如果两个符号都加载失败，返回错误
            return Err(libloading::Error::DlSymUnknown);
        }
    };

    // 调用加载的符号函数并获取结果
    unsafe {
        let cstr_ptr = _get_api_version();
        let c_str: &CStr = CStr::from_ptr(cstr_ptr);
        Ok(c_str.to_string_lossy().to_string())
    }
}
