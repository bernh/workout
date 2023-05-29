use wasm_bindgen::prelude::*;

// declare internal modules
mod config;
mod gui;
mod parse;
mod utils;
mod wtree;

// re-export public Rust API on top level to define crate extenal API
pub use crate::config::init;
pub use crate::gui::gui_create;
pub use crate::parse::summarize;

// define WASM API and implement wrappers
#[wasm_bindgen]
pub fn wasm_init(paces: JsValue) {
    config::init(serde_wasm_bindgen::from_value(paces).unwrap())
}

#[wasm_bindgen]
pub fn wasm_summarize(input: &str) -> String {
    parse::summarize(input)
}
