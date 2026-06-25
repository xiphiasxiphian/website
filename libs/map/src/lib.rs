use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn process_data(name: &str) -> String
{
    format!("Rust says: {} processed successfully at near-native speed!", name)
}
