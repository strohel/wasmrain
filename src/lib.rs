use log::{info, Level};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

const BLOCK_PIXELS: f64 = 30.0;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Debug).expect("logging can be initialized");

    draw_on_canvas(&[20.0, 18.0, 19.0, 16.0, 18.0, 14.0, 17.0, 12.0, 16.0, 10.0, 15.0]);
    info!("WASM Rain init done.");
}

fn draw_on_canvas(surface: &[f64]) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas: HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .expect("#canvas element exists")
        .dyn_into()
        .expect("#canvas is a HTML <canvas> element");
    let width = surface.len() as f64 * BLOCK_PIXELS;
    let height = surface.iter().cloned().fold(0.0, f64::max) * BLOCK_PIXELS;
    info!("Setting canvas size (w x h) to {} x {}.", width, height);
    canvas.set_width(width as u32);
    canvas.set_height(height as u32);

    let context: CanvasRenderingContext2d =
        canvas.get_context("2d").unwrap().unwrap().dyn_into().expect("CanvasRenderingContext2d");

    for (i, &segment_blocks) in surface.iter().enumerate() {
        let segment_height = segment_blocks * BLOCK_PIXELS;
        context.fill_rect(
            i as f64 * BLOCK_PIXELS,
            height - segment_height,
            BLOCK_PIXELS,
            segment_height,
        );
    }
}
