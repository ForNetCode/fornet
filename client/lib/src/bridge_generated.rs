#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.80.1.

use crate::flutter_api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::rust2dart::IntoIntoDart;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_get_config_path_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String>(
        WrapInfo {
            debug_name: "get_config_path",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(get_config_path()),
    )
}
fn wire_init_runtime_impl(
    port_: MessagePort,
    config_path: impl Wire2Api<String> + UnwindSafe,
    work_thread: impl Wire2Api<usize> + UnwindSafe,
    log_level: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, ()>(
        WrapInfo {
            debug_name: "init_runtime",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || {
            let api_config_path = config_path.wire2api();
            let api_work_thread = work_thread.wire2api();
            let api_log_level = log_level.wire2api();
            move |task_callback| {
                init_runtime(
                    api_config_path,
                    api_work_thread,
                    api_log_level,
                    task_callback.stream_sink::<_, ForNetFlutterMessage>(),
                )
            }
        },
    )
}
fn wire_join_network_impl(port_: MessagePort, invite_code: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String>(
        WrapInfo {
            debug_name: "join_network",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_invite_code = invite_code.wire2api();
            move |task_callback| join_network(api_invite_code)
        },
    )
}
fn wire_list_network_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, Vec<String>>(
        WrapInfo {
            debug_name: "list_network",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| list_network(),
    )
}
fn wire_version_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String>(
        WrapInfo {
            debug_name: "version",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(version()),
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

impl Wire2Api<usize> for usize {
    fn wire2api(self) -> usize {
        self
    }
}
// Section: impl IntoDart

impl support::IntoDart for ForNetFlutterMessage {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::Stop => 0,
            Self::Start => 1,
        }
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for ForNetFlutterMessage {}
impl rust2dart::IntoIntoDart<ForNetFlutterMessage> for ForNetFlutterMessage {
    fn into_into_dart(self) -> Self {
        self
    }
}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
