use std::collections::HashMap;

use clang::{Entity, TypeKind};

use super::parser::{
    convert_api_trait_func_, convert_base_type_, convert_char_s_type_, convert_incomplete_array,
    convert_pointee_type, convert_spi_trait_func_, handle_module_api_trait,
    handle_module_event_enum, handle_module_event_struct, handle_module_extern_cfn,
    handle_module_spi_trait, handle_module_static_table, handle_module_vtable_struct,
};

#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub enum ModuleFlavor {
    /// method is in a trait
    ApiTrait,
    SpiTrait,
    /// method is in a struct
    VTableStruct,
    StaticTable,
    EventEnum,
    EventEnumStruct,
    ExternCFn,
    SpiFn,
    /// only add debug log
    None,
}

#[derive(Clone, Debug, Copy)]
pub enum PointerStyle {
    RawPointerConst, // *const CThostFtdcRspUserLoginField
    RawPointerMut,   // *mut CThostFtdcRspUserLoginField
    MutRef,          // &mut CThostFtdcRspUserLoginField
    Ref,             // & CThostFtdcRspUserLoginField
    OptionRef,       // Option<&CThostFtdcRspUserLoginField>
}

impl Default for PointerStyle {
    fn default() -> Self {
        PointerStyle::RawPointerMut // 或者你认为合适的默认值
    }
}
#[derive(Clone, Debug)]
pub struct Config {
    pub version: String,
    pub module_flavor: ModuleFlavor,
    pub source_class_name: String, // CThostFtdcMdApi | CThostFtdcMdSpi | CThostFtdcTraderApi | CThostFtdcTraderSpi
    pub generate_trait_name: String,
    pub wrap_spi_trait: String,
    pub wrap_api_struct: bool,
    pub debug: bool,
    /* format style */
    pub generate_comments: bool, // keep comments
    pub method_to_snake: bool,   // 默认将方法名转换为蛇形命名: ReqUserLogin ? req_user_login
    pub life_time: String,
    pub param_to_snake: bool,         // pRspInfo ? p_rsp_info
    pub param_trim_prefix: bool,      // p_rsp_info ? rsp_info
    pub prefer_self_mut_ref: bool,    //  &self ? &mut self
    pub prefer_param_cffi_type: bool, // int -> i32 ? std::os::raw::c_int
    pub prefer_pointer: PointerStyle, // *const _ | *mut _| Option<&_>
}

impl Default for Config {
    fn default() -> Self {
        Config {
            version: "v1alpha1".to_string(),
            module_flavor: ModuleFlavor::None,
            source_class_name: "".to_string(),
            generate_trait_name: "".to_string(),
            wrap_spi_trait: "".to_string(),
            wrap_api_struct: false,
            debug: false,
            generate_comments: true,
            method_to_snake: true,
            life_time: "".to_string(),
            param_to_snake: true,
            param_trim_prefix: false,
            prefer_self_mut_ref: false,
            prefer_param_cffi_type: false,
            prefer_pointer: PointerStyle::default(),
        }
    }
}

impl Config {
    /// 创建一个默认配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 根据已有配置，设置额外选项
    pub fn set_option<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Config),
    {
        f(&mut self);
        self
    }

    /// 为 API 创建专用的配置
    pub fn for_api() -> Self {
        Config::default().set_option(|cfg| {
            cfg.module_flavor = ModuleFlavor::ApiTrait;
            cfg.source_class_name = "CThostFtdcTraderApi".to_string();
            cfg.generate_trait_name = "TraderApi".to_string();
            cfg.wrap_spi_trait = "TraderSpi".to_string();
            cfg.wrap_api_struct = true;
            cfg.debug = true;
            cfg.generate_comments = true;
            cfg.method_to_snake = true;
            cfg.param_to_snake = true;
            cfg.param_trim_prefix = false;
            cfg.prefer_self_mut_ref = false;
            cfg.prefer_param_cffi_type = false;
            cfg.prefer_pointer = PointerStyle::MutRef;
        })
    }

    /// 为 SPI 创建专用的配置
    pub fn for_spi() -> Self {
        Config::default().set_option(|cfg| {
            cfg.module_flavor = ModuleFlavor::SpiTrait;
            cfg.source_class_name = "CThostFtdcTraderSpi".to_string();
            cfg.generate_trait_name = "TraderSpi".to_string();
            cfg.wrap_spi_trait = "TraderSpi".to_string();
            cfg.wrap_api_struct = false;
            cfg.debug = false;
            cfg.generate_comments = true;
            cfg.method_to_snake = true;
            cfg.param_to_snake = true;
            cfg.param_trim_prefix = false;
            cfg.prefer_self_mut_ref = true;
            cfg.prefer_param_cffi_type = false;
            cfg.prefer_pointer = PointerStyle::RawPointerMut;
        })
    }
}

