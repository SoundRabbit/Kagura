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

    pub fn on_click(mut self, mut handler: impl FnMut(MouseEvent) -> Msg + 'static) -> Self {
        self.handlers
            .insert("click".to_string(), Box::new(move |_| handler(MouseEvent)));
        self
    }

    pub fn on_input(mut self, mut handler: impl FnMut(String) -> Msg + 'static) -> Self {
        self.handlers.insert(
            "input".to_string(),
            Box::new(move |e| {
                if let Ok(target) = e.target().dyn_into::<native::HtmlInputElement>() {
                    handler(target.value())
                } else {
                    handler("".to_string())
                }
            }),
        );
        self
    }
}
