extern crate wasm_bindgen;

use crate::native;
use std::collections::HashMap;
use std::convert::From;
use wasm_bindgen::JsCast;

/// Events for Html<Msg>
pub struct Events<Msg> {
    pub handlers: HashMap<String, Box<FnMut(native::Event) -> Msg>>,
}

/// Props of MouseEvent
pub struct MouseEvent {
    pub alt_key: bool,
    pub buttons: u32,
    pub client_x: i32,
    pub client_y: i32,
    pub ctrl_key: bool,
    pub meta_key: bool,
    pub movement_x: i32,
    pub movement_y: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub page_x: i32,
    pub page_y: i32,
    pub screen_x: i32,
    pub screen_y: i32,
    pub shift_key: bool,
}

/// Props of DragEvent
pub struct DragEvent {}

/// Props of KeyboardEvent
pub struct KeyboardEvent {
    pub alt_key: bool,
    pub code: String,
    pub key: String,
    pub locale: String,
    pub location: u64,
    pub meta_key: bool,
    pub repeat: bool,
    pub shift_key: bool,
}

impl<Msg> Events<Msg> {
    /// Creates new empty Events
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Adds event handler
    pub fn on(
        mut self,
        type_: impl Into<String>,
        handler: impl FnMut(native::Event) -> Msg + 'static,
    ) -> Self {
        self.handlers.insert(type_.into(), Box::new(handler));
        self
    }

    pub fn on_click(self, mut handler: impl FnMut(MouseEvent) -> Msg + 'static) -> Self {
        self.on("click", move |e| {
            if let Ok(e) = e.dyn_into::<native::MouseEvent>() {
                handler(MouseEvent::from(&e))
            } else {
                handler(MouseEvent::empty())
            }
        })
    }

    pub fn on_contextmenu(self, mut handler: impl FnMut(MouseEvent) -> Msg + 'static) -> Self {
        self.on("contextmenu", move |e| {
            if let Ok(e) = e.dyn_into::<native::MouseEvent>() {
                handler(MouseEvent::from(&e))
            } else {
                handler(MouseEvent::empty())
            }
        })
    }

    pub fn on_dblclick(self, mut handler: impl FnMut(MouseEvent) -> Msg + 'static) -> Self {
        self.on("dblclick", move |e| {
            if let Ok(e) = e.dyn_into::<native::MouseEvent>() {
                handler(MouseEvent::from(&e))
            } else {
                handler(MouseEvent::empty())
            }
        })
    }

    pub fn on_drag(self, mut handler: impl FnMut(DragEvent) -> Msg + 'static) -> Self {
        self.on("drag", move |_| handler(DragEvent::empty()))
    }

    pub fn on_dragend(self, mut handler: impl FnMut(DragEvent) -> Msg + 'static) -> Self {
        self.on("dragend", move |_| handler(DragEvent::empty()))
    }

    pub fn on_dragenter(self, mut handler: impl FnMut(DragEvent) -> Msg + 'static) -> Self {
        self.on("dragenter", move |_| handler(DragEvent::empty()))
    }

    pub fn on_dragstart(self, mut handler: impl FnMut(DragEvent) -> Msg + 'static) -> Self {
        self.on("dragstart", move |_| handler(DragEvent::empty()))
    }

    pub fn on_dragleave(self, mut handler: impl FnMut(DragEvent) -> Msg + 'static) -> Self {
        self.on("dragleave", move |_| handler(DragEvent::empty()))
    }

    pub fn on_dragover(self, mut handler: impl FnMut(DragEvent) -> Msg + 'static) -> Self {
        self.on("dragover", move |_| handler(DragEvent::empty()))
    }

    pub fn on_drop(self, mut handler: impl FnMut(DragEvent) -> Msg + 'static) -> Self {
        self.on("drop", move |_| handler(DragEvent::empty()))
    }

    pub fn on_input(self, mut handler: impl FnMut(String) -> Msg + 'static) -> Self {
        self.on("input", move |e| {
            if let Ok(target) = e.target().dyn_into::<native::HTMLInputElement>() {
                handler(target.value())
            } else {
                handler("".to_string())
            }
        })
    }

    pub fn on_keydown(self, mut handler: impl FnMut(KeyboardEvent) -> Msg + 'static) -> Self {
        self.on("keydown", move |e| {
            if let Ok(e) = e.dyn_into::<native::KeyboardEvent>() {
                handler(KeyboardEvent::from(&e))
            } else {
                handler(KeyboardEvent::empty())
            }
        })
    }

    pub fn on_keypress(self, mut handler: impl FnMut(KeyboardEvent) -> Msg + 'static) -> Self {
        self.on("keypress", move |e| {
            if let Ok(e) = e.dyn_into::<native::KeyboardEvent>() {
                handler(KeyboardEvent::from(&e))
            } else {
                handler(KeyboardEvent::empty())
            }
        })
    }

