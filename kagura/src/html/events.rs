extern crate wasm_bindgen;

use crate::dom;
use crate::dom::native;
use wasm_bindgen::JsCast;

pub struct Events<Msg> {
    pub on_click: Option<Box<FnMut(dom::MouseEvent) -> Msg>>,
    pub on_input: Option<Box<FnMut(dom::Event) -> Msg>>,
}

pub struct MouseEvent;

impl<Msg> Events<Msg> {
    pub fn new() -> Self {
        Self {
            on_click: None,
            on_input: None,
        }
    }

    pub fn with_on_click(mut self, mut handler: impl FnMut(MouseEvent) -> Msg + 'static) -> Self {
        self.on_click = Some(Box::new(move |_| handler(MouseEvent)));
        self
    }

    pub fn with_on_input(mut self, mut handler: impl FnMut(String) -> Msg + 'static) -> Self {
        self.on_input = Some(Box::new(move |e| {
            if let Ok(target) = e.target().dyn_into::<native::HtmlInputElement>() {
                handler(target.value())
            } else {
                handler("".to_string())
            }
        }));
        self
    }
}
