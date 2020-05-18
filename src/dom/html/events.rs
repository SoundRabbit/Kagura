use std::collections::HashMap;
use std::convert::From;
use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys;

/// Events for Html<Msg>
pub struct Events<Msg> {
    pub handlers: HashMap<String, Box<dyn FnOnce(web_sys::Event) -> Msg>>,
}

/// Props of MouseEvent
pub struct MouseEvent {
    implement: Option<web_sys::MouseEvent>,
}

/// Props of DragEvent
pub struct DragEvent {
    #[allow(dead_code)]
    implement: Option<web_sys::DragEvent>,
}

/// Props of KeyboardEvent
pub struct KeyboardEvent {
    implement: Option<web_sys::KeyboardEvent>,
}

impl<Msg> Clone for Events<Msg> {
    fn clone(&self) -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }
}

impl Deref for MouseEvent {
    type Target = web_sys::MouseEvent;
    fn deref(&self) -> &web_sys::MouseEvent {
        if let Some(implement) = &self.implement {
            implement
        } else {
            panic!("no event object");
        }
    }
}

impl Deref for KeyboardEvent {
    type Target = web_sys::KeyboardEvent;
    fn deref(&self) -> &web_sys::KeyboardEvent {
        if let Some(implement) = &self.implement {
            implement
        } else {
            panic!("no event object");
        }
    }
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
        handler: impl FnOnce(web_sys::Event) -> Msg + 'static,
    ) -> Self {
        self.handlers.insert(type_.into(), Box::new(handler));
        self
    }

    pub fn on_click(self, handler: impl FnOnce(MouseEvent) -> Msg + 'static) -> Self {
        self.on("click", |e| handler(MouseEvent::from(e)))
    }

    pub fn on_contextmenu(self, handler: impl FnOnce(MouseEvent) -> Msg + 'static) -> Self {
        self.on("contextmenu", |e| handler(MouseEvent::from(e)))
    }

    pub fn on_dblclick(self, handler: impl FnOnce(MouseEvent) -> Msg + 'static) -> Self {
        self.on("dblclick", |e| handler(MouseEvent::from(e)))
    }

    pub fn on_drag(self, handler: impl FnOnce(DragEvent) -> Msg + 'static) -> Self {
        self.on("drag", |e| handler(DragEvent::from(e)))
    }

    pub fn on_dragend(self, handler: impl FnOnce(DragEvent) -> Msg + 'static) -> Self {
        self.on("dragend", |e| handler(DragEvent::from(e)))
    }

    pub fn on_dragenter(self, handler: impl FnOnce(DragEvent) -> Msg + 'static) -> Self {
        self.on("dragenter", |e| handler(DragEvent::from(e)))
    }

    pub fn on_dragstart(self, handler: impl FnOnce(DragEvent) -> Msg + 'static) -> Self {
        self.on("dragstart", |e| handler(DragEvent::from(e)))
    }

    pub fn on_dragleave(self, handler: impl FnOnce(DragEvent) -> Msg + 'static) -> Self {
        self.on("dragleave", |e| handler(DragEvent::from(e)))
    }

    pub fn on_dragover(self, handler: impl FnOnce(DragEvent) -> Msg + 'static) -> Self {
        self.on("dragover", |e| handler(DragEvent::from(e)))
    }

    pub fn on_drop(self, handler: impl FnOnce(DragEvent) -> Msg + 'static) -> Self {
        self.on("drop", |e| handler(DragEvent::from(e)))
    }

    pub fn on_input(self, handler: impl FnOnce(String) -> Msg + 'static) -> Self {
        self.on("input", |e| {
            if let Some(target) = e.target() {
                match target.dyn_into::<web_sys::HtmlInputElement>() {
                    Ok(target) => handler(target.value()),
                    Err(target) => match target.dyn_into::<web_sys::HtmlTextAreaElement>() {
                        Ok(target) => handler(target.value()),
                        Err(_) => handler(String::new()),
                    },
                }
            } else {
                handler(String::new())
            }
        })
    }

    pub fn on_keydown(self, handler: impl FnOnce(KeyboardEvent) -> Msg + 'static) -> Self {
        self.on("keydown", |e| handler(KeyboardEvent::from(e)))
    }

    pub fn on_keypress(self, handler: impl FnOnce(KeyboardEvent) -> Msg + 'static) -> Self {
        self.on("keypress", |e| handler(KeyboardEvent::from(e)))
    }

    pub fn on_keyup(self, handler: impl FnOnce(KeyboardEvent) -> Msg + 'static) -> Self {
        self.on("keyup", |e| handler(KeyboardEvent::from(e)))
    }

    pub fn on_load(self, handler: impl FnOnce() -> Msg + 'static) -> Self {
        self.on("load", |_| handler())
    }

    pub fn on_mousedown(self, handler: impl FnOnce(MouseEvent) -> Msg + 'static) -> Self {
        self.on("mousedown", |e| handler(MouseEvent::from(e)))
    }

    pub fn on_mouseenter(self, handler: impl FnOnce(MouseEvent) -> Msg + 'static) -> Self {
        self.on("mouseenter", |e| handler(MouseEvent::from(e)))
    }

    pub fn on_mouseleave(self, handler: impl FnOnce(MouseEvent) -> Msg + 'static) -> Self {
        self.on("mouseleave", |e| handler(MouseEvent::from(e)))
    }

    pub fn on_mousemove(self, handler: impl FnOnce(MouseEvent) -> Msg + 'static) -> Self {
        self.on("mousemove", |e| handler(MouseEvent::from(e)))
    }

    pub fn on_mouseover(self, handler: impl FnOnce(MouseEvent) -> Msg + 'static) -> Self {
        self.on("mouseover", |e| handler(MouseEvent::from(e)))
    }

    pub fn on_mouseout(self, handler: impl FnOnce(MouseEvent) -> Msg + 'static) -> Self {
        self.on("mouseout", |e| handler(MouseEvent::from(e)))
    }

    pub fn on_mouseup(self, handler: impl FnOnce(MouseEvent) -> Msg + 'static) -> Self {
        self.on("mouseup", |e| handler(MouseEvent::from(e)))
    }
}

