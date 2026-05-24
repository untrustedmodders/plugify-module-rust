mod core;
mod cross_call_master;
use plugify::{register_plugin};
use std::error::Error;

fn on_plugin_start() -> Result<(), Box<dyn Error>> {
    println!("Rust: on_plugin_start");
    Ok(())
}

fn on_plugin_update(_dt: f32) -> Result<(), Box<dyn Error>> {
    println!("Rust: on_plugin_update");
    Ok(())
}

fn on_plugin_end() -> Result<(), Box<dyn Error>> {
    println!("Rust: on_plugin_end");
    Ok(())
}

register_plugin!(
    start: on_plugin_start,
    update: on_plugin_update,
    end: on_plugin_end
);
