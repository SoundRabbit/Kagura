use kagura::node::{BasicNodeMsg, Msg};
use std::collections::HashMap;
use wasm_bindgen::JsCast;

pub struct Events {
    handler_table: HashMap<String, Vec<Box<dyn FnOnce(web_sys::Event) -> Msg>>>,
}

use kagura::Component;

macro_rules! event_type {
    ($event_ty:tt as $f_name:ident) => {
        pub fn $f_name<Target: Component + 'static>(
            self,
            target: &Target,
            handler: impl FnOnce(web_sys::Event) -> Target::Msg + 'static,
        ) -> Self {
            self.on($event_ty, target, handler)
        }
    };

    ($event_ty:tt : $ty:ident as $f_name:ident) => {
        pub fn $f_name<Target: Component + 'static>(
            self,
            target: &Target,
            handler: impl FnOnce(web_sys::$ty) -> Target::Msg + 'static,
        ) -> Self {
            let event_ty = $event_ty;
            self.on(event_ty, target, move |e| {
                if let Ok(e) = e.dyn_into::<web_sys::$ty>() {
                    handler(e)
                } else {
                    let e = web_sys::$ty::new(&event_ty).unwrap();
                    handler(e)
                }
            })
        }
    };
}

impl Events {
    pub fn new() -> Self {
        Self {
            handler_table: HashMap::new(),
        }
    }

    pub fn on<Target: Component + 'static>(
        mut self,
        type_: impl Into<String>,
        target: &Target,
        handler: impl FnOnce(web_sys::Event) -> Target::Msg + 'static,
    ) -> Self {
        let type_ = type_.into();
        let target_id = Msg::target_id(target);
        let mut handelrs = if let Some(handlers) = self.handler_table.remove(&type_) {
            handlers
        } else {
            vec![]
        };
        handelrs.push(Box::new(move |e| {
            let msg = handler(e);
            let msg = BasicNodeMsg::<Target>::ComponentMsg(msg);
            Msg::new(target_id, Box::new(msg))
        }));
        self.handler_table.insert(type_, handelrs);
        self
    }

    pub fn on_input<Target: Component + 'static>(
        self,
        target: &Target,
        handler: impl FnOnce(String) -> Target::Msg + 'static,
    ) -> Self {
        self.on("input", target, move |e| {
            if let Some(target) = e.target() {
                if let Some(target) = target.dyn_ref::<web_sys::HtmlInputElement>() {
                    return handler(target.value());
                } else if let Some(target) = target.dyn_ref::<web_sys::HtmlTextAreaElement>() {
                    return handler(target.value());
                }
            }
            handler(String::new())
        })
    }

    event_type!("load" as on_load);

    event_type!("drag": DragEvent as on_drag);
    event_type!("dragend": DragEvent as on_dragend);
    event_type!("dragenter": DragEvent as on_dragenter);
    event_type!("dragstart": DragEvent as on_dragstart);
    event_type!("dragleave": DragEvent as on_dragleave);
    event_type!("dragover": DragEvent as on_dragover);
    event_type!("drop": DragEvent as on_drop);

    event_type!("click": MouseEvent as on_click);
    event_type!("dblclick": MouseEvent as on_dblclick);
    event_type!("mousedown": MouseEvent as on_mousedown);
    event_type!("mouseenter": MouseEvent as on_mouseenter);
    event_type!("mouseleave": MouseEvent as on_mouseleave);
    event_type!("mousemove": MouseEvent as on_mousemove);
    event_type!("mouseover": MouseEvent as on_mouseover);
    event_type!("mouseout": MouseEvent as on_mouseout);
    event_type!("mouseup": MouseEvent as on_mouseup);

    event_type!("keydown": KeyboardEvent as on_keydown);
    event_type!("keypress": KeyboardEvent as on_keypress);
    event_type!("keyup": KeyboardEvent as on_keyup);
}
