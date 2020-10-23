use log::{info, Level};
use std::str::FromStr;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlInputElement, Window};

const BLOCK_PIXELS: f64 = 30.0;

// https://www.pinterest.com/pin/103371753933791633/
// https://www.pinterest.com/pin/295548794293211914/
const LAND_COLOR: &str = "#eed994";
const WATER_COLOR: &str = "#0c60ae";
const SKY_COLOR: &str = "#edf4f4";
const CLOUD_COLOR: &str = "#b0b8bb";

#[wasm_bindgen(start)]
pub fn initialize() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Debug).expect("logging can be initialized");

    let start_button = document().get_element_by_id("start").expect("#start element exists");

    let boxed_fn = Closure::wrap(Box::new(simulate_world) as Box<dyn Fn()>).into_js_value();
    start_button
        .add_event_listener_with_callback("click", boxed_fn.as_ref().unchecked_ref())
        .expect("callback can be attached");
    info!("WASM Rain init done.");
}

fn document() -> Document {
    window().document().unwrap()
}

fn window() -> Window {
    web_sys::window().unwrap()
}

fn simulate_world() {
    let doc = document();
    let landscape_string: String = input_element(&doc, "landscape").value();
    let landscape_result: Result<Vec<_>, _> = landscape_string
        .split(&[' ', ','][..])
        .map(|string| {
            f64::from_str(string).map_err(|e| format!("Cannot parse '{}' as number: {}", string, e))
        })
        .collect();
    let landscape = match landscape_result {
        Ok(landscape) => landscape,
        Err(error) => {
            window().alert_with_message(&error).expect("alert() works");
            return;
        }
    };

    let rain_hours = input_element(&doc, "rain").value_as_number();

    info!("Simulate world with landscape: {:?} and {} hours of rain.", landscape, rain_hours);
    draw_on_canvas(&landscape);

    let start_button = input_element(&doc, "start");
    start_button.set_value("Raining...");
    start_button.set_disabled(true);
}

fn input_element(doc: &Document, id: &str) -> HtmlInputElement {
    doc.get_element_by_id(id)
        .expect("element with given id exists")
        .dyn_into()
        .expect("element is html <input>")
}

fn draw_on_canvas(surface: &[f64]) {
    let canvas: HtmlCanvasElement = document()
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

    // draw "cloud" to whole canvas
    context.set_fill_style(&CLOUD_COLOR.into());
    context.fill_rect(0.0, 0.0, width, height);

    // draw land segments
    context.set_fill_style(&LAND_COLOR.into());
    for (i, &segment_blocks) in surface.iter().enumerate() {
        let x_offset = i as f64 * BLOCK_PIXELS;
        let segment_height = segment_blocks * BLOCK_PIXELS;
        context.fill_rect(x_offset, height - segment_height, BLOCK_PIXELS, segment_height);
    }
}
