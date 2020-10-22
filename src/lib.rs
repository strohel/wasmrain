use log::{info, Level};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Debug).expect("logging can be initialized");

    draw_on_canvas();
    info!("WASM Rain init done.");
}

fn draw_on_canvas() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas: HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .expect("#canvas element exists")
        .dyn_into()
        .expect("#canvas is a HTML <canvas> element");
    let context: CanvasRenderingContext2d =
        canvas.get_context("2d").unwrap().unwrap().dyn_into().expect("CanvasRenderingContext2d");

    context.fill_rect(10.0, 20.0, 30.0, 40.0);
}
