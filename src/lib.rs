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

    let world = World::new(landscape, rain_hours);

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

struct World {
    landscape: Vec<f64>,
    surface: Vec<f64>,
    remaining_water_hours: f64,
    canvas_width: f64,
    canvas_height: f64,
    context: CanvasRenderingContext2d,
}

impl World {
    fn new(landscape: Vec<f64>, rain_hours: f64) -> Self {
        info!("Simulate world with landscape: {:?} and {} hours of rain.", landscape, rain_hours);
        let canvas_width = landscape.len() as f64 * BLOCK_PIXELS;
        let max_segment_height = landscape.iter().cloned().fold(0.0, f64::max);
        let canvas_height = (max_segment_height + rain_hours).ceil() * BLOCK_PIXELS;

        info!("Setting canvas size (w x h) to {} x {}.", canvas_width, canvas_height);
        let canvas: HtmlCanvasElement = document()
            .get_element_by_id("canvas")
            .expect("#canvas element exists")
            .dyn_into()
            .expect("#canvas is a HTML <canvas> element");
        canvas.set_width(canvas_width as u32);
        canvas.set_height(canvas_height as u32);

        let context: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .expect("CanvasRenderingContext2d");

        let world = Self {
            surface: landscape.clone(),
            landscape,
            remaining_water_hours: rain_hours,
            canvas_width,
            canvas_height,
            context,
        };
        world.draw_land_sky();
        world
    }

    fn draw_land_sky(&self) {
        // draw sky or cloud to whole canvas
        let sky_color = if self.remaining_water_hours > 0.0 { CLOUD_COLOR } else { SKY_COLOR };
        self.context.set_fill_style(&sky_color.into());
        self.context.fill_rect(0.0, 0.0, self.canvas_width, self.canvas_height);

        // draw land segments
        self.context.set_fill_style(&LAND_COLOR.into());
        for (i, &segment_blocks) in self.landscape.iter().enumerate() {
            let x_offset = i as f64 * BLOCK_PIXELS;
            let segment_height = segment_blocks * BLOCK_PIXELS;
            self.context.fill_rect(
                x_offset,
                self.canvas_height - segment_height,
                BLOCK_PIXELS,
                segment_height,
            );
        }
    }
}
