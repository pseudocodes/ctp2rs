#![allow(unused_variables, unused_macros, unreachable_code)]
use std::env::var;

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use clang::*;

mod codegen;
use codegen::*;

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo::warning={}", format!($($tokens)*))
    }
}
fn get_sdk_path() -> &'static std::path::Path {
    // 基于版本的分支判断

    if cfg!(feature = "mini_v1_6_9") {
        return Path::new("./api/mini/v1.6.9/CTPMini_V1.6.9_linux64_api_20240527/");
    }

    if cfg!(feature = "ctp_v6_7_7") {
        if cfg!(feature = "openctp") {
            panic!("`openctp` feature not supported for `v6_7_7`.");
        }
        if cfg!(target_os = "macos") {
            return Path::new("./api/ctp/v6.7.7/v6.7.7_MacOS_20240716");
        }
        if cfg!(target_os = "linux") {
            return Path::new("./api/ctp/v6.7.7/v6.7.7_20240607_api_traderapi_se_linux64");
        }
    }

    if cfg!(feature = "ctp_v6_7_8") {
        if cfg!(feature = "openctp") {
            panic!("`openctp` feature not supported for `v6_7_8`.");
        }
        if cfg!(target_os = "macos") {
            panic!("`macOS platform` not supported for `v6_7_8`.");
        }
        if cfg!(target_os = "linux") {
            return Path::new("./api/ctp/v6.7.8/v6.7.8_20240918_api_traderapi_se_linux64");
        }
    }

    if cfg!(feature = "ctp_v6_7_2") {
        if cfg!(feature = "openctp") {
            return Path::new("./api/ctp/v6.7.2/v6.7.2_20230913_api_traderapi_se_linux64");
        }
        if cfg!(target_os = "macos") {
            return Path::new("./api/ctp/v6.7.2/v6.7.2_MacOS_20231016");
        }
        if cfg!(target_os = "linux") {
            return Path::new("./api/ctp/v6.7.2/v6.7.2_20230913_api_traderapi_se_linux64");
        }
    }

    panic!("Either 'ctp_v6_7_2' or 'ctp_v6_7_7' feature must be enabled.");
}

fn ensure_dir_exists(path: &PathBuf) -> io::Result<()> {
    if !path.exists() {
        // 如果目录不存在，创建它（包括父目录）
        fs::create_dir_all(path)?;
    }
    Ok(())
}

fn build_dyn() {
    #[cfg(feature = "v1alpha1")]
    let version = "v1alpha1";

    #[cfg(feature = "v1alpha2")]
    let version: &str = "v1alpha2";

    let base_dir = var("CARGO_MANIFEST_DIR").unwrap();
    let source_dir = Path::new(&base_dir).join("src");
    let generate_dir = source_dir.join(version);
    let _ = ensure_dir_exists(&generate_dir);

    let sdk_path = get_sdk_path();
    let include_arg = format!("-I{}", sdk_path.to_string_lossy());
    p!("include: {}", include_arg);

    println!("cargo:rerun-if-changed=src/wrapper.hpp");
    println!("cargo:rerun-if-changed=build.rs");

    clang_sys::load().expect("");
    let clang = Clang::new().unwrap();
    let index = Index::new(&clang, false, false);

    let file_path = Path::new("src/wrapper.hpp");
    let tu = index
        .parser(file_path)
        .arguments(&[include_arg.as_str()])
        .parse()
        .unwrap();

    let out_path = PathBuf::from(var("OUT_DIR").unwrap());
    generate_mduser_wrapper_code(&tu.get_entity(), &out_path);
    generate_trader_wrapper_code(&tu.get_entity(), &out_path);
    generate_stream_wrapper_code(&tu.get_entity(), &out_path);

    let bindings = bindgen::Builder::default()
        .header("src/wrapper.hpp")
        .clang_arg(include_arg)
        .clang_arg("-x")
        .clang_arg("c++")
        .rustified_enum(".*")
        .vtable_generation(true)
        .derive_default(true)
        .derive_debug(true)
        .derive_copy(true)
        .ignore_methods()
        .ignore_functions()
        .derive_hash(false)
        .layout_tests(false)
        .generate_comments(true)
        .generate()
        .expect("Unable to generate bindings");

    let binding_file = out_path.join("bindings.rs");
    bindings
        .write_to_file(&binding_file)
        .expect("Couldn't write bindings!");
}

fn main() {
    build_dyn();
}
