use log::{debug, error, info, trace, warn, Level};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn start() {
    console_log::init_with_level(Level::Debug).expect("Logging can be initialized");
    trace!("trace");
    debug!("debug");
    info!("info");
    warn!("warn");
    error!("error");
    alert(&format!("Hello, {}!", "Matejaku"));
}
