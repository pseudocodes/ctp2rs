# 生成代码示例

> ⚠️ 此目录不包含任何 `.rs` 文件。所有 Rust 绑定代码由 `build.rs` 在编译期生成，产物位于：
>
> ```
> target/<target-triple>/<profile>/build/ctp2rs-<hash>/out/
> ```
>
> 以下代码块摘自实际构建产物，仅供参考。**不同 CTP 版本和目标平台生成的接口数量、参数签名各有差异。**

---

## 生成文件清单

| 文件 | 内容 |
|------|------|
| `bindings.rs` | bindgen 生成的 FFI 类型（常量、`#[repr(C)]` 结构体、VTable） |
| `mdapi.rs` | `struct MdApi` — 行情 API 封装 |
| `mdspi.rs` | `trait MdSpi: Send` — 行情回调 trait |
| `traderapi.rs` | `struct TraderApi` — 交易 API 封装 |
| `traderspi.rs` | `trait TraderSpi: Send` — 交易回调 trait |
| `error.rs` | `enum CtpError` — CTP 错误码枚举 |
| `mod.rs` | 模块组织（`pub mod error` 等） |

---

## TraderApi 节选（ctp_v6_7_2 + macOS）

macOS 版本的 `ReqUserLogin` 比 Linux/Windows 多出 `length` 和 `system_info` 两个参数，
这是因为 macOS SDK 头文件中 C++ 签名本身就不同：

**C++ 头文件对比：**

```cpp
// Linux / Windows (v6.7.2)
virtual int ReqUserLogin(CThostFtdcReqUserLoginField *pReqUserLoginField, int nRequestID) = 0;

// macOS (v6.7.2)
virtual int ReqUserLogin(CThostFtdcReqUserLoginField *pReqUserLoginField, int nRequestID,
                         TThostFtdcSystemInfoLenType length,
                         TThostFtdcClientSystemInfoType systemInfo) = 0;
```

**生成的 Rust 代码对比：**

```rust
// ── macOS (v6.7.2) ──────────────────────────────────────────
// TThostFtdcSystemInfoLenType 是 int 的 typedef，生成为 i32
// TThostFtdcClientSystemInfoType 是 char[N] 的 typedef，生成为对应数组类型

/// 用户登录请求
pub fn req_user_login(
    &self,
    p_req_user_login_field: &mut CThostFtdcReqUserLoginField,
    n_request_id: i32,
    length: i32,
    system_info: TThostFtdcClientSystemInfoType,
) -> i32 {
    unsafe {
        ((*(*self.api_ptr).vtable_).CThostFtdcTraderApi_ReqUserLogin)(
            self.api_ptr,
            p_req_user_login_field as *mut CThostFtdcReqUserLoginField,
            n_request_id,
            length,
            system_info.as_ptr() as *mut i8,
        )
    }
}
```

```rust
// ── Linux / Windows (v6.7.2, v6.7.11 等) ───────────────────

/// 用户登录请求
pub fn req_user_login(
    &self,
    p_req_user_login_field: &mut CThostFtdcReqUserLoginField,
    n_request_id: i32,
) -> i32 {
    unsafe {
        ((*(*self.api_ptr).vtable_).CThostFtdcTraderApi_ReqUserLogin)(
            self.api_ptr,
            p_req_user_login_field as *mut CThostFtdcReqUserLoginField,
            n_request_id,
        )
    }
}
```

> 这就是为什么不能假设接口签名——同一个方法在不同平台下参数列表可能完全不同。

---

## TraderApi 结构体定义

```rust
#[derive(Debug)]
pub struct TraderApi {
    pub api_ptr: *mut CThostFtdcTraderApi,
    pub spi_ptr: AtomicPtr<CThostFtdcTraderSpiExt>,
    pub dynlib: Option<Library>,
    pub released: AtomicBool,
}
```

---

## TraderSpi trait 节选

```rust
pub trait TraderSpi: Send {
    /// 当客户端与交易后台建立起通信连接时（还未登录前），该方法被调用。
    fn on_front_connected(&mut self) {
    }

    /// 当客户端与交易后台通信连接断开时，该方法被调用。
    fn on_front_disconnected(&mut self, n_reason: i32) {
    }

    /// 客户端认证响应
    fn on_rsp_authenticate(
        &mut self,
        p_rsp_authenticate_field: Option<&CThostFtdcRspAuthenticateField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        n_request_id: i32,
        b_is_last: bool,
    ) {
    }

    /// 登录请求响应
    fn on_rsp_user_login(
        &mut self,
        p_rsp_user_login: Option<&CThostFtdcRspUserLoginField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        n_request_id: i32,
        b_is_last: bool,
    ) {
    }

    /// 报单录入请求响应
    fn on_rsp_order_insert(
        &mut self,
        p_input_order: Option<&CThostFtdcInputOrderField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        n_request_id: i32,
        b_is_last: bool,
    ) {
    }

    /// 报单通知
    fn on_rtn_order(
        &mut self,
        p_order: Option<&CThostFtdcOrderField>,
    ) {
    }

    /// 成交通知
    fn on_rtn_trade(
        &mut self,
        p_trade: Option<&CThostFtdcTradeField>,
    ) {
    }

    // ... 约 100+ 个回调方法，与 TraderApi 请求方法一一对应
}
```

