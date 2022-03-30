use crate::v_node::v_element::{VEvent, VEvents};
use kagura::node::{BasicNodeMsg, Msg};
use kagura::Component;
use std::collections::HashMap;
use wasm_bindgen::JsCast;

pub struct Events {
    events: VEvents,
}

impl Into<VEvents> for Events {
    fn into(self) -> VEvents {
        self.events
    }
}

impl Events {
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
        }
    }

    pub fn on<Target: Component + 'static>(
        mut self,
        type_: impl Into<String>,
        target: &Target,
        handler: impl FnOnce(VEvent) -> Target::Msg + 'static,
    ) -> Self {
        let type_ = type_.into();
        let target_id = Msg::target_id(target);
        let mut handelrs = if let Some(handlers) = self.events.remove(&type_) {
            handlers
        } else {
            vec![]
        };
        handelrs.push(Box::new(move |e| {
            let msg = handler(e);
            let msg = BasicNodeMsg::<Target>::ComponentMsg(msg);
            Msg::new(target_id, Box::new(msg))
        }));
        self.events.insert(type_, handelrs);
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
}

macro_rules! event_type {
    ($event_ty:tt as $f_name:ident) => {
        pub fn $f_name<Target: Component + 'static>(
            self,
            target: &Target,
            handler: impl FnOnce(VEvent) -> Target::Msg + 'static,
        ) -> Self {
            self.on($event_ty, target, handler)
        }
    };
}

impl Events {
    event_type!("load" as on_load);

    event_type!("drag" as on_drag);
    event_type!("dragend" as on_dragend);
    event_type!("dragenter" as on_dragenter);
    event_type!("dragstart" as on_dragstart);
    event_type!("dragleave" as on_dragleave);
    event_type!("dragover" as on_dragover);
    event_type!("drop" as on_drop);

    event_type!("click" as on_click);
    event_type!("dblclick" as on_dblclick);
    event_type!("mousedown" as on_mousedown);
    event_type!("mouseenter" as on_mouseenter);
    event_type!("mouseleave" as on_mouseleave);
    event_type!("mousemove" as on_mousemove);
    event_type!("mouseover" as on_mouseover);
    event_type!("mouseout" as on_mouseout);
    event_type!("mouseup" as on_mouseup);

    event_type!("keydown" as on_keydown);
    event_type!("keypress" as on_keypress);
    event_type!("keyup" as on_keyup);
}
