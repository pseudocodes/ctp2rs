use std::{borrow::Cow, ptr};

use encoding_rs::GB18030;

/// 创建目录
pub fn check_make_dir(dir: &str) {
    match std::fs::create_dir_all(dir) {
        Ok(_) => (),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
            } else {
                panic!("create dir {} failed: {}", dir, e);
            }
        }
    }
}
pub fn copy_str_to_i8_array_with_truncation<const N: usize>(
    buffer: &mut [i8; N],
    text: &str,
) -> Result<(), &'static str> {
    if N == 0 {
        return Err("Buffer size is zero, cannot copy string");
    }

    // 获取字符串的字节切片
    let text_bytes = text.as_bytes();

    // 计算要拷贝的长度（保留 1 字节给 `\0`）
    let copy_len = std::cmp::min(text_bytes.len(), N - 1);

    // 使用 unsafe 进行拷贝
    unsafe {
        // 将 buffer 转换为 `*mut u8`，然后通过偏移访问每个元素
        ptr::copy_nonoverlapping(
            text_bytes.as_ptr() as *const i8, // 源指针
            buffer.as_mut_ptr(),              // 目标指针
            copy_len,                         // 拷贝长度
        );
    }

    // 添加终止符
    buffer[copy_len] = 0;

    Ok(())
}

pub fn copy_str_to_i8_array(dst: &mut [i8], src: &str) {
    if dst.is_empty() {
        return;
    }
    let bytes = src.as_bytes();
    let len = usize::min(bytes.len(), dst.len() - 1);

    unsafe {
        ptr::copy_nonoverlapping(bytes.as_ptr(), dst.as_mut_ptr() as *mut u8, len);
    }
    dst[len] = 0;
}

#[macro_export]
macro_rules! print_rsp_info {
    ($p:expr) => {
        if let Some(p) = $p {
            println!(
                "ErrorID[{}] Message[{}]",
                p.ErrorID,
                gb18030_cstr_i8_to_str(&p.ErrorMsg).unwrap().to_string()
            );
        }
    };
}

/// 将 `&[std::os::raw::c_char]` 转换为 `Cow<str>`
/// - 如果输入为 ASCII，返回 `Cow::Borrowed`。
/// - 如果输入为 GB18030，解码后返回 `Cow::Owned`。
/// - 如果解码失败，返回 `Err`。
pub fn gb18030_cstr_i8_to_str<'a>(
    c_chars: &'a [std::os::raw::c_char],
) -> Result<Cow<'a, str>, String> {
    let len = c_chars
        .iter()
        .position(|&c| c == 0)
        .unwrap_or(c_chars.len());
    let bytes = &c_chars[..len];

    let bytes = unsafe { &*(bytes as *const [std::os::raw::c_char] as *const [u8]) };

    if bytes.is_ascii() {
        return std::str::from_utf8(bytes)
            .map(Cow::Borrowed)
            .map_err(|e| format!("Invalid UTF-8: {}", e));
    }

    // 非 ASCII 的情况：使用 GB18030 解码
    let (decoded, _, had_errors) = GB18030.decode(bytes);
    if had_errors {
        return Err("Failed to decode GB18030 string".to_string());
    }
    Ok(Cow::Owned(decoded.into_owned()))
}

pub trait AssignFromString {
    /// 将 `&str` 的内容写入到 `[i8; N]` 数组中
    /// 超出数组大小的部分将被截断，未被覆盖的部分保留原值或清零。
    fn assign_from_str(&mut self, s: &str);
}

impl<const N: usize> AssignFromString for [i8; N] {
    fn assign_from_str(&mut self, s: &str) {
        copy_str_to_i8_array(self, s);
    }
}

impl<const N: usize> AssignFromString for &mut [i8; N] {
    fn assign_from_str(&mut self, s: &str) {
        copy_str_to_i8_array(*self, s);
    }
}

pub trait SetString {
    fn set_str(&mut self, s: &str);
}

impl<const N: usize> SetString for [i8; N] {
    fn set_str(&mut self, s: &str) {
        copy_str_to_i8_array(self, s);
    }
}

impl<const N: usize> SetString for &mut [i8; N] {
    fn set_str(&mut self, s: &str) {
        copy_str_to_i8_array(*self, s);
    }
}

pub trait WrapToString {
    fn to_string(&self) -> String;
}

impl<const N: usize> WrapToString for [i8; N] {
    fn to_string(&self) -> String {
        let str_ = gb18030_cstr_i8_to_str(self);
        str_.unwrap().to_string()
    }
}

impl<const N: usize> WrapToString for &[i8; N] {
    fn to_string(&self) -> String {
        let str_ = gb18030_cstr_i8_to_str(*self);
        str_.unwrap().to_string()
    }
}

#[derive(Debug, Clone)]
pub struct WrapString(pub String); // 包装 String

impl<const N: usize> From<[i8; N]> for WrapString {
    fn from(value: [i8; N]) -> Self {
        let str_ = gb18030_cstr_i8_to_str(&value).expect("failed to decode");
        WrapString(str_.into())
    }
}

impl<const N: usize> From<&[i8; N]> for WrapString {
    fn from(value: &[i8; N]) -> Self {
        let str_ = gb18030_cstr_i8_to_str(value).expect("failed to decode");
        WrapString(str_.into())
    }
}

pub trait WrapFrom<T> {
    fn wrap_from(value: T) -> Self;
}

// 自定义 MyInto trait
pub trait WrapInto<T>: Sized {
    fn wrap_into(self) -> T;
}

// 为 MyFrom 创建对应的 MyInto 实现
impl<T, U> WrapInto<U> for T
where
    U: WrapFrom<T>,
{
    fn wrap_into(self) -> U {
        U::wrap_from(self)
    }
}

impl WrapFrom<&[i8]> for String {
    fn wrap_from(bytes: &[i8]) -> Self {
        let u8_bytes: Vec<u8> = bytes.iter().map(|&b| b as u8).collect();
        String::from_utf8(u8_bytes).expect("Invalid UTF-8 sequence")
    }
}

impl<const N: usize> WrapFrom<[i8; N]> for String {
    fn wrap_from(value: [i8; N]) -> Self {
        let str_ = gb18030_cstr_i8_to_str(&value).expect("failed to decode");
        str_.into()
    }
}

impl<const N: usize> WrapFrom<&[i8; N]> for String {
    fn wrap_from(value: &[i8; N]) -> Self {
        let str_ = gb18030_cstr_i8_to_str(value).expect("failed to decode");
        str_.into()
    }
}