impl MouseEvent {
    fn prop<T>(&self, default: T, mapper: impl FnOnce(&web_sys::MouseEvent) -> T) -> T {
        if let Some(e) = &self.implement {
            mapper(e)
        } else {
            default
        }
    }

    pub fn screen_x(&self) -> i32 {
        self.prop(0, |e| e.screen_x())
    }

    pub fn screen_y(&self) -> i32 {
        self.prop(0, |e| e.screen_y())
    }

    pub fn client_x(&self) -> i32 {
        self.prop(0, |e| e.client_x())
    }

    pub fn client_y(&self) -> i32 {
        self.prop(0, |e| e.client_y())
    }

    pub fn x(&self) -> i32 {
        self.prop(0, |e| e.x())
    }

    pub fn y(&self) -> i32 {
        self.prop(0, |e| e.y())
    }

    pub fn offset_x(&self) -> i32 {
        self.prop(0, |e| e.offset_x())
    }

    pub fn offset_y(&self) -> i32 {
        self.prop(0, |e| e.offset_y())
    }

    pub fn ctrl_key(&self) -> bool {
        self.prop(false, |e| e.ctrl_key())
    }

    pub fn shift_key(&self) -> bool {
        self.prop(false, |e| e.shift_key())
    }

    pub fn meta_key(&self) -> bool {
        self.prop(false, |e| e.meta_key())
    }

    pub fn button(&self) -> i16 {
        self.prop(0, |e| e.button())
    }

    pub fn buttons(&self) -> u16 {
        self.prop(0, |e| e.buttons())
    }

    pub fn movement_x(&self) -> i32 {
        self.prop(0, |e| e.movement_x())
    }

    pub fn movement_y(&self) -> i32 {
        self.prop(0, |e| e.movement_y())
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

impl KeyboardEvent {
    fn prop<T>(&self, default: T, mapper: impl FnOnce(&web_sys::KeyboardEvent) -> T) -> T {
        if let Some(e) = &self.implement {
            mapper(e)
        } else {
            default
        }
    }

    pub fn char_code(&self) -> u32 {
        self.prop(0, |e| e.char_code())
    }

    pub fn key_code(&self) -> u32 {
        self.prop(0, |e| e.key_code())
    }

    pub fn alt_key(&self) -> bool {
        self.prop(false, |e| e.alt_key())
    }

    pub fn ctrl_key(&self) -> bool {
        self.prop(false, |e| e.ctrl_key())
    }

    pub fn shift_key(&self) -> bool {
        self.prop(false, |e| e.shift_key())
    }

    pub fn meta_key(&self) -> bool {
        self.prop(false, |e| e.meta_key())
    }

    pub fn location(&self) -> u32 {
        self.prop(0, |e| e.location())
    }

    pub fn repeat(&self) -> bool {
        self.prop(false, |e| e.repeat())
    }

    pub fn key(&self) -> String {
        self.prop(String::from(""), |e| e.key())
    }

    pub fn code(&self) -> String {
        self.prop(String::from(""), |e| e.code())
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
