#![allow(unused_variables, unused_macros)]
#![allow(unused_imports, unused_parens)]
#![allow(dead_code)]
pub mod errors;
pub mod parser;
pub mod setting;
pub mod stream;

use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use clang::Entity;
pub use errors::*;
pub use parser::*;
pub use setting::*;
pub use stream::*;

pub fn generate_trader_wrapper_code<P: AsRef<Path>>(e: &Entity, target_dir: P) {
    let mut cfg = Config::for_api();
    cfg.source_class_name = "CThostFtdcTraderApi".to_string();
    cfg.generate_trait_name = "TraderApi".to_string();
    cfg.wrap_spi_trait = "TraderSpi".to_string();

    let mut ctx = setting::Context::for_api(cfg);

    let api_code = prepare_api_wrapper_code(&mut ctx, e);

    let mut cfg = setting::Config::for_spi();
    cfg.source_class_name = "CThostFtdcTraderSpi".to_string();
    cfg.generate_trait_name = "TraderSpi".to_string();
    cfg.wrap_spi_trait = "TraderSpi".to_string();
    let mut ctx = setting::Context::for_spi(cfg);

    let spi_code = prepare_spi_wrapper_code(&mut ctx, e);
    let mut file =
        File::create(target_dir.as_ref().join("traderapi.rs")).expect("create file error");
    file.write_all(api_code.as_bytes())
        .expect("write code failed!");

    let mut file =
        File::create(target_dir.as_ref().join("traderspi.rs")).expect("create file error");
    file.write_all(spi_code.as_bytes())
        .expect("write code failed!");
}

pub fn generate_mduser_wrapper_code<P: AsRef<Path>>(e: &Entity, target_dir: P) {
    let mut cfg = Config::for_api();
    cfg.source_class_name = "CThostFtdcMdApi".to_string();
    cfg.generate_trait_name = "MdApi".to_string();
    cfg.wrap_spi_trait = "MdSpi".to_string();

    let mut ctx = setting::Context::for_api(cfg);

    let api_code = prepare_api_wrapper_code(&mut ctx, e);

    let mut cfg = setting::Config::for_spi();
    cfg.source_class_name = "CThostFtdcMdSpi".to_string();
    cfg.generate_trait_name = "MdSpi".to_string();
    cfg.wrap_spi_trait = "MdSpi".to_string();
    let mut ctx = setting::Context::for_spi(cfg);

    let spi_code = prepare_spi_wrapper_code(&mut ctx, e);
    let mut file = File::create(target_dir.as_ref().join("mdapi.rs")).expect("create file error");
    file.write_all(api_code.as_bytes())
        .expect("write code failed!");

    let mut file = File::create(target_dir.as_ref().join("mdspi.rs")).expect("create file error");
    file.write_all(spi_code.as_bytes())
        .expect("write code failed!");
}

pub fn get_ctp_include_path() -> Result<PathBuf, String> {
    // 从环境变量读取路径
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
