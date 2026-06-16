// ─── IR 数据结构 ─────────────────────────────────────────────
//
// 所有从 C++ AST 提取的信息都表达为以下数据结构，
// 不包含任何代码生成逻辑。generator.rs 只消费这些结构。

use clang::{Entity, EntityKind, EntityVisitResult, TypeKind};
use inflector::Inflector;

/// 结构体指针在生成代码中的表示风格
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum PointerStyle {
    /// `*const T` / `*mut T` — 原始指针
    RawConst,
    RawMut,
    /// `&mut T` — 可变引用（API 端默认）
    MutRef,
    /// `&T` — 不可变引用
    Ref,
    /// `Option<&T>` — 可选引用（SPI trait 端默认）
    OptionRef,
}

/// codegen 配置，控制代码生成行为
///
/// 通过修改此处的默认值即可切换命名风格等选项，
/// 无需引入 Cargo feature flag。
#[derive(Clone, Debug)]
pub struct CodegenConfig {
    /// 方法名是否转为 snake_case（true: req_user_login, false: ReqUserLogin）
    pub method_to_snake: bool,
    /// 参数名是否转为 snake_case（true: p_rsp_info, false: pRspInfo）
    pub param_to_snake: bool,
    /// API 端结构体指针参数的风格（默认 MutRef → `&mut T`）
    pub api_pointer_style: PointerStyle,
    /// SPI trait 端结构体指针参数的风格（默认 OptionRef → `Option<&T>`）
    pub spi_pointer_style: PointerStyle,
}

impl Default for CodegenConfig {
    fn default() -> Self {
        Self {
            method_to_snake: true,
            param_to_snake: true,
            api_pointer_style: PointerStyle::MutRef,
            spi_pointer_style: PointerStyle::OptionRef,
        }
    }
}

/// API 类型：行情 or 交易
#[derive(Clone, Copy, Debug)]
pub enum ApiKind {
    Md,
    Trader,
}

impl ApiKind {
    /// C++ API 类名
    pub fn api_class(&self) -> &'static str {
        match self {
            ApiKind::Md => "CThostFtdcMdApi",
            ApiKind::Trader => "CThostFtdcTraderApi",
        }
    }
    /// C++ SPI 类名
    pub fn spi_class(&self) -> &'static str {
        match self {
            ApiKind::Md => "CThostFtdcMdSpi",
            ApiKind::Trader => "CThostFtdcTraderSpi",
        }
    }
    /// 生成的 Rust API struct 名
    pub fn api_name(&self) -> &'static str {
        match self {
            ApiKind::Md => "MdApi",
            ApiKind::Trader => "TraderApi",
        }
    }
    /// 生成的 Rust SPI trait 名
    pub fn spi_name(&self) -> &'static str {
        match self {
            ApiKind::Md => "MdSpi",
            ApiKind::Trader => "TraderSpi",
        }
    }
    /// SPI 模块文件名（用于 use 声明）
    pub fn spi_mod(&self) -> &'static str {
        match self {
            ApiKind::Md => "mdspi",
            ApiKind::Trader => "traderspi",
        }
    }
}

/// 从 AST 提取的单个方法信息
#[derive(Clone, Debug)]
pub struct MethodInfo {
    /// C++ 原始方法名，如 "ReqUserLogin"
    pub cpp_name: String,
    /// Rust snake_case 方法名，如 "req_user_login"
    pub rust_name: String,
    /// C++ 文档注释（原样保留）
    pub comment: String,
    /// 参数列表
    pub params: Vec<ParamInfo>,
    /// 返回类型
    pub return_type: ReturnType,
    /// 是否为静态方法（如 CreateFtdcTraderApi / GetApiVersion）
    pub is_static: bool,
    /// 方法的特殊分类（用于 API 端的特殊处理）
    pub method_kind: MethodKind,
}

/// 方法的特殊分类
#[derive(Clone, Debug, PartialEq)]
pub enum MethodKind {
    /// 普通方法（大多数 Req/Qry 方法）
    General,
    /// RegisterSpi — 需要特殊的内存管理
    RegisterSpi,
    /// Release — 需要 AtomicBool 保护
    Release,
    /// RegisterFront / RegisterNameServer — char* 参数需要 CString 转换
    RegisterString,
    /// Subscribe/UnSubscribe 系列 — IncompleteArray 参数
    Subscribe,
    /// 静态方法（CreateFtdcXxxApi / GetApiVersion）
    Static,
}

