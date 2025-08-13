use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use std::thread;

mod apimd;
use apimd::*;

mod apitd;
use apitd::*;

#[derive(Debug, Clone, ValueEnum)]
pub enum Environment {
    /// 仿真环境
    Sim,
    /// 7x24小时模拟环境
    Tts,
}

#[derive(Debug, Clone)]
pub struct CtpAccountConfig {
    // mdapi 配置
    pub md_user_id: String,
    pub md_front_address: String,
    pub md_dynlib_path: PathBuf,

    // tdapi 配置
    pub td_user_id: String,
    pub td_password: String,
    pub td_app_id: String,
    pub td_auth_code: String,
    pub td_front_address: String,
    pub td_dynlib_path: PathBuf,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 选择运行环境
    #[arg(short, long, value_enum, default_value_t = Environment::Tts)]
    environment: Environment,

    /// 用户ID
    #[arg(short, long, env = "OPENCTP_USER_ID")]
    user_id: String,

    /// 密码
    #[arg(short, long, env = "OPENCTP_PASS")]
    password: String,
}

fn create_config_for_environment(
    env: Environment,
    user_id: String,
    password: String,
) -> CtpAccountConfig {
    let base_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let base_path = std::path::Path::new(&base_dir);

    match env {
        Environment::Sim => {
            // 仿真环境配置
            #[cfg(target_os = "macos")]
            let md_dynlib_path = base_path.join("../../../ctp-dyn/api/ctp/v6.7.2/v6.7.2_MacOS_20231016/thostmduserapi_se.framework/thostmduserapi_se");
            #[cfg(all(target_os = "linux", not(feature = "ctp_v6_7_11")))]
            let md_dynlib_path = base_path.join("../../../ctp-dyn/api/ctp/v6.7.2/v6.7.2_20230913_api_traderapi_se_linux64/thostmduserapi_se.so");
            #[cfg(all(target_os = "linux", feature = "ctp_v6_7_11"))]
            let md_dynlib_path = base_path.join("../../../ctp-dyn/api/ctp/v6.7.11/v6.7.11_20250617_api_traderapi_se_linux64/thostmduserapi_se.so");

            #[cfg(all(target_os = "windows", not(feature = "ctp_v6_7_11")))]
            let md_dynlib_path = base_path.join("../../../ctp-dyn/api/ctp/v6.7.2/v6.7.2_20230913_api_traderapi_se_win64/thostmduserapi_se.dll");
            #[cfg(all(target_os = "windows", feature = "ctp_v6_7_11"))]
            let md_dynlib_path = base_path.join("../../../ctp-dyn/api/ctp/v6.7.11/v6.7.11_20250617_traderapi64_se_windows/thostmduserapi_se.dll");

            #[cfg(target_os = "macos")]
            let td_dynlib_path = base_path.join("tts/v6_7_2/mac_arm64/thosttraderapi_se.dylib");
            #[cfg(target_os = "linux")]
            let td_dynlib_path = base_path.join("tts/v6_7_2/lin64/thosttraderapi_se.so");
            #[cfg(target_os = "windows")]
            let td_dynlib_path = base_path.join("tts/v6_7_2/win64/thosttraderapi_se.dll");

            CtpAccountConfig {
                md_user_id: user_id.clone(),
                md_front_address: "tcp://182.254.243.31:30011".to_string(), // SimNow 仿真环境
                md_dynlib_path,
                td_user_id: user_id,
                td_password: password,
                td_app_id: "simnow_client_test".to_string(),
                td_auth_code: "0000000000000000".to_string(),
                td_front_address: "tcp://121.37.90.193:20002".to_string(), // OPENCTP 仿真环境
                td_dynlib_path,
            }
        }
        Environment::Tts => {
            // 7x24小时模拟环境配置
            #[cfg(target_os = "macos")]
            let md_dynlib_path = base_path.join("tts/v6_7_2/mac_arm64/thostmduserapi_se.dylib");
            #[cfg(target_os = "linux")]
            let md_dynlib_path = base_path.join("tts/v6_7_2/lin64/thostmduserapi_se.so");
            #[cfg(target_os = "windows")]
            let md_dynlib_path = base_path.join("tts/v6_7_2/win64/thostmduserapi_se.dll");

            #[cfg(target_os = "macos")]
            let td_dynlib_path = base_path.join("tts/v6_7_2/mac_arm64/thosttraderapi_se.dylib");
            #[cfg(target_os = "linux")]
            let td_dynlib_path = base_path.join("tts/v6_7_2/lin64/thosttraderapi_se.so");
            #[cfg(target_os = "windows")]
            let td_dynlib_path = base_path.join("tts/v6_7_2/win64/thosttraderapi_se.dll");

            CtpAccountConfig {
                md_user_id: user_id.clone(),
                md_front_address: "tcp://121.37.80.177:20004".to_string(), // TTS 7x24 环境
                md_dynlib_path,
                td_user_id: user_id,
                td_password: password,
                td_app_id: "simnow_client_test".to_string(),
                td_auth_code: "0000000000000000".to_string(),
                td_front_address: "tcp://121.37.80.177:20002".to_string(), // TTS 7x24 环境
                td_dynlib_path,
            }
        }
    }
}

fn run_two_loops(config: CtpAccountConfig) {
    let config_clone = config.clone();
    let handle1 = thread::spawn(move || run_td(config)); // 启动第 td 线程
    let handle2 = thread::spawn(move || run_md(config_clone)); // 启动第 md 线程

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Both loops are finished. Main thread exiting.");
}

fn main() {
    let args = Args::parse();

    println!("Running with environment: {:?}", args.environment);

    let config = create_config_for_environment(args.environment, args.user_id, args.password);

    run_two_loops(config);
}
