mod util;

use log::info;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlRenderingContext, HtmlCanvasElement};

use crate::util::color::Color;

#[wasm_bindgen]
pub fn start_map_engine(canvas_id: &str) -> Result<(), JsValue> {
    console_log::init_with_level(log::Level::Debug).expect("Failed to init logger");
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(canvas_id).unwrap();

    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| JsValue::from_str("Element is not a canvas"))?;

    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()
        .map_err(|_| JsValue::from_str("Failed to get WebGL context"))?;

    // --- MAP RENDERING LOGIC GOES HERE ---
    // For demonstration, let's clear the canvas to an "ocean blue" map background
    let w @ (r, g, b, a) = Color::from_hex("c4aff5", u8::MAX).ok_or(JsValue::null())?.to_floats();
    info!("{:?}", w);
    context.clear_color(r, g, b, a);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    // In a real application, you would compile your map shaders,
    // load your tile buffers, and kick off a requestAnimationFrame loop here.

    Ok(())
}
