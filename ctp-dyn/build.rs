#![allow(unreachable_code)]
#[cfg(all(feature = "v1alpha1", feature = "v1alpha2"))]
compile_error!("features `v1alpha1` and `v1alpha2` cannot be enabled at the same time");

#[cfg(not(any(feature = "v1alpha1", feature = "v1alpha2")))]
compile_error!("one API layer feature must be enabled: `v1alpha1` or `v1alpha2`");

use std::env;
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
    // Feature 互斥检查：同一时间只能启用一个 CTP 版本
    let version_features: &[(&str, bool)] = &[
        ("ctp_v6_7_2", cfg!(feature = "ctp_v6_7_2")),
        ("ctp_v6_7_7", cfg!(feature = "ctp_v6_7_7")),
        ("ctp_v6_7_11", cfg!(feature = "ctp_v6_7_11")),
        ("ctp_v6_7_13", cfg!(feature = "ctp_v6_7_13")),
        ("mini_v1_7_0", cfg!(feature = "mini_v1_7_0")),
        ("mini_v1_7_5", cfg!(feature = "mini_v1_7_5")),
        ("sopt_v3_7_3", cfg!(feature = "sopt_v3_7_3")),
        ("sopt_v3_7_5", cfg!(feature = "sopt_v3_7_5")),
    ];
    let enabled: Vec<&str> = version_features
        .iter()
        .filter(|(_, enabled)| *enabled)
        .map(|(name, _)| *name)
        .collect();
    if enabled.len() > 1 {
        p!(
            "WARNING: Multiple CTP version features enabled simultaneously: [{}]. \
             This may be caused by Cargo workspace feature unification. \
             The first matched version will be used. \
             For correct behavior, build specific packages with `cargo build -p <package>`.",
            enabled.join(", ")
        );
    }

    // 基于版本的分支判断
    if cfg!(feature = "mini_v1_7_0") {
        if cfg!(target_os = "linux") {
            return Path::new("./api/mini/v1.7.0/CTPIIMini_V1.7.0_linux64_api_20240923/");
        }
        if cfg!(target_os = "windows") {
            return Path::new("./api/mini/v1.7.0/CTPIIMini_V1.7.0_win_api_20240923/win64/");
        }
    }

    if cfg!(feature = "mini_v1_7_5") {
        if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
            return Path::new("./api/mini/v1.7.5/CTPMini_V1.7.5_linux64_api_20260115/");
        }
        if cfg!(target_os = "windows") {
            return Path::new("./api/mini/v1.7.5/CTPMini_V1.7.5_win_api_20260115/win64/");
        }
    }

    if cfg!(feature = "sopt_v3_7_3") {
        if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
            return Path::new("./api/ctpsopt/v3.7.3/v3.7.3_20240910_api_traderapi_linux64_se/");
        }
        if cfg!(target_os = "windows") {
            return Path::new("./api/ctpsopt/v3.7.3/20240910_traderapi64_windows_se/");
        }
    }

    if cfg!(feature = "sopt_v3_7_5") {
        if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
            return Path::new("./api/ctpsopt/v3.7.5/v3.7.5_20251125_api_traderapi_linux64_se/");
        }
        if cfg!(target_os = "windows") {
            return Path::new("./api/ctpsopt/v3.7.5/20251125_traderapi64_windows_se/");
        }
    }

    // ── CTP 主线版本（按优先级排列，非 default 版本优先）──
    // 当 workspace feature unification 导致多个版本同时启用时，
    // 非 default 版本（由 example 显式指定）应优先于 default 版本。
    // 当前 default = ctp_v6_7_13，因此放在最后匹配。

    if cfg!(feature = "ctp_v6_7_2") {
        if cfg!(target_os = "windows") {
            return Path::new("./api/ctp/v6.7.2/v6.7.2_20230913_api_traderapi64_se_windows");
        }
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

    if cfg!(feature = "ctp_v6_7_7") {
        if cfg!(target_os = "macos") {
            if cfg!(feature = "openctp") {
                return Path::new("./api/ctp/v6.7.7/v6.7.7_20240607_api_traderapi_se_linux64");
            }
            return Path::new("./api/ctp/v6.7.7/v6.7.7_MacOS_20240716");
        }
        if cfg!(target_os = "linux") {
            return Path::new("./api/ctp/v6.7.7/v6.7.7_20240607_api_traderapi_se_linux64");
        }
        if cfg!(target_os = "windows") {
            return Path::new("./api/ctp/v6.7.7/v6.7.7_20240607_traderapi64_se_windows/");
        }
    }

    if cfg!(feature = "ctp_v6_7_11") {
        if cfg!(target_os = "macos") {
            panic!("`macos` feature not supported for `v6_7_11`.");
        }
        if cfg!(target_os = "linux") {
            return Path::new("./api/ctp/v6.7.11/v6.7.11_20250617_api_traderapi_se_linux64/");
        }
        if cfg!(target_os = "windows") {
            return Path::new("./api/ctp/v6.7.11/v6.7.11_20250617_traderapi64_se_windows/");
        }
    }

    // default 版本（ctp_v6_7_13）放最后，仅在没有其他显式版本匹配时生效
    if cfg!(feature = "ctp_v6_7_13") {
        if cfg!(target_os = "macos") {
            return Path::new("./api/ctp/v6.7.13/v6.7.13_MacOS_20260729");
        }
        if cfg!(target_os = "linux") {
            return Path::new("./api/ctp/v6.7.13/v6.7.13_20260225_api_traderapi_se_linux64");
        }
        if cfg!(target_os = "windows") {
            return Path::new("./api/ctp/v6.7.13/v6.7.13_20260225_winApi");
        }
    }

    // 没有任何版本 feature 启用时（default-features = false 且未选版本），
    // fallback 到 v6.7.7：其 create 接口为非 union 签名，与 builder.rs 默认符号兼容
    if cfg!(target_os = "macos") {
        if cfg!(feature = "openctp") {
            return Path::new("./api/ctp/v6.7.7/v6.7.7_20240607_api_traderapi_se_linux64");
        }
        return Path::new("./api/ctp/v6.7.7/v6.7.7_MacOS_20240716");
    }
    if cfg!(target_os = "linux") {
        return Path::new("./api/ctp/v6.7.7/v6.7.7_20240607_api_traderapi_se_linux64");
    }
    if cfg!(target_os = "windows") {
        return Path::new("./api/ctp/v6.7.7/v6.7.7_20240607_traderapi64_se_windows/");
    }

    panic!("Unsupported target platform.");
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

    let sdk_path = if env::var("CTP_API_INCLUDE_DIR").is_ok() {
        println!("cargo:rerun-if-env-changed=CTP_API_INCLUDE_DIR");
        get_ctp_include_path().unwrap_or_else(|e| {
            p!(
                "fetch `CTP_API_INCLUDE_DIR` environment variable failed {}. Using default path.",
                e
            );
            get_sdk_path().to_path_buf()
        })
    } else {
        get_sdk_path().to_path_buf()
    };
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
    let mod_rs_path = Path::new(&out_path).join("mod.rs");
    if !mod_rs_path.exists() {
        fs::write(
            mod_rs_path,
            "// This file is generated by build.rs. Do not edit.\n",
        )
        .unwrap();
    }
    generate_mduser_wrapper_code(&tu.get_entity(), &out_path, &CodegenConfig::default());
    generate_trader_wrapper_code(&tu.get_entity(), &out_path, &CodegenConfig::default());

    if let Err(_err) = generate_errors_wrapper_code(sdk_path.join("error.xml"), &out_path) {
        // panic!("Failed to generate errors wrapper code: {}", err);
    }

    let bindings = bindgen::Builder::default()
        .header("src/wrapper.hpp")
        .clang_arg(include_arg)
        .clang_arg("-x")
        .clang_arg("c++")
        .rustified_enum(".*")
        .vtable_generation(true)
        .disable_name_namespacing()
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
