use std::{fs::File, io::Write, path::Path};

// #[allow(named_arguments_used_positionally)]
use clang::Entity;

use crate::{
    convert_spi_event_enum_, convert_spi_event_struct_,
    setting::{Config, ModuleFlavor, PointerStyle},
    traverse_ast,
};

use super::{
    rustify_method_name2,
    setting::{Context, ItemVec, ParamVec},
};

pub fn generate_stream_wrapper_code<P: AsRef<Path>>(e: &Entity, target_dir: P) {
    let mut cfg = Config::for_spi();
    cfg.source_class_name = "CThostFtdcMdSpi".to_string();
    cfg.generate_trait_name = "MdSpi".to_string();
    cfg.wrap_spi_trait = "MdSpi".to_string();

    let mut ctx = Context::for_spi(cfg);
    // let mdspi_event_code = prepare_spi_event_code(&mut ctx, e);

    let mut cfg = Config::for_spi();
    cfg.source_class_name = "CThostFtdcTraderSpi".to_string();
    cfg.generate_trait_name = "TraderSpi".to_string();
    cfg.wrap_spi_trait = "TraderSpi".to_string();

    ctx.cfg = cfg;
    let spi_event_code = prepare_spi_event_code(e);

    let mut file = File::create(target_dir.as_ref().join("event.rs")).expect("create file error");
    file.write_all(spi_event_code.as_bytes())
        .expect("write code failed!");

    let stream_spi_code = prepare_stream_spi_code(e);
    let mut file = File::create(target_dir.as_ref().join("stream.rs")).expect("create file error");
    file.write_all(stream_spi_code.as_bytes())
        .expect("write code failed!");
}

pub fn prepare_spi_event_code(e: &Entity) -> String {
    let mut cfg = Config::for_spi();
    cfg.source_class_name = "CThostFtdcMdSpi".to_string();
    cfg.generate_trait_name = "MdSpi".to_string();
    cfg.wrap_spi_trait = "MdSpi".to_string();

    let mut ctx = Context::for_spi(cfg);

    ctx.cfg = Config {
        module_flavor: ModuleFlavor::EventEnum,
        prefer_pointer: PointerStyle::RawPointerConst,
        prefer_param_cffi_type: true,
        param_trim_prefix: false,
        ..ctx.cfg.clone()
    };
    ctx.methods
        .insert("_".to_string(), Box::new(convert_spi_event_enum_));

    let code_0 = traverse_ast(&mut ctx, e);

    ctx.cfg = Config {
        module_flavor: ModuleFlavor::EventEnumStruct,
        prefer_pointer: PointerStyle::RawPointerConst,
        prefer_param_cffi_type: false,
        param_trim_prefix: true,
        ..ctx.cfg.clone()
    };
    ctx.methods
        .insert("_".to_string(), Box::new(convert_spi_event_struct_));

    let code_1 = traverse_ast(&mut ctx, e);

    ctx.cfg = Config {
        source_class_name: "CThostFtdcTraderSpi".to_string(),
        generate_trait_name: "TraderSpi".to_string(),
        wrap_spi_trait: "TraderSpi".to_string(),

        module_flavor: ModuleFlavor::EventEnum,
        prefer_pointer: PointerStyle::RawPointerConst,
        prefer_param_cffi_type: false,
        param_trim_prefix: true,
        ..ctx.cfg.clone()
    };
    ctx.methods
        .insert("_".to_string(), Box::new(convert_spi_event_enum_));

    let code_2 = traverse_ast(&mut ctx, e);

    ctx.cfg = Config {
        module_flavor: ModuleFlavor::EventEnumStruct,
        prefer_pointer: PointerStyle::RawPointerConst,
        prefer_param_cffi_type: false,
        param_trim_prefix: true,
        ..ctx.cfg.clone()
    };
    ctx.methods
        .insert("_".to_string(), Box::new(convert_spi_event_struct_));
    let code_3 = traverse_ast(&mut ctx, e);

    let version = &ctx.cfg.version;
    format!(
        r#"
use crate::{version}::bindings::*;
{code_0}
{code_1}
{code_2}
{code_3}
"#
    )
}

