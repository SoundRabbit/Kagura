extern crate wasm_bindgen;

use crate::native;
use std::collections::HashMap;
use wasm_bindgen::JsCast;

pub struct Events<Msg> {
    pub handlers: HashMap<String, Box<FnMut(native::Event) -> Msg>>,
}

pub struct MouseEvent;

impl<Msg> Events<Msg> {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn on(
        mut self,
        type_: impl Into<String>,
        handler: impl FnMut(native::Event) -> Msg + 'static,
    ) -> Self {
        self.handlers.insert(type_.into(), Box::new(handler));
        self
    }

    pub fn on_click(self, mut handler: impl FnMut(MouseEvent) -> Msg + 'static) -> Self {
        self.on("click", move |_| handler(MouseEvent))
    }

    pub fn on_input(self, mut handler: impl FnMut(String) -> Msg + 'static) -> Self {
        self.on("input", move |e| {
            if let Ok(target) = e.target().dyn_into::<native::HtmlInputElement>() {
                handler(target.value())
            } else {
                handler("".to_string())
            }
        })
    }
}
