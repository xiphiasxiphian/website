mod canvas;
mod renderer;
mod util;
mod input;

use log::info;
use wasm_bindgen::prelude::*;

use crate::canvas::Canvas;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue>
{
    console_log::init_with_level(log::Level::Debug).map_err(|e| JsValue::from_str(&e.to_string()))?;
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    info!("Map WASM Module Loaded Successfully");
    Ok(())
}

#[wasm_bindgen]
pub fn start_map_engine(canvas_id: String)
{
    wasm_bindgen_futures::spawn_local(async move {
        Canvas::attach(&canvas_id).await.run().await;
    });
}