---

## MdApi 节选

```rust
pub struct MdApi {
    pub api_ptr: *mut CThostFtdcMdApi,
    pub spi_ptr: AtomicPtr<CThostFtdcMdSpiExt>,
    pub dynlib: Option<Library>,
    pub released: AtomicBool,
}

impl MdApi {
    pub fn release(&self) { /* ... */ }
    pub fn init(&self) { /* ... */ }
    pub fn join(&self) -> i32 { /* ... */ }
    pub fn get_trading_day(&self) -> String { /* ... */ }
    pub fn register_front(&self, psz_front_address: &str) { /* ... */ }
    pub fn register_spi(&self, p_spi: *mut dyn MdSpi) { /* ... */ }
    pub fn subscribe_market_data(&self, pp_instrument_id: &[impl AsRef<str>]) -> i32 { /* ... */ }
    pub fn unsubscribe_market_data(&self, pp_instrument_id: &[impl AsRef<str>]) -> i32 { /* ... */ }
    pub fn req_user_login(&self, p_req_user_login_field: &mut CThostFtdcReqUserLoginField, n_request_id: i32) -> i32 { /* ... */ }
    pub fn req_user_logout(&self, p_user_logout: &mut CThostFtdcUserLogoutField, n_request_id: i32) -> i32 { /* ... */ }
    // ...
}
```

---

## MdSpi trait 节选

```rust
pub trait MdSpi: Send {
    fn on_front_connected(&mut self) {}
    fn on_front_disconnected(&mut self, n_reason: i32) {}
    fn on_heart_beat_warning(&mut self, n_time_lapse: i32) {}

    fn on_rsp_user_login(
        &mut self,
        p_rsp_user_login: Option<&CThostFtdcRspUserLoginField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        n_request_id: i32,
        b_is_last: bool,
    ) {
    }

    fn on_rsp_sub_market_data(
        &mut self,
        p_specific_instrument: Option<&CThostFtdcSpecificInstrumentField>,
        p_rsp_info: Option<&CThostFtdcRspInfoField>,
        n_request_id: i32,
        b_is_last: bool,
    ) {
    }

    fn on_rtn_depth_market_data(
        &mut self,
        p_depth_market_data: Option<&CThostFtdcDepthMarketDataField>,
    ) {
    }

    // ... 共约 13 个回调方法
}
```

---

## SPI VTable 与 extern "C" 桥接（内部机制）

```rust
// codegen 为每个 SPI 生成 VTable 结构体、静态表和 extern "C" 桥接函数，
// 实现 C++ 虚函数表 → Rust trait 的调用转发。

#[repr(C)]
#[derive(Debug)]
pub struct TraderSpiVTable {
    on_front_connected: unsafe extern "C" fn(spi: *mut CThostFtdcTraderSpiExt),
    on_front_disconnected: unsafe extern "C" fn(spi: *mut CThostFtdcTraderSpiExt, n_reason: std::os::raw::c_int),
    on_rsp_authenticate: unsafe extern "C" fn(spi: *mut CThostFtdcTraderSpiExt, p_rsp_authenticate_field: *const CThostFtdcRspAuthenticateField, p_rsp_info: *const CThostFtdcRspInfoField, n_request_id: std::os::raw::c_int, b_is_last: bool),
    // ... 每个 SPI 回调方法对应一个函数指针
}

static SPI_VTABLE: TraderSpiVTable = TraderSpiVTable {
    on_front_connected: spi_on_front_connected,
    on_front_disconnected: spi_on_front_disconnected,
    on_rsp_authenticate: spi_on_rsp_authenticate,
    // ...
};

#[repr(C)]
pub struct CThostFtdcTraderSpiExt {
    vtable: *const TraderSpiVTable,
    pub spi_ptr: *mut dyn TraderSpi,
}

// 桥接函数示例：将 C FFI 调用转发到 Rust trait 方法
extern "C" fn spi_on_rsp_user_login(
    spi: *mut CThostFtdcTraderSpiExt,
    p_rsp_user_login: *const CThostFtdcRspUserLoginField,
    p_rsp_info: *const CThostFtdcRspInfoField,
    n_request_id: std::os::raw::c_int,
    b_is_last: bool,
) {
    unsafe {
        (*(*spi).spi_ptr).on_rsp_user_login(
            p_rsp_user_login.as_ref(),  // *const T → Option<&T>
            p_rsp_info.as_ref(),
            n_request_id,
            b_is_last,
        )
    }
}
```

---

## CtpError 节选

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CtpError {
    /// CTP:正确 (0)
    None,
    /// CTP:不在已同步状态 (1)
    InvalidDataSyncStatus,
    /// CTP:不合法的登录 (3)
    InvalidLogin,
    /// CTP:找不到该用户 (11)
    UserNotFound,
    /// CTP:找不到合约 (16)
    InstrumentNotFound,
    // ... 共约 200+ 个错误码
    Unknown(i32),
}

impl CtpError {
    pub fn from_code(code: i32) -> Self { /* ... */ }
    pub fn code(&self) -> i32 { /* ... */ }
    pub fn message(&self) -> &'static str { /* ... */ }
}
```
