use encoding_rs::{Encoding, GB18030, UTF_8};
use roxmltree::{Document, Node, ParsingOptions};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

pub fn generate_errors_wrapper_code<X, P>(
    source_file: X,
    target_dir: P,
) -> Result<(), Box<dyn std::error::Error>>
where
    X: AsRef<Path>,
    P: AsRef<Path>,
{
    let xml_content = std::fs::read_to_string(source_file)?;
    let popts = ParsingOptions {
        allow_dtd: true,
        nodes_limit: u32::MAX,
    };
    // 解析XML
    let doc = Document::parse_with_options(&xml_content, popts)?;

    // 创建输出文件
    let mut output =
        File::create(target_dir.as_ref().join("error.rs")).expect("create `errors.rs` error");

    // 写入文件头部
    write_header(&mut output)?;

    // 收集所有错误定义
    let mut errors = Vec::new();
    for node in doc.descendants().filter(|n| n.has_tag_name("error")) {
        if let (Some(id), Some(value), Some(prompt)) = (
            node.attribute("id"),
            node.attribute("value"),
            node.attribute("prompt"),
        ) {
            errors.push((id, value.parse::<i32>().unwrap_or(0), prompt));
        }
    }

    // 按错误码排序
    errors.sort_by_key(|e| e.1);

    // 生成错误枚举
    write_enum_definition(&mut output, &errors)?;

    // 生成实现方法
    write_impl(&mut output, &errors)?;

    // 生成Display和Error trait实现
    write_traits(&mut output)?;

    // 生成From trait实现
    write_from_impls(&mut output)?;

    // 添加模块引用到 mod.rs 文件
    add_error_module_to_mod_rs()?;

    Ok(())
}

/// 添加 error 模块引用到 mod.rs 文件
fn add_error_module_to_mod_rs() -> Result<(), Box<dyn std::error::Error>> {
    // 获取项目根目录
    let base_dir = std::env::var("OUT_DIR").unwrap();

    // 构建 mod.rs 文件路径
    let mod_rs_path = Path::new(&base_dir).join("mod.rs");

    // 准备错误模块的内容
    let error_module = r#"
pub mod error {
    include!(concat!(env!("OUT_DIR"), "/error.rs"));
}
pub use error::*;
"#;

    // 如果文件不存在，创建新文件并写入内容
    if !mod_rs_path.exists() {
        std::fs::write(&mod_rs_path, error_module)?;
        // println!("cargo::warning=Created mod.rs with error module");
        return Ok(());
    }

    // 如果文件存在，检查是否已包含 error 模块
    let mod_content = std::fs::read_to_string(&mod_rs_path)?;
    if mod_content.contains("pub mod error {") {
        // println!("cargo::warning=Error module already exists in mod.rs");
        return Ok(());
    }

    // 追加错误模块到文件末尾
    let new_content = mod_content + error_module;
    std::fs::write(mod_rs_path, new_content)?;

    // println!("cargo::warning=Added error module to mod.rs");
    Ok(())
}

/// 读取文件并自动检测编码
fn read_file_with_encoding_detection(
    path: &str,
    check_xml_declaration: bool,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    // 首先读取文件的原始字节
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    // 根据参数决定是否尝试从XML声明中提取编码
    let (encoding, encoding_name) = if check_xml_declaration {
        // 尝试从XML声明中提取编码
        let declared_encoding = extract_encoding_from_xml_declaration(&bytes);

        // 根据声明的编码或自动检测选择编码器
        match declared_encoding {
            Some(enc) if enc.eq_ignore_ascii_case("gb2312") => (GB18030, "gb2312".to_string()),
            Some(enc) => {
                // 尝试根据声明的编码名称获取编码器
                match Encoding::for_label(enc.as_bytes()) {
                    Some(e) => (e, enc),
                    None => detect_encoding(&bytes),
                }
            }
            None => detect_encoding(&bytes),
        }
    } else {
        // 不检查XML声明，直接使用编码检测
        detect_encoding(&bytes)
    };

    // 解码文件内容
    let (cow, _used_encoding, _had_errors) = encoding.decode(&bytes);

    Ok((cow.into_owned(), encoding_name))
}