/// 参数信息
#[derive(Clone, Debug)]
pub struct ParamInfo {
    /// C++ 原始参数名（IR 完整性保留，generator 不直接使用）
    #[allow(dead_code)]
    pub cpp_name: String,
    /// Rust snake_case 参数名
    pub rust_name: String,
    /// 参数类型分类
    pub kind: ParamKind,
}

/// 参数类型分类
///
/// 将 C++ 的复杂类型系统简化为有限的几种情况，
/// 每种情况在 API 端和 SPI 端有不同的 Rust 表达方式。
#[derive(Clone, Debug)]
pub enum ParamKind {
    /// int → i32
    Int,
    /// bool → bool
    Bool,
    /// enum THOST_TE_RESUME_TYPE → THOST_TE_RESUME_TYPE
    Enum(String),
    /// char * → &str (API 端) / *mut i8 (FFI 端)
    CharPtr,
    /// CThostFtdcXxxField * → &mut T (API 端) / *const T (SPI FFI 端) / Option<&T> (SPI trait 端)
    StructPtr(String),
    /// char *[] (IncompleteArray) → &[impl AsRef<str>] (API 端)
    IncompleteArray,
    /// 常量数组类型，如 TThostFtdcInstrumentIDType[N]
    ConstantArray(String),
}

/// 返回类型
#[derive(Clone, Debug, PartialEq)]
pub enum ReturnType {
    /// void
    Void,
    /// int → i32
    Int,
    /// const char * → String
    CharPtr,
}

// ─── AST 解析函数 ────────────────────────────────────────────

/// 从 AST 中提取指定 C++ 类的所有非静态方法信息
///
/// # 参数
/// - `entity`: clang AST 根节点
/// - `class_name`: 目标 C++ 类名，如 "CThostFtdcTraderApi"
///
/// # 返回
/// 该类所有非静态方法的 MethodInfo 列表（按声明顺序）
pub fn extract_methods(
    entity: &Entity,
    class_name: &str,
    config: &CodegenConfig,
) -> Vec<MethodInfo> {
    let mut methods = Vec::new();
    entity.visit_children(|child, _parent| {
        if child.get_kind() == EntityKind::ClassDecl {
            if let Some(name) = child.get_name() {
                if name == class_name {
                    for m in child.get_children() {
                        if m.get_kind() == EntityKind::Method && !m.is_static_method() {
                            methods.push(extract_method_info(&m, config));
                        }
                    }
                }
            }
        }
        // sopt feature 需要递归进入 namespace
        if cfg!(feature = "sopt") {
            EntityVisitResult::Recurse
        } else {
            EntityVisitResult::Continue
        }
    });
    methods
}

/// 从单个方法 Entity 提取 MethodInfo
fn extract_method_info(m: &Entity, config: &CodegenConfig) -> MethodInfo {
    let cpp_name = m.get_name().unwrap();
    let rust_name = to_rust_method_name(&cpp_name, config.method_to_snake);
    let comment = m.get_comment().unwrap_or_default();
    let is_static = m.is_static_method();

    let params: Vec<ParamInfo> = m
        .get_arguments()
        .unwrap_or_default()
        .iter()
        .map(|a| extract_param_info(a, config))
        .collect();

    let return_type = classify_return_type(&m.get_result_type().unwrap().get_display_name());

    let method_kind = classify_method_kind(&cpp_name, is_static, &params);

    MethodInfo {
        cpp_name,
        rust_name,
        comment,
        params,
        return_type,
        is_static,
        method_kind,
    }
}

/// 从参数 Entity 提取 ParamInfo
fn extract_param_info(e: &Entity, config: &CodegenConfig) -> ParamInfo {
    let cpp_name = e.get_name().unwrap_or_default();
    let rust_name = to_rust_param_name(&cpp_name, config.param_to_snake);
    let tp = e.get_type().unwrap();
    let kind = classify_param_type(&tp);
    ParamInfo {
        cpp_name,
        rust_name,
        kind,
    }
}

