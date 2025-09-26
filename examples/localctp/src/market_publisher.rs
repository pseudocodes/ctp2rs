use std::ffi::{c_char, c_void};

pub struct MarketPublisher {
    raw: *mut c_void,
}
impl MarketPublisher {
    pub fn create(name: &str, max_msg: u32, max_size: u32) -> Result<Self, String> {
        let cname = std::ffi::CString::new(name).map_err(|e| e.to_string())?;
        let raw = unsafe { marketdata_publisher_create(cname.as_ptr(), max_msg, max_size) };
        if raw.is_null() {
            return Err("marketdata_publisher_create returned null".into());
        }
        Ok(Self { raw })
    }
    #[allow(dead_code)]
    #[inline]
    pub fn as_ptr(&self) -> *mut c_void {
        self.raw
    }
    #[allow(dead_code)]
    pub fn is_null(&self) -> bool {
        self.raw.is_null()
    }
    pub fn publish(&self, bytes: &[u8]) -> Result<(), String> {
        if self.raw.is_null() {
            return Err("publisher is null".into());
        }
        unsafe {
            if !marketdata_publisher_publish(self.raw, bytes.as_ptr(), bytes.len()) {
                return Err("failed to publish message".into());
            }
        }
        Ok(())
    }
}
impl Drop for MarketPublisher {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { marketdata_publisher_destroy(self.raw) };
        }
    }
}
// 假设底层 publisher 要么线程安全，要么只在单线程使用（调用方需确保不并发违规）。
unsafe impl Send for MarketPublisher {}
unsafe impl Sync for MarketPublisher {}

extern "C" {
    pub fn marketdata_publisher_create(
        mq_name: *const c_char,
        max_msg: u32,
        max_size: u32,
    ) -> *mut c_void;
    pub fn marketdata_publisher_destroy(publisher: *mut c_void);
    pub fn marketdata_publisher_publish(
        publisher: *mut c_void,
        data: *const u8,
        size: usize,
    ) -> bool;
}

// 转换成 &[u8]
pub fn as_bytes_unsafe<T>(val: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts((val as *const T) as *const u8, std::mem::size_of::<T>()) }
}