/// 从XML声明中提取编码信息
fn extract_encoding_from_xml_declaration(bytes: &[u8]) -> Option<String> {
    // 先尝试用UTF-8解码前100个字节，这通常足够包含XML声明
    let (declaration, _, _) = UTF_8.decode(&bytes[..std::cmp::min(100, bytes.len())]);
    let declaration = declaration.to_string();

    // 查找编码声明
    if let Some(xml_decl_end) = declaration.find("?>") {
        let xml_decl = &declaration[..xml_decl_end];
        if let Some(encoding_start) = xml_decl.find("encoding=") {
            let encoding_part = &xml_decl[encoding_start + 9..]; // 9 是 "encoding="的长度
            if let Some(quote_char) = encoding_part.chars().next() {
                if quote_char == '"' || quote_char == '\'' {
                    if let Some(end_pos) = encoding_part[1..].find(quote_char) {
                        return Some(encoding_part[1..=end_pos].to_string());
                    }
                }
            }
        }
    }

    None
}

/// 自动检测文件编码
fn detect_encoding(bytes: &[u8]) -> (&'static Encoding, String) {
    // 检查BOM标记
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return (UTF_8, "utf-8 with BOM".to_string());
    }

    // 检查是否可能是GB2312/GB18030
    // 简单启发式：如果有大量字节在0x80-0xFF范围内，且没有无效的GB18030序列，可能是GB18030
    let high_byte_count = bytes.iter().filter(|&&b| b >= 0x80).count();
    if high_byte_count > 0 && high_byte_count as f32 / bytes.len() as f32 > 0.1 {
        // 尝试用GB18030解码，看是否有错误
        let (_, _, had_errors) = GB18030.decode(bytes);
        if !had_errors {
            return (GB18030, "gb18030/gb2312".to_string());
        }
    }

    // 默认使用UTF-8
    (UTF_8, "utf-8".to_string())
}

fn write_header(output: &mut impl Write) -> io::Result<()> {
    writeln!(output, "// 自动生成的代码 - 请勿手动修改")?;
    writeln!(output, "// 由 gen_error.rs 从 error.xml 生成")?;
    writeln!(output, "")?;
    writeln!(output, "use std::fmt;")?;
    writeln!(output, "use std::error::Error as StdError;")?;
    writeln!(output, "")?;
    writeln!(output, "/// CTP错误代码和消息，从error.xml转换而来")?;
    writeln!(output, "#[derive(Debug, Clone, PartialEq, Eq)]")?;
    writeln!(output, "pub enum CtpError {{")?;
    Ok(())
}

fn write_enum_definition(output: &mut impl Write, errors: &[(&str, i32, &str)]) -> io::Result<()> {
    // 写入成功情况
    writeln!(output, "    /// {} ({})", errors[0].2, errors[0].1)?;
    writeln!(output, "    None,")?;
    writeln!(output, "")?;

    // 按错误码范围分组写入其他错误
    let mut current_range = 0;
    for (id, code, prompt) in errors.iter().skip(1) {
        // 确定错误范围
        let range = match *code {
            1..=100 => 1,
            101..=999 => 2,
            1000..=1999 => 3,
            2000..=2999 => 4,
            3000..=3999 => 5,
            _ => 6,
        };

        // 如果进入新范围，添加注释
        if range != current_range {
            current_range = range;
            match range {
                1 => writeln!(output, "    // 一般错误 (1-100)")?,
                2 => writeln!(output, "    // 灾备系统错误 (101-999)")?,
                3 => writeln!(output, "    // 转账系统错误 (1000-1999)")?,
                4 => writeln!(output, "    // 附加转账错误 (2000-2999)")?,
                5 => writeln!(output, "    // 外汇系统错误 (3000-3999)")?,
                _ => writeln!(output, "    // 其他错误 ({}+)", code)?,
            }
        }

        // 写入错误枚举项
        writeln!(output, "    /// {} ({})", prompt, code)?;
        writeln!(output, "    {},", to_rust_enum_name(id))?;
    }

    // 添加未知错误类型
    writeln!(output, "")?;
    writeln!(output, "    // 未知错误")?;
    writeln!(output, "    Unknown(i32),")?;
    writeln!(output, "}}")?;
    writeln!(output, "")?;

    Ok(())
}