pub fn prepare_stream_spi_code(e: &Entity) -> String {
    let mut cfg = Config::for_spi();
    cfg.source_class_name = "CThostFtdcMdSpi".to_string();
    cfg.generate_trait_name = "MdSpi".to_string();
    cfg.wrap_spi_trait = "MdSpi".to_string();

    let mut ctx = Context::for_spi(cfg);
    ctx.cfg = Config {
        module_flavor: ModuleFlavor::SpiFn,
        prefer_pointer: PointerStyle::RawPointerConst,
        prefer_param_cffi_type: false,
        param_trim_prefix: true,
        ..ctx.cfg.clone()
    };

    ctx.methods
        .insert("_".to_string(), Box::new(convert_stream_spi_trait_func_));
    ctx.modules
        .insert(ModuleFlavor::SpiFn, Box::new(handle_module_spi_stream));

    let code_0 = traverse_ast(&mut ctx, e);

    ctx.cfg = Config {
        source_class_name: "CThostFtdcTraderSpi".to_string(),
        generate_trait_name: "TraderSpi".to_string(),
        wrap_spi_trait: "TraderSpi".to_string(),

        module_flavor: ModuleFlavor::SpiFn,
        prefer_pointer: PointerStyle::RawPointerConst,
        prefer_param_cffi_type: false,
        param_trim_prefix: true,
        ..ctx.cfg.clone()
    };
    let code_1 = traverse_ast(&mut ctx, e);
    let version = &ctx.cfg.version;
    let header = format!(
        r#"
use futures::stream::Stream;
use std::{{
    pin::Pin,
    sync::{{Arc, Mutex}},
    task::Waker,
}};

use crate::{version}::bindings::*;
use crate::{version}::event::*;
use crate::{version}::mdspi::*;
use crate::{version}::traderspi::*;
"#
    );
    format!(
        r#"{header}
{code_0}
{code_1}
"#
    )
}

pub fn handle_module_spi_stream(ctx: &Context, e: &Entity, items: &ItemVec) -> String {
    let spi_trait = &ctx.cfg.generate_trait_name;

    let declare_code = format!(
        r#"
struct {spi_trait}Inner {{
    buf: std::collections::VecDeque<{spi_trait}Event>,
    waker: Option<Waker>,
}}

impl {spi_trait}Inner {{
    fn push(&mut self, msg: {spi_trait}Event) {{
        self.buf.push_back(msg);
        if let Some(waker) = self.waker.take() {{
            waker.wake()
        }}
    }}
}}

pub struct {spi_trait}Stream {{
    inner: Arc<Mutex<{spi_trait}Inner>>,
}}

impl Stream for {spi_trait}Stream {{
    type Item = {spi_trait}Event;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut futures::task::Context<'_>,
    ) -> futures::task::Poll<Option<Self::Item>> {{
        use futures::task::Poll;
        let mut inner = self.inner.lock().unwrap();
        if let Some(i) = inner.buf.pop_front() {{
            Poll::Ready(Some(i))
        }} else {{
            inner.waker = Some(cx.waker().clone());
            Poll::Pending
        }}
    }}

    fn size_hint(&self) -> (usize, Option<usize>) {{
        (0, None)
    }}
}}"#
    );

    format!(
        r#"{declare_code}
impl {spi_trait} for {spi_trait}Stream {{ 
{spi_code}
}}
    "#,
        spi_code = items.join("\n")
    )
}

macro_rules! indent {
    ($width:expr) => {
        " ".repeat($width)
    };
}

pub fn convert_stream_spi_trait_func_(ctx: &Context, e: &Entity, params: &ParamVec) -> String {
    let method_call = e.get_name().unwrap();
    let method_name = match ctx.cfg.method_to_snake {
        true => rustify_method_name2(&method_call),
        false => method_call.clone(),
    };
    let spi_trait = &ctx.cfg.generate_trait_name;
    let mut arg_vec = params
        .iter()
        .map(|x| format!("{}: {}", x.0, x.1))
        .collect::<Vec<String>>();

    if ctx.cfg.prefer_self_mut_ref {
        arg_vec.insert(0, "&mut self".to_string());
    } else {
        arg_vec.insert(0, "&self".to_string());
    }
    let field_code = params
        .iter()
        .map(|x| format!("{}{}: {}", indent!(16), x.0, x.2))
        .collect::<Vec<_>>()
        .join(",\n");

    let trait_method_code = format!(
        r#"   
    fn {method_name}({arg_list}) {{
        self.inner.lock().unwrap().push({spi_trait}Event::{method_call}(
            {spi_trait}{method_call}Event {{ 
{field_code} 
            }}
        ))
    }}"#,
        arg_list = arg_vec.join(", "),
    );
    trait_method_code
}
