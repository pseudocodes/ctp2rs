#![allow(unused_variables, unused_macros)]
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
    #[cfg(feature = "ctp_v6_7_2")]
    {
        #[cfg(all(feature = "openctp", target_os = "macos"))]
        return Path::new("./api/v6.7.2/v6.7.2_20230913_api_traderapi_se_linux64");

        #[cfg(target_os = "macos")]
        return Path::new("./api/v6.7.2/v6.7.2_MacOS_20231016");

        #[cfg(target_os = "linux")]
        return Path::new("./api/v6.7.2/v6.7.2_20230913_api_traderapi_se_linux64");
    }

    #[cfg(feature = "ctp_v6_7_7")]
    {
        #[cfg(feature = "openctp")]
        compile_error!("`openctp` feature not support for `v6_7_7`.");
        // return Path::new("../api/v6.7.7/v6.7.7_20240607_api_traderapi_se_linux64/");

        #[cfg(target_os = "macos")]
        return Path::new("./api/v6.7.7/v6.7.7_MacOS_20240716");

        #[cfg(target_os = "linux")]
        return Path::new("./api/v6.7.7/v6.7.7_20240607_api_traderapi_se_linux64");
    }

    // 默认情况下触发编译错误
    #[cfg(not(any(feature = "ctp_v6_7_2", feature = "ctp_v6_7_7")))]
    compile_error!("Either 'ctp_v6_7_2' or 'ctp_v6_7_7' feature must be enabled.");
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

    generate_mduser_wrapper_code(&tu.get_entity(), &generate_dir);
    generate_trader_wrapper_code(&tu.get_entity(), &generate_dir);
    generate_stream_wrapper_code(&tu.get_entity(), &generate_dir);

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

    let binding_file = generate_dir.join("bindings.rs");
    bindings
        .write_to_file(&binding_file)
        .expect("Couldn't write bindings!");
}

fn main() {
    build_dyn();
}
