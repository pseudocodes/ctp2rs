use std::error::Error;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::Path;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};

use libloading::Library;

use crate::v1alpha1::{CThostFtdcMdApi, CThostFtdcTraderApi, MdApi, TraderApi};

/// CTP 动态库 C++ 导出符号名定义
///
/// 根据目标平台（Linux/macOS/Windows）和 feature（`sopt`、`union`、默认）
/// 选择对应的 mangled 符号名。
mod symbols {
    // GET_API_VERSION 符号
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

    // CREATE 符号 - 优先级: sopt > union > 默认
    #[cfg(not(target_os = "windows"))]
    pub const MDAPI_CREATE_API_SYMBOL: &[u8] = if cfg!(feature = "sopt") {
        b"_ZN8ctp_sopt15CThostFtdcMdApi15CreateFtdcMdApiEPKcbb"
    } else if cfg!(feature = "union") {
        b"_ZN15CThostFtdcMdApi15CreateFtdcMdApiEPKcbbb"
    } else {
        b"_ZN15CThostFtdcMdApi15CreateFtdcMdApiEPKcbb"
    };

    #[cfg(not(target_os = "windows"))]
    pub const TDAPI_CREATE_API_SYMBOL: &[u8] = if cfg!(feature = "sopt") {
        b"_ZN8ctp_sopt19CThostFtdcTraderApi19CreateFtdcTraderApiEPKc"
    } else if cfg!(feature = "union") {
        b"_ZN19CThostFtdcTraderApi19CreateFtdcTraderApiEPKcb"
    } else {
        b"_ZN19CThostFtdcTraderApi19CreateFtdcTraderApiEPKc"
    };

    #[cfg(target_os = "windows")]
    pub const MDAPI_CREATE_API_SYMBOL: &[u8] = if cfg!(feature = "sopt") {
        b"?CreateFtdcMdApi@CThostFtdcMdApi@ctp_sopt@@SAPEAV12@PEBD_N1@Z"
    } else if cfg!(feature = "union") {
        b"?CreateFtdcMdApi@CThostFtdcMdApi@@SAPEAV1@PEBD_N1_N@Z"
    } else {
        b"?CreateFtdcMdApi@CThostFtdcMdApi@@SAPEAV1@PEBD_N1@Z"
    };

    #[cfg(target_os = "windows")]
    pub const TDAPI_CREATE_API_SYMBOL: &[u8] = if cfg!(feature = "sopt") {
        b"?CreateFtdcTraderApi@CThostFtdcTraderApi@ctp_sopt@@SAPEAV12@PEBD@Z"
    } else if cfg!(feature = "union") {
        b"?CreateFtdcTraderApi@CThostFtdcTraderApi@@SAPEAV1@PEBD_N@Z"
    } else {
        b"?CreateFtdcTraderApi@CThostFtdcTraderApi@@SAPEAV1@PEBD@Z"
    };
}
// 重新导出符号
pub use symbols::*;

impl MdApi {
    /// 创建行情 API 实例（非 union 版本）
    ///
    /// # 参数
    /// - `dynlib_path` - CTP 行情动态库路径（如 `thostmduserapi_se.so`）
    /// - `flow_path` - 流文件存储目录，CTP 内部用于断线重连时恢复数据
    /// - `is_using_udp` - 是否使用 UDP 协议接收行情
    /// - `is_multicast` - 是否使用组播模式接收行情
    ///
    /// # Panics
    /// 动态库加载失败或符号未找到时 panic（属于部署配置错误）。
    /// 若需要错误处理，请使用 [`MdApiBuilder`]。
    #[cfg(not(feature = "union"))]
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
            spi_ptr: AtomicPtr::new(null_mut()),
            dynlib: Some(dynlib),
            released: AtomicBool::new(false),
        }
    }

    /// 创建行情 API 实例（union 版本，额外支持 `is_production_mode` 参数）
    ///
    /// # 参数
    /// - `dynlib_path` - CTP 行情动态库路径
    /// - `flow_path` - 流文件存储目录
    /// - `is_using_udp` - 是否使用 UDP 协议
    /// - `is_multicast` - 是否使用组播模式
    /// - `is_production_mode` - 是否为生产环境模式
    ///
    /// # Panics
    /// 动态库加载失败或符号未找到时 panic。
    #[cfg(feature = "union")]
    pub fn create_api<P: AsRef<Path>, F: AsRef<Path>>(
        dynlib_path: P,
        flow_path: F,
        is_using_udp: bool,
        is_multicast: bool,
        is_production_mode: bool,
    ) -> Self {
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
            spi_ptr: AtomicPtr::new(null_mut()),
            dynlib: Some(dynlib),
            released: AtomicBool::new(false),
        }
    }

    /// 获取 CTP 行情 API 版本号
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
        Self {
            api_ptr: std::ptr::null_mut(),
            spi_ptr: AtomicPtr::new(std::ptr::null_mut()),
            dynlib: None,
            released: AtomicBool::new(false),
        }
    }
}