pub type ParamVec = Vec<(String, String, String)>;
pub type ItemVec = Vec<Vec<String>>;

pub type ModuleHandler = dyn Fn(&Context, &Entity, &ItemVec) -> String;
pub type FuncHandler = dyn Fn(&Context, &Entity, &ParamVec) -> Vec<String>;
pub type ParamHandler = dyn Fn(&Context, &str, &str, &TypeKind) -> (String, String, String);

// type HandlerMap<'a> = HashMap<String, Box<MethodHandler<'a>>>;

pub struct Context {
    pub cfg: Config,
    pub modules: HashMap<ModuleFlavor, Box<ModuleHandler>>,
    pub methods: HashMap<String, Box<FuncHandler>>,
    pub params: HashMap<TypeKind, Box<ParamHandler>>,
}

impl Context {
    /// 通用构造函数
    pub fn new(cfg: Config) -> Self {
        Context {
            cfg,
            modules: HashMap::new(),
            methods: HashMap::new(),
            params: HashMap::new(),
        }
    }

    /// 为 API 创建 Context 的构造函数
    pub fn for_api(cfg: Config) -> Self {
        let mut ctx = Self::new(cfg);
        ctx.cfg.prefer_param_cffi_type = false;
        ctx.modules
            .insert(ModuleFlavor::ApiTrait, Box::new(handle_module_api_trait));

        ctx.methods
            .insert("_".to_string(), Box::new(convert_api_trait_func_));

        ctx.params
            .insert(TypeKind::Int, Box::new(convert_base_type_));
        ctx.params
            .insert(TypeKind::Bool, Box::new(convert_base_type_));
        ctx.params
            .insert(TypeKind::Enum, Box::new(convert_base_type_));
        ctx.params
            .insert(TypeKind::CharS, Box::new(convert_char_s_type_));
        ctx.params.insert(
            TypeKind::IncompleteArray,
            Box::new(convert_incomplete_array),
        );
        ctx.params
            .insert(TypeKind::Pointer, Box::new(convert_pointee_type));

        ctx
    }

    /// 为 SPI 创建 Context 的构造函数
    pub fn for_spi(cfg: Config) -> Self {
        let mut ctx = Self::new(cfg);

        ctx.modules
            .insert(ModuleFlavor::SpiTrait, Box::new(handle_module_spi_trait));
        ctx.modules.insert(
            ModuleFlavor::VTableStruct,
            Box::new(handle_module_vtable_struct),
        );
        ctx.modules.insert(
            ModuleFlavor::StaticTable,
            Box::new(handle_module_static_table),
        );
        ctx.modules
            .insert(ModuleFlavor::ExternCFn, Box::new(handle_module_extern_cfn));

        ctx.modules
            .insert(ModuleFlavor::EventEnum, Box::new(handle_module_event_enum));

        ctx.modules.insert(
            ModuleFlavor::EventEnumStruct,
            Box::new(handle_module_event_struct),
        );

        ctx.methods
            .insert("_".to_string(), Box::new(convert_spi_trait_func_));

        ctx.params
            .insert(TypeKind::Int, Box::new(convert_base_type_));
        ctx.params
            .insert(TypeKind::Bool, Box::new(convert_base_type_));
        ctx.params
            .insert(TypeKind::Enum, Box::new(convert_base_type_));
        ctx.params
            .insert(TypeKind::CharS, Box::new(convert_char_s_type_));
        ctx.params
            .insert(TypeKind::Pointer, Box::new(convert_pointee_type));

        ctx
    }
}
