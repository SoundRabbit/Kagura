use super::*;
use wasm_bindgen::JsCast;
use web_sys;

impl<Msg> Events<Msg> {
    /// Creates new empty Events
    pub fn new() -> Self {
        Self {
            handler_table: HashMap::new(),
        }
    }

    /// Adds event handler
    pub fn on(
        mut self,
        type_: impl Into<String>,
        handler: impl FnOnce(web_sys::Event) -> Msg + 'static,
    ) -> Self {
        let type_ = type_.into();
        if let Some(handlers) = self.handler_table.get_mut(&type_) {
            handlers.push(EventHandler::Unrwapped(Box::new(move |e| handler(e))));
        } else {
            self.handler_table.insert(
                type_,
                vec![EventHandler::Unrwapped(Box::new(move |e| handler(e)))],
            );
        }
        self
    }

    fn mouse_event(e: web_sys::Event, type_arg: &str) -> web_sys::MouseEvent {
        if let Ok(e) = e.dyn_into::<web_sys::MouseEvent>() {
            e
        } else {
            web_sys::MouseEvent::new(type_arg).unwrap()
        }
    }

    fn drag_event(e: web_sys::Event, type_: &str) -> web_sys::DragEvent {
        if let Ok(e) = e.dyn_into::<web_sys::DragEvent>() {
            e
        } else {
            web_sys::DragEvent::new(type_).unwrap()
        }
    }

    fn keyboard_event(e: web_sys::Event, type_arg: &str) -> web_sys::KeyboardEvent {
        if let Ok(e) = e.dyn_into::<web_sys::KeyboardEvent>() {
            e
        } else {
            web_sys::KeyboardEvent::new(type_arg).unwrap()
        }
    }

    pub fn on_click(self, handler: impl FnOnce(web_sys::MouseEvent) -> Msg + 'static) -> Self {
        self.on("click", |e| handler(Self::mouse_event(e, "click")))
    }

    pub fn on_contextmenu(
        self,
        handler: impl FnOnce(web_sys::MouseEvent) -> Msg + 'static,
    ) -> Self {
        self.on("contextmenu", |e| {
            handler(Self::mouse_event(e, "contextmenu"))
        })
    }

    pub fn on_dblclick(self, handler: impl FnOnce(web_sys::MouseEvent) -> Msg + 'static) -> Self {
        self.on("dblclick", |e| handler(Self::mouse_event(e, "dblclick")))
    }

    pub fn on_drag(self, handler: impl FnOnce(web_sys::DragEvent) -> Msg + 'static) -> Self {
        self.on("drag", |e| handler(Self::drag_event(e, "drag")))
    }

    pub fn on_dragend(self, handler: impl FnOnce(web_sys::DragEvent) -> Msg + 'static) -> Self {
        self.on("dragend", |e| handler(Self::drag_event(e, "dragend")))
    }

    pub fn on_dragenter(self, handler: impl FnOnce(web_sys::DragEvent) -> Msg + 'static) -> Self {
        self.on("dragenter", |e| handler(Self::drag_event(e, "dragenter")))
    }

    pub fn on_dragstart(self, handler: impl FnOnce(web_sys::DragEvent) -> Msg + 'static) -> Self {
        self.on("dragstart", |e| handler(Self::drag_event(e, "dragstart")))
    }

    pub fn on_dragleave(self, handler: impl FnOnce(web_sys::DragEvent) -> Msg + 'static) -> Self {
        self.on("dragleave", |e| handler(Self::drag_event(e, "dragleave")))
    }

    pub fn on_dragover(self, handler: impl FnOnce(web_sys::DragEvent) -> Msg + 'static) -> Self {
        self.on("dragover", |e| handler(Self::drag_event(e, "dragover")))
    }

    pub fn on_drop(self, handler: impl FnOnce(web_sys::DragEvent) -> Msg + 'static) -> Self {
        self.on("drop", |e| handler(Self::drag_event(e, "drop")))
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

    pub fn on_keydown(self, handler: impl FnOnce(web_sys::KeyboardEvent) -> Msg + 'static) -> Self {
        self.on("keydown", |e| handler(Self::keyboard_event(e, "keydown")))
    }

    pub fn on_keypress(
        self,
        handler: impl FnOnce(web_sys::KeyboardEvent) -> Msg + 'static,
    ) -> Self {
        self.on("keypress", |e| handler(Self::keyboard_event(e, "keypress")))
    }

    pub fn on_keyup(self, handler: impl FnOnce(web_sys::KeyboardEvent) -> Msg + 'static) -> Self {
        self.on("keyup", |e| handler(Self::keyboard_event(e, "keyup")))
    }

    pub fn on_load(self, handler: impl FnOnce() -> Msg + 'static) -> Self {
        self.on("load", |_| handler())
    }

    pub fn on_mousedown(self, handler: impl FnOnce(web_sys::MouseEvent) -> Msg + 'static) -> Self {
        self.on("mousedown", |e| handler(Self::mouse_event(e, "mousedown")))
    }

    pub fn on_mouseenter(self, handler: impl FnOnce(web_sys::MouseEvent) -> Msg + 'static) -> Self {
        self.on("mouseenter", |e| {
            handler(Self::mouse_event(e, "mouseenter"))
        })
    }

    pub fn on_mouseleave(self, handler: impl FnOnce(web_sys::MouseEvent) -> Msg + 'static) -> Self {
        self.on("mouseleave", |e| {
            handler(Self::mouse_event(e, "mouseleave"))
        })
    }

    pub fn on_mousemove(self, handler: impl FnOnce(web_sys::MouseEvent) -> Msg + 'static) -> Self {
        self.on("mousemove", |e| handler(Self::mouse_event(e, "mousemove")))
    }

    pub fn on_mouseover(self, handler: impl FnOnce(web_sys::MouseEvent) -> Msg + 'static) -> Self {
        self.on("mouseover", |e| handler(Self::mouse_event(e, "mouseover")))
    }

    pub fn on_mouseout(self, handler: impl FnOnce(web_sys::MouseEvent) -> Msg + 'static) -> Self {
        self.on("mouseout", |e| handler(Self::mouse_event(e, "mouseout")))
    }

    pub fn on_mouseup(self, handler: impl FnOnce(web_sys::MouseEvent) -> Msg + 'static) -> Self {
        self.on("mouseup", |e| handler(Self::mouse_event(e, "mouseup")))
    }
}