/// Drop 释放顺序：
/// 1. 注销 SPI 回调（RegisterSpi(null)），防止后续回调触发
/// 2. 调用 CTP Release 等待内部线程退出，确保无回调在飞行中
/// 3. 释放 SPI 包装对象内存（此时可安全释放）
/// 4. 卸载动态库
impl Drop for MdApi {
    fn drop(&mut self) {
        let spi_ptr = self.spi_ptr.load(Ordering::Acquire);
        // 1. 注销 SPI 回调
        if !spi_ptr.is_null() && !self.api_ptr.is_null() {
            unsafe {
                ((*(*self.api_ptr).vtable_).CThostFtdcMdApi_RegisterSpi)(
                    self.api_ptr,
                    std::ptr::null_mut(),
                );
            }
        }
        // 2. Release：等待 CTP 内部线程退出
        let already = self.released.swap(true, Ordering::SeqCst);
        if !already && !self.api_ptr.is_null() {
            unsafe {
                ((*(*self.api_ptr).vtable_).CThostFtdcMdApi_Release)(self.api_ptr);
            }
        }
        // 3. 此时内部线程已退出，安全释放 SPI 内存
        if !spi_ptr.is_null() {
            unsafe {
                let _ = Box::from_raw(spi_ptr);
            }
        }
        // 4. 卸载动态库
        if let Some(lib) = self.dynlib.take() {
            let _ = lib.close();
        }
    }
}

/// 行情 API 构建器，使用 Builder 模式创建 [`MdApi`] 实例。
///
/// # 示例
/// ```no_run
/// use ctp2rs::v1alpha1::MdApiBuilder;
///
/// let mdapi = MdApiBuilder::new()
///     .with_dynlib("path/to/thostmduserapi_se.so")
///     .flow_path("./md_")
///     .using_udp(false)
///     .multicast(false)
///     .build()
///     .expect("failed to create MdApi");
/// ```
#[derive(Debug)]
pub struct MdApiBuilder {
    flow_path: Option<String>,
    use_udp: bool,
    use_multicast: bool,
    use_production_mode: bool,
    dynlib: Option<String>,
}

impl Default for MdApiBuilder {
    fn default() -> Self {
        Self {
            flow_path: None,
            use_udp: false,
            use_multicast: false,
            use_production_mode: true, // 默认使用生产模式
            dynlib: None,
        }
    }
}

impl MdApiBuilder {
    /// 创建新的行情 API 构建器，所有参数使用默认值
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置 CTP 行情动态库路径
    pub fn with_dynlib<P: AsRef<Path>>(self, path: P) -> Self {
        Self {
            dynlib: Some(path.as_ref().as_os_str().to_str().unwrap().into()),
            ..self
        }
    }

    /// 设置流文件存储目录，CTP 内部用于断线重连时恢复数据
    pub fn flow_path<P: AsRef<Path>>(self, value: P) -> Self {
        Self {
            flow_path: Some(value.as_ref().as_os_str().to_str().unwrap().into()),
            ..self
        }
    }

    /// 设置是否使用 UDP 协议接收行情，默认 `false`
    pub fn using_udp(self, value: bool) -> Self {
        Self {
            use_udp: value,
            ..self
        }
    }

    /// 设置是否使用组播模式接收行情，默认 `false`
    pub fn multicast(self, value: bool) -> Self {
        Self {
            use_multicast: value,
            ..self
        }
    }

    /// 设置是否为生产环境模式，默认 `true`
    ///
    /// 仅在启用 `union` feature 时生效。
    pub fn production_mode(self, value: bool) -> Self {
        Self {
            use_production_mode: value,
            ..self
        }
    }

