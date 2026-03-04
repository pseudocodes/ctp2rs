use roxmltree::{Document, ParsingOptions};
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use super::naming::to_rust_enum_name;

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
    let doc = Document::parse_with_options(&xml_content, popts)?;

    let mut output =
        File::create(target_dir.as_ref().join("error.rs")).expect("create `errors.rs` error");

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

    write_header(&mut output)?;
    write_enum_definition(&mut output, &errors)?;
    write_impl(&mut output, &errors)?;
    write_traits(&mut output)?;
    write_from_impls(&mut output)?;

    // 添加模块引用到 mod.rs 文件
    add_error_module_to_mod_rs()?;

    Ok(())
}

/// 添加 error 模块引用到 mod.rs 文件
fn add_error_module_to_mod_rs() -> Result<(), Box<dyn std::error::Error>> {
    let base_dir = std::env::var("OUT_DIR").unwrap();
    let mod_rs_path = Path::new(&base_dir).join("mod.rs");

    let error_module = r#"
pub mod error {
    include!(concat!(env!("OUT_DIR"), "/error.rs"));
}
pub use error::*;
"#;

    if !mod_rs_path.exists() {
        std::fs::write(&mod_rs_path, error_module)?;
        return Ok(());
    }

    let mod_content = std::fs::read_to_string(&mod_rs_path)?;
    if mod_content.contains("pub mod error {") {
        return Ok(());
    }

    let new_content = mod_content + error_module;
    std::fs::write(mod_rs_path, new_content)?;

    Ok(())
}

fn write_header(output: &mut impl Write) -> io::Result<()> {
    writeln!(output, "// 自动生成的代码 - 请勿手动修改")?;
    writeln!(output, "// 由 gen_error.rs 从 error.xml 生成")?;
    writeln!(output)?;
    writeln!(output, "use std::fmt;")?;
    writeln!(output, "use std::error::Error as StdError;")?;
    writeln!(output)?;
    writeln!(output, "/// CTP错误代码和消息，从error.xml转换而来")?;
    writeln!(output, "#[derive(Debug, Clone, PartialEq, Eq)]")?;
    writeln!(output, "pub enum CtpError {{")?;
    Ok(())
}

fn write_enum_definition(output: &mut impl Write, errors: &[(&str, i32, &str)]) -> io::Result<()> {
    writeln!(output, "    /// {} ({})", errors[0].2, errors[0].1)?;
    writeln!(output, "    None,")?;
    writeln!(output)?;

    let mut current_range = 0;
    for (id, code, prompt) in errors.iter().skip(1) {
        let range = match *code {
            1..=100 => 1,
            101..=999 => 2,
            1000..=1999 => 3,
            2000..=2999 => 4,
            3000..=3999 => 5,
            _ => 6,
        };

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

        writeln!(output, "    /// {} ({})", prompt, code)?;
        writeln!(output, "    {},", to_rust_enum_name(id))?;
    }

    writeln!(output)?;
    writeln!(output, "    // 未知错误")?;
    writeln!(output, "    Unknown(i32),")?;
    writeln!(output, "}}")?;
    writeln!(output)?;

    Ok(())
}

fn write_impl(output: &mut impl Write, errors: &[(&str, i32, &str)]) -> io::Result<()> {
    writeln!(output, "impl CtpError {{")?;

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
    writeln!(output)?;

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
    writeln!(output)?;

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
    writeln!(output)?;

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
    writeln!(output)?;
    writeln!(output, "impl StdError for CtpError {{}}")?;
    writeln!(output)?;
    Ok(())
}

fn write_from_impls(output: &mut impl Write) -> io::Result<()> {
    writeln!(output)?;
    writeln!(output, "// 实现从i32转换为CtpError")?;
    writeln!(output, "impl From<i32> for CtpError {{")?;
    writeln!(output, "    fn from(code: i32) -> Self {{")?;
    writeln!(output, "        CtpError::from_code(code)")?;
    writeln!(output, "    }}")?;
    writeln!(output, "}}")?;
    writeln!(output)?;
    Ok(())
}