    pub fn on_keyup(self, mut handler: impl FnMut(KeyboardEvent) -> Msg + 'static) -> Self {
        self.on("keyup", move |e| {
            if let Ok(e) = e.dyn_into::<native::KeyboardEvent>() {
                handler(KeyboardEvent::from(&e))
            } else {
                handler(KeyboardEvent::empty())
            }
        })
    }

    pub fn on_load(self, mut handler: impl FnMut() -> Msg + 'static) -> Self {
        self.on("load", move |_| handler())
    }

    pub fn on_mousedown(self, mut handler: impl FnMut(MouseEvent) -> Msg + 'static) -> Self {
        self.on("mousedown", move |e| {
            if let Ok(e) = e.dyn_into::<native::MouseEvent>() {
                handler(MouseEvent::from(&e))
            } else {
                handler(MouseEvent::empty())
            }
        })
    }

    pub fn on_mouseenter(self, mut handler: impl FnMut(MouseEvent) -> Msg + 'static) -> Self {
        self.on("mouseenter", move |e| {
            if let Ok(e) = e.dyn_into::<native::MouseEvent>() {
                handler(MouseEvent::from(&e))
            } else {
                handler(MouseEvent::empty())
            }
        })
    }

    pub fn on_mouseleave(self, mut handler: impl FnMut(MouseEvent) -> Msg + 'static) -> Self {
        self.on("mouseleave", move |e| {
            if let Ok(e) = e.dyn_into::<native::MouseEvent>() {
                handler(MouseEvent::from(&e))
            } else {
                handler(MouseEvent::empty())
            }
        })
    }

    pub fn on_mousemove(self, mut handler: impl FnMut(MouseEvent) -> Msg + 'static) -> Self {
        self.on("mousemove", move |e| {
            if let Ok(e) = e.dyn_into::<native::MouseEvent>() {
                handler(MouseEvent::from(&e))
            } else {
                handler(MouseEvent::empty())
            }
        })
    }

    pub fn on_mouseover(self, mut handler: impl FnMut(MouseEvent) -> Msg + 'static) -> Self {
        self.on("mouseover", move |e| {
            if let Ok(e) = e.dyn_into::<native::MouseEvent>() {
                handler(MouseEvent::from(&e))
            } else {
                handler(MouseEvent::empty())
            }
        })
    }

    pub fn on_mouseout(self, mut handler: impl FnMut(MouseEvent) -> Msg + 'static) -> Self {
        self.on("mouseout", move |e| {
            if let Ok(e) = e.dyn_into::<native::MouseEvent>() {
                handler(MouseEvent::from(&e))
            } else {
                handler(MouseEvent::empty())
            }
        })
    }

    pub fn on_mouseup(self, mut handler: impl FnMut(MouseEvent) -> Msg + 'static) -> Self {
        self.on("mouseup", move |e| {
            if let Ok(e) = e.dyn_into::<native::MouseEvent>() {
                handler(MouseEvent::from(&e))
            } else {
                handler(MouseEvent::empty())
            }
        })
    }
}

impl MouseEvent {
    fn empty() -> Self {
        MouseEvent {
            alt_key: false,
            buttons: 0,
            client_x: 0,
            client_y: 0,
            ctrl_key: false,
            meta_key: false,
            movement_x: 0,
            movement_y: 0,
            offset_x: 0,
            offset_y: 0,
            page_x: 0,
            page_y: 0,
            screen_x: 0,
            screen_y: 0,
            shift_key: false,
        }
    }
}

impl From<&native::MouseEvent> for MouseEvent {
    fn from(e: &native::MouseEvent) -> Self {
        MouseEvent {
            alt_key: e.alt_key(),
            buttons: e.buttons(),
            client_x: e.client_x(),
            client_y: e.client_y(),
            ctrl_key: e.ctrl_key(),
            meta_key: e.meta_key(),
            movement_x: e.movement_x(),
            movement_y: e.movement_y(),
            offset_x: e.offset_x(),
            offset_y: e.offset_y(),
            page_x: e.page_x(),
            page_y: e.page_y(),
            screen_x: e.screen_x(),
            screen_y: e.screen_y(),
            shift_key: e.shift_key(),
        }
    }
}

impl DragEvent {
    fn empty() -> Self {
        DragEvent {}
    }
}

impl KeyboardEvent {
    fn empty() -> Self {
        KeyboardEvent {
            alt_key: false,
            code: "".to_string(),
            key: "".to_string(),
            locale: "".to_string(),
            location: 0,
            meta_key: false,
            repeat: false,
            shift_key: false,
        }
    }
}

impl From<&native::KeyboardEvent> for KeyboardEvent {
    fn from(e: &native::KeyboardEvent) -> Self {
        KeyboardEvent {
            alt_key: e.alt_key(),
            code: e.code(),
            key: e.key(),
            locale: e.locale(),
            location: e.location(),
            meta_key: e.meta_key(),
            repeat: e.repeat(),
            shift_key: e.shift_key(),
        }
    }
}

#[cfg(test)]
mod test {}