/// 将 C++ 参数类型分类为 ParamKind
fn classify_param_type(tp: &clang::Type) -> ParamKind {
    match tp.get_kind() {
        TypeKind::Int => ParamKind::Int,
        TypeKind::Bool => ParamKind::Bool,
        TypeKind::Enum => ParamKind::Enum(rust_type_name(&tp.get_display_name())),
        TypeKind::Elaborated => {
            let canonical = tp.get_canonical_type();
            match canonical.get_kind() {
                TypeKind::Int => ParamKind::Int,
                TypeKind::Bool => ParamKind::Bool,
                TypeKind::Enum => ParamKind::Enum(rust_type_name(&tp.get_display_name())),
                TypeKind::ConstantArray => {
                    ParamKind::ConstantArray(rust_type_name(&tp.get_display_name()))
                }
                TypeKind::Pointer => classify_pointer_type(&canonical),
                _ => ParamKind::StructPtr(rust_type_name(&tp.get_display_name())),
            }
        }
        TypeKind::Pointer => classify_pointer_type(tp),
        TypeKind::IncompleteArray => ParamKind::IncompleteArray,
        other => panic!("未处理的参数类型: {:?} ({})", other, tp.get_display_name()),
    }
}

/// 分类指针类型的 pointee
fn classify_pointer_type(tp: &clang::Type) -> ParamKind {
    let pointee = tp.get_pointee_type().unwrap();
    match pointee.get_kind() {
        TypeKind::CharS | TypeKind::CharU | TypeKind::SChar | TypeKind::UChar => ParamKind::CharPtr,
        TypeKind::Pointer | TypeKind::Elaborated | TypeKind::Record => {
            ParamKind::StructPtr(rust_type_name(&pointee.get_display_name()))
        }
        other => panic!(
            "未处理的指针目标类型: {:?} ({})",
            other,
            pointee.get_display_name()
        ),
    }
}

/// 将 libclang display name 转为 bindgen 当前输出的 Rust 类型名。
fn rust_type_name(display_name: &str) -> String {
    display_name
        .rsplit("::")
        .next()
        .unwrap_or(display_name)
        .to_string()
}

/// 分类返回类型
fn classify_return_type(display_name: &str) -> ReturnType {
    match display_name {
        "void" => ReturnType::Void,
        "int" => ReturnType::Int,
        "const char *" => ReturnType::CharPtr,
        other => panic!("未处理的返回类型: {}", other),
    }
}

/// 根据方法名和参数特征判断方法的特殊分类
fn classify_method_kind(cpp_name: &str, is_static: bool, params: &[ParamInfo]) -> MethodKind {
    if is_static {
        return MethodKind::Static;
    }
    match cpp_name {
        "RegisterSpi" => MethodKind::RegisterSpi,
        "Release" => MethodKind::Release,
        "RegisterFront" | "RegisterNameServer" => MethodKind::RegisterString,
        name if name.contains("Subscribe") => {
            if params
                .iter()
                .any(|p| matches!(p.kind, ParamKind::IncompleteArray))
            {
                MethodKind::Subscribe
            } else {
                MethodKind::General
            }
        }
        _ => MethodKind::General,
    }
}

// ─── 命名工具函数 ────────────────────────────────────────────

/// C++ 方法名 → Rust 方法名
///
/// 当 `snake_case = true` 时转为 snake_case（默认行为），
/// 否则保持 C++ 原始命名（如 `ReqUserLogin`）。
///
/// 特殊处理：CTP 的 "UnSubscribe" 需要先替换为 "Unsubscribe"，
/// 否则 Inflector 会生成 "un_sub_scribe" 而非 "unsubscribe"。
pub fn to_rust_method_name(cpp_name: &str, snake_case: bool) -> String {
    if snake_case {
        cpp_name.replace("UnSub", "Unsub").to_snake_case()
    } else {
        cpp_name.to_string()
    }
}

/// C++ 参数名 → Rust 参数名
///
/// 当 `snake_case = true` 时转为 snake_case（默认行为），
/// 否则保持 C++ 原始命名（如 `pRspUserLogin`）。
pub fn to_rust_param_name(cpp_name: &str, snake_case: bool) -> String {
    if snake_case {
        cpp_name.to_snake_case()
    } else {
        cpp_name.to_string()
    }
}
