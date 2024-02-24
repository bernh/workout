// declare internal modules
mod config;
mod parse;
mod utils;
mod wtree;

#[cfg(feature = "egui")]
mod egui;

// re-export public Rust API on top level to define crate extenal API
pub use crate::config::init;
pub use crate::parse::summarize;

#[cfg(feature = "egui")]
pub use crate::egui::gui_create;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// define WASM API and implement wrappers
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_init(paces: JsValue) {
    config::init(serde_wasm_bindgen::from_value(paces).unwrap())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_summarize(input: &str) -> String {
    parse::summarize(input).unwrap_or("invalid workout".to_string())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_gui_create() {
    egui::gui_create();
}