fn write_impl(output: &mut impl Write, errors: &[(&str, i32, &str)]) -> io::Result<()> {
    writeln!(output, "impl CtpError {{")?;

    // from_code 方法
    writeln!(output, "    /// 从错误码转换为CtpError枚举")?;
    writeln!(output, "    pub fn from_code(code: i32) -> Self {{")?;
    writeln!(output, "        match code {{")?;

    for (id, code, _) in errors {
        writeln!(
            output,
            "            {} => CtpError::{},",
            code,
            to_rust_enum_name(id)
        )?;
    }

    writeln!(
        output,
        "            unknown_code => CtpError::Unknown(unknown_code),"
    )?;
    writeln!(output, "        }}")?;
    writeln!(output, "    }}")?;
    writeln!(output, "")?;

    // code 方法
    writeln!(output, "    /// 获取错误码")?;
    writeln!(output, "    pub fn code(&self) -> i32 {{")?;
    writeln!(output, "        match self {{")?;

    for (id, code, _) in errors {
        writeln!(
            output,
            "            CtpError::{} => {},",
            to_rust_enum_name(id),
            code
        )?;
    }

    writeln!(output, "            CtpError::Unknown(code) => *code,")?;
    writeln!(output, "        }}")?;
    writeln!(output, "    }}")?;
    writeln!(output, "")?;

    // message 方法
    writeln!(output, "    /// 获取错误消息")?;
    writeln!(output, "    pub fn message(&self) -> &'static str {{")?;
    writeln!(output, "        match self {{")?;

    for (id, _, prompt) in errors {
        writeln!(
            output,
            "            CtpError::{} => \"{}\",",
            to_rust_enum_name(id),
            prompt
        )?;
    }

    writeln!(
        output,
        "            CtpError::Unknown(_) => \"CTP:未知错误\","
    )?;
    writeln!(output, "        }}")?;
    writeln!(output, "    }}")?;
    writeln!(output, "}}")?;
    writeln!(output, "")?;

    Ok(())
}

fn write_traits(output: &mut impl Write) -> io::Result<()> {
    writeln!(output, "impl fmt::Display for CtpError {{")?;
    writeln!(
        output,
        "    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{"
    )?;
    writeln!(
        output,
        "        write!(f, \"{{}} ({{}})\", self.message(), self.code())"
    )?;
    writeln!(output, "    }}")?;
    writeln!(output, "}}")?;
    writeln!(output, "")?;
    writeln!(output, "impl StdError for CtpError {{}}")?;
    writeln!(output, "")?;
    Ok(())
}

/// 生成From trait的实现
fn write_from_impls(output: &mut impl Write) -> io::Result<()> {
    // 实现从i32转换为CtpError
    writeln!(output, "")?;
    writeln!(output, "// 实现从i32转换为CtpError")?;
    writeln!(output, "impl From<i32> for CtpError {{")?;
    writeln!(output, "    fn from(code: i32) -> Self {{")?;
    writeln!(output, "        CtpError::from_code(code)")?;
    writeln!(output, "    }}")?;
    writeln!(output, "}}")?;
    writeln!(output, "")?;

    Ok(())
}

/// 将XML中的错误ID转换为Rust枚举名称
fn to_rust_enum_name(id: &str) -> String {
    // 特殊情况处理
    if id == "NONE" {
        return "None".to_string();
    }

    let mut result = String::new();
    let mut capitalize_next = true;

    for c in id.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c.to_ascii_lowercase());
        }
    }

    result
}
