mod core;
mod cross_call_master;
use plugify::{register_plugin};

fn on_plugin_start() {
    println!("Rust: on_plugin_start")
}

fn on_plugin_update(_dt: f32) {
    println!("Rust: on_plugin_update")
}

fn on_plugin_end() {
    println!("Rust: on_plugin_end")
}

register_plugin!(
    start: on_plugin_start,
    update: on_plugin_update,
    end: on_plugin_end
);