    /// 构建 [`MdApi`] 实例
    ///
    /// # 返回
    /// - `Ok(MdApi)` - 成功创建行情 API 实例
    /// - `Err(&str)` - 未设置动态库路径时返回错误
    ///
    /// # Panics
    /// - `flow_path` 未设置时 panic
    /// - 动态库加载失败或符号未找到时 panic
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
                    spi_ptr: AtomicPtr::new(null_mut()),
                    dynlib: None,
                    released: AtomicBool::new(false),
                };
                unsafe {
                    let lib = libloading::Library::new(dynlib_path).expect("load dynlib error");
                    if cfg!(not(feature = "union")) {
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
                        mdapi.dynlib = Some(lib);
                    }
                }
                Ok(mdapi)
            }
            None => Err("dynlib null"),
        }
    }
}

impl TraderApi {
    /// 创建交易 API 实例（非 union 版本）
    ///
    /// # 参数
    /// - `dynlib_path` - CTP 交易动态库路径（如 `thosttraderapi_se.so`）
    /// - `flow_path` - 流文件存储目录
    ///
    /// # Panics
    /// 动态库加载失败或符号未找到时 panic。
    /// 若需要错误处理，请使用 [`TraderApiBuilder`]。
    #[cfg(not(feature = "union"))]
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
            spi_ptr: AtomicPtr::new(null_mut()),
            dynlib: Some(dynlib),
            released: AtomicBool::new(false),
        }
    }

    /// 创建交易 API 实例（union 版本，额外支持 `is_production_mode` 参数）
    ///
    /// # 参数
    /// - `dynlib_path` - CTP 交易动态库路径
    /// - `flow_path` - 流文件存储目录
    /// - `is_production_mode` - 是否为生产环境模式
    ///
    /// # Panics
    /// 动态库加载失败或符号未找到时 panic。
    #[cfg(feature = "union")]
    pub fn create_api<P: AsRef<Path>, F: AsRef<Path>>(
        dynlib_path: P,
        flow_path: F,
        is_production_mode: bool,
    ) -> Self {
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
            spi_ptr: AtomicPtr::new(null_mut()),
            dynlib: Some(dynlib),
            released: AtomicBool::new(false),
        }
    }

    /// 获取 CTP 交易 API 版本号
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
        Self {
            api_ptr: std::ptr::null_mut(),
            spi_ptr: AtomicPtr::new(std::ptr::null_mut()),
            dynlib: None,
            released: AtomicBool::new(false),
        }
    }
}

/// 交易 API 构建器，使用 Builder 模式创建 [`TraderApi`] 实例。
///
/// 相比 [`MdApiBuilder`]，交易 API 不支持 UDP / 组播参数。
///
/// # 示例
/// ```no_run
/// use ctp2rs::v1alpha1::TraderApiBuilder;
///
/// let tdapi = TraderApiBuilder::new()
///     .with_dynlib("path/to/thosttraderapi_se.so")
///     .flow_path("./td_")
///     .build()
///     .expect("failed to create TraderApi");
/// ```
#[derive(Debug)]
pub struct TraderApiBuilder {
    flow_path: Option<String>,
    use_production_mode: bool,
    dynlib: Option<String>,
}

impl Default for TraderApiBuilder {
    fn default() -> Self {
        Self {
            flow_path: None,
            use_production_mode: true, // 默认使用生产模式
            dynlib: None,
        }
    }
}

impl TraderApiBuilder {
    /// 创建新的交易 API 构建器，所有参数使用默认值
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置 CTP 交易动态库路径
    pub fn with_dynlib<P: AsRef<Path>>(self, path: P) -> Self {
        Self {
            dynlib: Some(path.as_ref().as_os_str().to_str().unwrap().into()),
            ..self
        }
    }

    /// 设置流文件存储目录
    pub fn flow_path<P: AsRef<Path>>(self, value: P) -> Self {
        Self {
            flow_path: Some(value.as_ref().as_os_str().to_str().unwrap().into()),
            ..self
        }
    }

    /// 设置是否为生产环境模式，默认 `true`
    ///
    /// 仅在启用 `union` feature 时生效。
    pub fn production_mode(self, value: bool) -> Self {
        Self {
            use_production_mode: value,
            ..self
        }
    }

