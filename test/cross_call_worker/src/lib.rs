mod core;
mod cross_call_master;
use std::ffi::c_void;
use plugify::plugin;

fn on_plugin_start() {
    println!("Rust: on_plugin_start")
}

fn on_plugin_update(_dt: f32) {
    println!("Rust: on_plugin_update")
}

fn on_plugin_end() {
    println!("Rust: on_plugin_end")
}

extern "C" fn main() {
    plugin::on_plugin_start(on_plugin_start);
    plugin::on_plugin_update(on_plugin_update);
    plugin::on_plugin_end(on_plugin_end);
}

#[cfg(target_os = "windows")]
#[unsafe(no_mangle)]
pub extern "system" fn DllMain(
    _module: *mut c_void,
    reason: u32,
    _reserved: *mut c_void,
) -> u32 {
    if reason == 1 {
        main();
    }
    1
}

#[cfg(target_os = "linux")]
#[no_mangle]
#[link_section = ".init_array"]
pub static INIT_ARRAY: extern "C" fn() = main;

#[cfg(target_os = "macos")]
#[no_mangle]
#[link_section = "__DATA,__mod_init_func"]
pub static INIT: extern "C" fn() = main;
