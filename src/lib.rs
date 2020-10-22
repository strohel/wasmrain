use log::{debug, error, info, trace, warn, Level};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)]
pub fn start() {
    console_log::init_with_level(Level::Debug).expect("Logging can be initialized");
    trace!("trace");
    debug!("debug");
    info!("info");
    warn!("warn");
    error!("error");
    let window = web_sys::window().expect("window object exists");
    window
        .alert_with_message(&format!("Hello, {}!", "Matejaku"))
        .expect("call succeeds");
}