    /// 构建 [`TraderApi`] 实例
    ///
    /// # 返回
    /// - `Ok(TraderApi)` - 成功创建交易 API 实例
    /// - `Err(&str)` - 未设置动态库路径时返回错误
    ///
    /// # Panics
    /// - `flow_path` 未设置时 panic
    /// - 动态库加载失败或符号未找到时 panic
    pub fn build(self) -> Result<TraderApi, &'static str> {
        let flow_path = self.flow_path.unwrap();
        let use_production_mode = self.use_production_mode;

        match self.dynlib {
            Some(dynlib_path) => {
                let mut tdapi = TraderApi {
                    api_ptr: null_mut(),
                    spi_ptr: AtomicPtr::new(null_mut()),
                    dynlib: None,
                    released: AtomicBool::new(false),
                };
                unsafe {
                    let lib = libloading::Library::new(dynlib_path).expect("load dynlib error");
                    if cfg!(not(feature = "union")) {
                        type TraderApiCreator =
                            unsafe extern "C" fn(*const c_char) -> *mut CThostFtdcTraderApi;
                        let create_api: libloading::Symbol<TraderApiCreator> = lib
                            .get(TDAPI_CREATE_API_SYMBOL)
                            .expect("get td_create_api symbol error");
                        let cflow_path =
                            CString::new(flow_path.as_bytes()).expect("create cflow path error");
                        tdapi.api_ptr = create_api(cflow_path.as_ptr());
                        tdapi.dynlib = Some(lib);
                    } else {
                        type TraderApiCreator =
                            unsafe extern "C" fn(*const c_char, bool) -> *mut CThostFtdcTraderApi;
                        let create_api: libloading::Symbol<TraderApiCreator> = lib
                            .get(TDAPI_CREATE_API_SYMBOL)
                            .expect("get td_create_api symbol error");
                        let cflow_path =
                            CString::new(flow_path.as_bytes()).expect("create cflow path error");
                        tdapi.api_ptr = create_api(cflow_path.as_ptr(), use_production_mode);
                        tdapi.dynlib = Some(lib);
                    }
                }
                Ok(tdapi)
            }
            None => Err("dynlib null"),
        }
    }
}

/// Drop 释放顺序：
/// 1. 注销 SPI 回调（RegisterSpi(null)），防止后续回调触发
/// 2. 调用 CTP Release 等待内部线程退出，确保无回调在飞行中
/// 3. 释放 SPI 包装对象内存（此时可安全释放）
/// 4. 卸载动态库
impl Drop for TraderApi {
    fn drop(&mut self) {
        let spi_ptr = self.spi_ptr.load(Ordering::Acquire);
        // 1. 注销 SPI 回调
        if !spi_ptr.is_null() && !self.api_ptr.is_null() {
            unsafe {
                ((*(*self.api_ptr).vtable_).CThostFtdcTraderApi_RegisterSpi)(
                    self.api_ptr,
                    std::ptr::null_mut(),
                );
            }
        }
        // 2. Release：等待 CTP 内部线程退出
        let already = self.released.swap(true, Ordering::SeqCst);
        if !already && !self.api_ptr.is_null() {
            unsafe {
                ((*(*self.api_ptr).vtable_).CThostFtdcTraderApi_Release)(self.api_ptr);
            }
        }
        // 3. 此时内部线程已退出，安全释放 SPI 内存
        if !spi_ptr.is_null() {
            unsafe {
                let _ = Box::from_raw(spi_ptr);
            }
        }
        // 4. 卸载动态库
        if let Some(lib) = self.dynlib.take() {
            let _ = lib.close();
        }
    }
}

/// 通过指定的符号名从动态库中读取 API 版本号
///
/// # 参数
/// - `dynlib_path` - CTP 动态库路径
/// - `symbol` - 导出符号名（参见 [`MDAPI_GET_API_VERSION_SYMBOL`] / [`TDAPI_GET_API_VERSION_SYMBOL`]）
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

/// 自动探测动态库类型（行情 / 交易）并获取 API 版本号
///
/// 优先尝试行情 API 符号，失败后回退到交易 API 符号。
/// 适用于不确定动态库类型的场景。
///
/// # 参数
/// - `dynlib_path` - CTP 动态库路径
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
