use std::collections::HashMap;
use std::convert::From;
use wasm_bindgen::JsCast;
use web_sys;

/// Events for Html<Msg>
pub struct Events<Msg> {
    pub handlers: HashMap<String, Box<FnMut(web_sys::Event) -> Msg>>,
}

/// Props of MouseEvent
pub struct MouseEvent {
    implement: Option<web_sys::MouseEvent>,
}

/// Props of DragEvent
pub struct DragEvent {
    implement: Option<web_sys::DragEvent>,
}

/// Props of KeyboardEvent
pub struct KeyboardEvent {
    implement: Option<web_sys::KeyboardEvent>,
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
        handler: impl FnMut(web_sys::Event) -> Msg + 'static,
    ) -> Self {
        self.handlers.insert(type_.into(), Box::new(handler));
        self
    }

    pub fn on_click(self, mut handler: impl FnMut(MouseEvent) -> Msg + 'static) -> Self {
        self.on("click", move |e| handler(MouseEvent::from(e)))
    }

    pub fn on_contextmenu(self, mut handler: impl FnMut(MouseEvent) -> Msg + 'static) -> Self {
        self.on("contextmenu", move |e| handler(MouseEvent::from(e)))
    }

    pub fn on_dblclick(self, mut handler: impl FnMut(MouseEvent) -> Msg + 'static) -> Self {
        self.on("dblclick", move |e| handler(MouseEvent::from(e)))
    }

    pub fn on_drag(self, mut handler: impl FnMut(DragEvent) -> Msg + 'static) -> Self {
        self.on("drag", move |e| handler(DragEvent::from(e)))
    }

    pub fn on_dragend(self, mut handler: impl FnMut(DragEvent) -> Msg + 'static) -> Self {
        self.on("dragend", move |e| handler(DragEvent::from(e)))
    }

    pub fn on_dragenter(self, mut handler: impl FnMut(DragEvent) -> Msg + 'static) -> Self {
        self.on("dragenter", move |e| handler(DragEvent::from(e)))
    }

    pub fn on_dragstart(self, mut handler: impl FnMut(DragEvent) -> Msg + 'static) -> Self {
        self.on("dragstart", move |e| handler(DragEvent::from(e)))
    }

    pub fn on_dragleave(self, mut handler: impl FnMut(DragEvent) -> Msg + 'static) -> Self {
        self.on("dragleave", move |e| handler(DragEvent::from(e)))
    }

    pub fn on_dragover(self, mut handler: impl FnMut(DragEvent) -> Msg + 'static) -> Self {
        self.on("dragover", move |e| handler(DragEvent::from(e)))
    }

    pub fn on_drop(self, mut handler: impl FnMut(DragEvent) -> Msg + 'static) -> Self {
        self.on("drop", move |e| handler(DragEvent::from(e)))
    }

    pub fn on_input(self, mut handler: impl FnMut(String) -> Msg + 'static) -> Self {
        self.on("input", move |e| {
            if let Some(Ok(target)) = e
                .target()
                .map(|target| target.dyn_into::<web_sys::HtmlInputElement>())
            {
                handler(target.value())
            } else {
                handler("".to_string())
            }
        })
    }

    pub fn on_keydown(self, mut handler: impl FnMut(KeyboardEvent) -> Msg + 'static) -> Self {
        self.on("keydown", move |e| handler(KeyboardEvent::from(e)))
    }

    pub fn on_keypress(self, mut handler: impl FnMut(KeyboardEvent) -> Msg + 'static) -> Self {
        self.on("keypress", move |e| handler(KeyboardEvent::from(e)))
    }

    pub fn on_keyup(self, mut handler: impl FnMut(KeyboardEvent) -> Msg + 'static) -> Self {
        self.on("keyup", move |e| handler(KeyboardEvent::from(e)))
    }

    pub fn on_load(self, mut handler: impl FnMut() -> Msg + 'static) -> Self {
        self.on("load", move |_| handler())
    }

    pub fn on_mousedown(self, mut handler: impl FnMut(MouseEvent) -> Msg + 'static) -> Self {
        self.on("mousedown", move |e| handler(MouseEvent::from(e)))
    }

    pub fn on_mouseenter(self, mut handler: impl FnMut(MouseEvent) -> Msg + 'static) -> Self {
        self.on("mouseenter", move |e| handler(MouseEvent::from(e)))
    }

    pub fn on_mouseleave(self, mut handler: impl FnMut(MouseEvent) -> Msg + 'static) -> Self {
        self.on("mouseleave", move |e| handler(MouseEvent::from(e)))
    }

    pub fn on_mousemove(self, mut handler: impl FnMut(MouseEvent) -> Msg + 'static) -> Self {
        self.on("mousemove", move |e| handler(MouseEvent::from(e)))
    }

    pub fn on_mouseover(self, mut handler: impl FnMut(MouseEvent) -> Msg + 'static) -> Self {
        self.on("mouseover", move |e| handler(MouseEvent::from(e)))
    }

    pub fn on_mouseout(self, mut handler: impl FnMut(MouseEvent) -> Msg + 'static) -> Self {
        self.on("mouseout", move |e| handler(MouseEvent::from(e)))
    }

    pub fn on_mouseup(self, mut handler: impl FnMut(MouseEvent) -> Msg + 'static) -> Self {
        self.on("mouseup", move |e| handler(MouseEvent::from(e)))
    }
}

impl From<web_sys::Event> for MouseEvent {
    fn from(e: web_sys::Event) -> Self {
        if let Ok(e) = e.dyn_into::<web_sys::MouseEvent>() {
            Self { implement: Some(e) }
        } else {
            Self { implement: None }
        }
    }
}

impl From<web_sys::Event> for DragEvent {
    fn from(e: web_sys::Event) -> Self {
        if let Ok(e) = e.dyn_into::<web_sys::DragEvent>() {
            Self { implement: Some(e) }
        } else {
            Self { implement: None }
        }
    }
}

impl From<web_sys::Event> for KeyboardEvent {
    fn from(e: web_sys::Event) -> Self {
        if let Ok(e) = e.dyn_into::<web_sys::KeyboardEvent>() {
            Self { implement: Some(e) }
        } else {
            Self { implement: None }
        }
    }
}
