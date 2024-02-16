// declare internal modules
mod config;
mod parse;
mod utils;
mod wtree;

// re-export public Rust API on top level to define crate extenal API
pub use crate::config::init;
pub use crate::parse::summarize;


#[cfg(feature = "egui")]
mod egui;
#[cfg(feature = "egui")]
pub use crate::egui::gui_create;


#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

// define WASM API and implement wrappers
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn wasm_init(paces: JsValue) {
    config::init(serde_wasm_bindgen::from_value(paces).unwrap())
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn wasm_summarize(input: &str) -> String {
    parse::summarize(input).unwrap_or("invalid workout".to_string())
}
