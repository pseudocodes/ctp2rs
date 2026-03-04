pub mod errors;
pub mod generator;
pub mod naming;
pub mod parser;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use clang::Entity;

pub use errors::generate_errors_wrapper_code;
use parser::ApiKind;
pub use parser::CodegenConfig;

/// 生成 Trader API/SPI 文件
pub fn generate_trader_wrapper_code<P: AsRef<Path>>(
    entity: &Entity,
    out_dir: P,
    config: &CodegenConfig,
) {
    let kind = ApiKind::Trader;

    let api_methods = parser::extract_methods(entity, kind.api_class(), config);
    let spi_methods = parser::extract_methods(entity, kind.spi_class(), config);

    let api_code = generator::generate_api_file(kind, &api_methods, config);
    let spi_code = generator::generate_spi_file(kind, &spi_methods, config);

    fs::write(out_dir.as_ref().join("traderapi.rs"), api_code).expect("写入 traderapi.rs 失败");
    fs::write(out_dir.as_ref().join("traderspi.rs"), spi_code).expect("写入 traderspi.rs 失败");
}

/// 生成 MdUser API/SPI 文件
pub fn generate_mduser_wrapper_code<P: AsRef<Path>>(
    entity: &Entity,
    out_dir: P,
    config: &CodegenConfig,
) {
    let kind = ApiKind::Md;

    let api_methods = parser::extract_methods(entity, kind.api_class(), config);
    let spi_methods = parser::extract_methods(entity, kind.spi_class(), config);

    let api_code = generator::generate_api_file(kind, &api_methods, config);
    let spi_code = generator::generate_spi_file(kind, &spi_methods, config);

    fs::write(out_dir.as_ref().join("mdapi.rs"), api_code).expect("写入 mdapi.rs 失败");
    fs::write(out_dir.as_ref().join("mdspi.rs"), spi_code).expect("写入 mdspi.rs 失败");
}

pub fn get_ctp_include_path() -> Result<PathBuf, String> {
    let path_str = env::var("CTP_API_INCLUDE_DIR")
        .map_err(|_| "Environment variable `CTP_API_INCLUDE_DIR` not found".to_string())?;

    if path_str.trim().is_empty() {
        return Err("Path cannot be empty".to_string());
    }

    let path = Path::new(&path_str);

    let absolute_path = env::current_dir()
        .map_err(|_| "Failed to get current directory".to_string())?
        .join(if path.is_absolute() {
            path
        } else {
            Path::new(&path_str)
        });

    let canonical_path = absolute_path
        .canonicalize()
        .map_err(|_| format!("Path does not exist: {}", absolute_path.display()))?;

    if !canonical_path.exists() {
        return Err(format!("Path does not exist: {}", canonical_path.display()));
    }

    Ok(canonical_path)
}
