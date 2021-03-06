//! Simple rain simulation as web app written in Rust compiled into WASM, thus running in browsers.

// Make writing "unsafe" in code a compilation error. We should not need unsafe at all.
#![forbid(unsafe_code)]
// Warn on generally recommended lints that are not enabled by default.
#![warn(future_incompatible, rust_2018_idioms, unused, macro_use_extern_crate)]
// Warn when we write more code than necessary.
#![warn(unused_lifetimes, single_use_lifetimes, unreachable_pub, trivial_casts)]
// Warn when we don't implement (derive) commonly needed traits. May be too strict.
#![warn(missing_copy_implementations, missing_debug_implementations)]
// Turn on some extra Clippy (Rust code linter) warnings. Run `cargo clippy`.
#![warn(clippy::all)]

use log::{info, trace, Level};
use snapwater::solve_landscape;
use std::str::FromStr;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlInputElement, Window};

/// Number of pixels one "segment" has horizontally, also height of "one hour" of a rain.
const BLOCK_PIXELS: f64 = 30.0;

// https://www.pinterest.com/pin/103371753933791633/
// https://www.pinterest.com/pin/295548794293211914/
const LAND_COLOR: &str = "#eed994";
const WATER_COLOR: &str = "#0c60ae";
const SKY_COLOR: &str = "#edf4f4";
const CLOUD_COLOR: &str = "#b0b8bb";

/// This method gets called automatically when the WASM module is initialized.
#[wasm_bindgen(start)]
pub fn initialize() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Debug).expect("logging can be initialized");

    let start_button = document().get_element_by_id("start").expect("#start element exists");

    let boxed_simulate: Box<dyn Fn()> = Box::new(simulate_world);
    // The .into_js_value() actually leaks the box, see its docs.
    let simulate_closure = Closure::wrap(boxed_simulate).into_js_value();
    start_button
        .add_event_listener_with_callback("click", simulate_closure.unchecked_ref())
        .expect("callback can be attached");
    info!("WASM Rain init done.");
}

/// Convenience wrapper to get DOM Window.
fn window() -> Window {
    web_sys::window().unwrap()
}

/// Convenience wrapper to get DOM Document.
fn document() -> Document {
    window().document().unwrap()
}

/// Called when user clicks on the Start button.
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

    let start_button = input_element(&doc, "start");
    start_button.set_value("Raining...");
    start_button.set_disabled(true);

    let world = World::new(landscape, rain_hours);
    world.schedule_next_or_finish();
}

/// Called by simulated world when it finishes.
fn finish_simulation() {
    let start_button = input_element(&document(), "start");
    start_button.set_value("Start");
    start_button.set_disabled(false);
}

/// Convenience wrapper to get HTML <input> element by id.
fn input_element(doc: &Document, id: &str) -> HtmlInputElement {
    doc.get_element_by_id(id)
        .expect("element with given id exists")
        .dyn_into()
        .expect("element is html <input>")
}

/// Description of a current state of a world we are simulating.
struct World {
    landscape: Vec<f64>,
    surface: Vec<f64>,
    remaining_rain_hours: f64,
    canvas_width: f64,
    canvas_height: f64,
    context: CanvasRenderingContext2d,
    last_timestamp: Option<f64>,
}

impl World {
    /// Create the world. Resizes the canvas and draws landscape on it, but does not animate.
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
            remaining_rain_hours: rain_hours,
            canvas_width,
            canvas_height,
            context,
            last_timestamp: None,
        };
        world.draw_land_sky();
        world
    }

    /// Draw land and sky/cloud on the canvas. Erases all previous canvas contents.
    fn draw_land_sky(&self) {
        // draw sky or cloud to whole canvas
        let sky_color = if self.remaining_rain_hours > 0.0 { CLOUD_COLOR } else { SKY_COLOR };
        self.context.set_fill_style(&sky_color.into());
        self.context.fill_rect(0.0, 0.0, self.canvas_width, self.canvas_height);

        // draw land segments
        self.context.set_fill_style(&LAND_COLOR.into());
        for (i, &land) in self.landscape.iter().enumerate() {
            let x_offset = i as f64 * BLOCK_PIXELS;
            let pixel_height = land * BLOCK_PIXELS;
            self.context.fill_rect(
                x_offset,
                self.canvas_height - pixel_height,
                BLOCK_PIXELS,
                pixel_height,
            );
        }
    }

    /// Draw water levels on the canvas. Expects that landscape is already drawn.
    fn draw_water(&self) {
        self.context.set_fill_style(&WATER_COLOR.into());
        for (i, (&land, &surface)) in self.landscape.iter().zip(&self.surface).enumerate() {
            let pixel_height = (surface - land) * BLOCK_PIXELS;
            if pixel_height <= 0.0 {
                continue; // nothing to draw
            }

            let x_offset = i as f64 * BLOCK_PIXELS;
            let pixel_surface = surface * BLOCK_PIXELS;
            self.context.fill_rect(
                x_offset,
                self.canvas_height - pixel_surface,
                BLOCK_PIXELS,
                pixel_height,
            );
        }
    }

    /// Schedule the next frame or finish. Consumes the World.
    fn schedule_next_or_finish(self) {
        if self.remaining_rain_hours <= 0.0 {
            info!("Final landscape: {:?}.", self.surface);
            finish_simulation();
            return;
        }

        let closure = Closure::once_into_js(|timestamp| self.step(timestamp));
        window()
            .request_animation_frame(closure.unchecked_ref())
            .expect("request_animation_frame() works");
    }

    /// Perform one animation step, draw it and either finish or schedule a new one
    fn step(mut self, timestamp: f64) {
        let elapsed_ms = match self.last_timestamp {
            Some(last_timestamp) => timestamp - last_timestamp,
            None => 0.0, // this should happen only for the very first iteration
        };
        self.last_timestamp = Some(timestamp);
        trace!("step, elapsed_ms: {}", elapsed_ms);

        // Advance world state. One simulation hour is one real second for us.
        let rain_hours = elapsed_ms / 1000.0;
        // fn solve_landscape(landscape: impl IntoIterator<Item = f64>, rain_hours: f64) -> Vec<f64>
        self.surface = solve_landscape(self.surface, rain_hours);
        self.remaining_rain_hours -= rain_hours;

        // Draw.
        if self.remaining_rain_hours <= 0.0 {
            // Switch from cloudy sky to clear sky.
            self.draw_land_sky();
        }
        self.draw_water();

        self.schedule_next_or_finish();
    }
}
