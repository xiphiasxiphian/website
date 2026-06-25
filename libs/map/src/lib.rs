mod util;
mod canvas;

use log::info;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement};
use wgpu::MemoryHints;
use wgpu::PowerPreference;
use wgpu::{
    Instance,
    SurfaceTarget,
    RequestAdapterOptions,
    DeviceDescriptor,
    Limits,
    SurfaceConfiguration,
    TextureUsages,
    PresentMode,
    CommandEncoderDescriptor,
    RenderPassDescriptor,
    RenderPassColorAttachment,
    Operations,
    LoadOp,
    StoreOp,
};

use crate::canvas::Canvas;
use crate::util::color::Color;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue>
{
    console_log::init_with_level(log::Level::Debug).map_err(|e| JsValue::from_str(&e.to_string()))?;
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    info!("WASM Module Loaded Successfully");
    Ok(())
}

#[wasm_bindgen]
pub fn start_map_engine(canvas_id: String)
{
    wasm_bindgen_futures::spawn_local(async move {
        Canvas::attach(&canvas_id).await.run().await;
    });
}
