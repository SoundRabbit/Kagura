extern crate js_sys;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate web_sys;

mod document;
pub mod html;
mod libs;

pub use document::Document;
pub use html::Html;
