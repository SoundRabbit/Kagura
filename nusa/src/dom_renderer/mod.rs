use crate::html::Html;
use crate::v_node::v_element::{VAttributeValues, VAttributes, VEvent, VEventHandler, VEvents};
use crate::v_node::{VElement, VText};
use crate::VNode;
use kagura::component::Render;
use kagura::node::Msg;
use std::cell::Cell;
use std::collections::{HashMap, VecDeque};
use std::pin::Pin;
use std::rc::Rc;
use wasm_bindgen::JsCast;

pub type VEventListener = Box<dyn FnOnce(web_sys::Event) -> (bool, VecDeque<Msg>)>;
pub type VEventListeners = HashMap<String, VEventListener>;

pub struct DomRenderer {
    root: web_sys::Node,
    prevs: VecDeque<VNode>,
    document: web_sys::Document,
}

impl DomRenderer {
    pub fn new(root: web_sys::Node) -> Self {
        Self {
            root,
            prevs: VecDeque::new(),
            document: web_sys::window().unwrap().document().unwrap(),
        }
    }

    pub fn render(&mut self, nows: VecDeque<VNode>) -> VEventListeners {
        let mut prevs = VecDeque::new();
        std::mem::swap(&mut self.prevs, &mut prevs);
        let (rendered, event_listeners) = self.render_nodes(prevs, nows, &self.root);
        self.prevs = rendered;
        event_listeners
    }

    fn render_nodes(
        &self,
        prevs: VecDeque<VNode>,
        nows: VecDeque<VNode>,
        raw_parent: &web_sys::Node,
    ) -> (VecDeque<VNode>, VEventListeners) {
        let mixeds = crate::util::mix(prevs, nows, Self::compare_nodes, 5.0, 10.0, 1.0);
        let mut raws = {
            let raws = raw_parent.child_nodes();
            let mut buf = VecDeque::new();
            let raws_len = raws.length();
            for i in 0..raws_len {
                if let Some(raw) = raws.get(i) {
                    buf.push_back(raw);
                }
            }
            buf
        };

        let (rendered, events) = mixeds.into_iter().fold(
            (VecDeque::new(), HashMap::new()),
            |(mut rendered, mut events), mixed| {
                match mixed {
                    crate::util::mix::Edit::Append(now) => {
                        let now_rendered = now.as_rendered();
                        let event_lsiteners = self.append_node(now, &raw_parent, raws.front());
                        Self::append_events(&mut events, event_lsiteners);
                        rendered.push_back(now_rendered);
                    }
                    crate::util::mix::Edit::Keep(prev, now) => {
                        if let Some(raw) = raws.pop_front() {
                            let now_rendered = now.as_rendered();
                            let event_lsiteners = self.keep_node(prev, now, &raw);
                            rendered.push_back(now_rendered);
                        }
                    }
                    crate::util::mix::Edit::Remove(..) => {
                        if let Some(raw_remove) = raws.pop_front() {
                            raw_parent.remove_child(&raw_remove);
                        }
                    }
                    crate::util::mix::Edit::Replace(_, now) => {
                        if let Some(raw) = raws.pop_front() {
                            let now_rendered = now.as_rendered();
                            let event_lsiteners = self.replace_node(now, &raw_parent, &raw);
                            rendered.push_back(now_rendered);
                        }
                    }
                }

                (rendered, events)
            },
        );

        let event_listeners = events.into_iter().fold(
            HashMap::new(),
            |mut event_listeners, (event_type, event_listener_list)| {
                event_listeners.insert(
                    event_type,
                    Box::new(move |e: web_sys::Event| {
                        let mut msgs = VecDeque::new();
                        let mut stop_propagation = false;
                        for event_listener in event_listener_list {
                            let mut res = event_listener(e.clone());
                            stop_propagation = stop_propagation | res.0;
                            msgs.append(&mut res.1);
                        }
                        (stop_propagation, msgs)
                    }) as VEventListener,
                );
                event_listeners
            },
        );

        (rendered, event_listeners)
    }

    fn append_events(
        events: &mut HashMap<String, Vec<VEventListener>>,
        event_listeners: VEventListeners,
    ) {
        for (event_type, event_listener) in event_listeners {
            if let Some(event_listener_list) = events.get_mut(&event_type) {
                event_listener_list.push(event_listener);
            } else {
                events.insert(event_type, vec![event_listener]);
            }
        }
    }

    fn compare_nodes(prev: &VNode, now: &VNode) -> bool {
        match (prev, now) {
            (VNode::VElement(prev), VNode::VElement(now)) => {
                prev.tag_name == now.tag_name && prev.index_id == now.index_id
            }
            (VNode::VText(..), VNode::VText(..)) => true,
            _ => false,
        }
    }

    fn append_node(
        &self,
        now: VNode,
        raw_parent: &web_sys::Node,
        raw_after: Option<&web_sys::Node>,
    ) -> VEventListeners {
        let (event_listeners, raw) = match now {
            VNode::VElement(now) => self.create_element(now),
            VNode::VText(now) => {
                let raw = self.document.create_text_node(&now.text);
                (HashMap::new(), raw.into())
            }
        };

        if let Some(raw_after) = raw_after {
            raw_parent.insert_before(raw_after, Some(&raw));
        } else {
            raw_parent.append_child(&raw);
        }

        event_listeners
    }

    fn keep_node(&self, prev: VNode, now: VNode, raw: &web_sys::Node) -> VEventListeners {
        match (prev, now) {
            (VNode::VElement(prev), VNode::VElement(now)) => self.keep_element(prev, now, raw),
            (VNode::VText(prev), VNode::VText(now)) => self.keep_text(prev, now, raw),
            _ => HashMap::new(),
        }
    }

    fn replace_node(
        &self,
        now: VNode,
        raw_parent: &web_sys::Node,
        prev_raw: &web_sys::Node,
    ) -> VEventListeners {
        let (event_listeners, now_raw) = match now {
            VNode::VElement(now) => self.create_element(now),
            VNode::VText(now) => {
                let raw = self.document.create_text_node(&now.text);
                (HashMap::new(), raw.into())
            }
        };

        let _ = raw_parent.replace_child(&now_raw, &prev_raw);

        event_listeners
    }

    fn keep_element(&self, prev: VElement, now: VElement, raw: &web_sys::Node) -> VEventListeners {
        if let Some(raw) = raw.dyn_ref::<web_sys::Element>() {
            let (children, child_event_listeners) =
                self.render_nodes(prev.children, now.children, &raw);

            Self::update_attributes(&prev.attributes, &now.attributes, &raw);

            let event_listeners =
                Self::create_event_listeners(now.events, child_event_listeners, raw);

            event_listeners
        } else {
            HashMap::new()
        }
    }

    fn update_attributes(prev: &VAttributes, now: &VAttributes, raw: &web_sys::Element) {
        for (attr_name, now_values) in now {
            if let Some(prev_values) = prev.get(attr_name) {
                if *prev_values != *now_values {
                    Self::set_attribute(attr_name, &now_values, &raw);
                }
            } else {
                Self::set_attribute(attr_name, &now_values, &raw);
            }
        }

        for (attr_name, _) in prev {
            if !now.contains_key(attr_name) {
                let _ = raw.remove_attribute(attr_name);
            }
        }
    }

    fn set_attribute(attr_name: &String, now: &VAttributeValues, raw: &web_sys::Element) {
        if attr_name == "value" {
            if let Some(raw) = raw.dyn_ref::<web_sys::HtmlInputElement>() {
                raw.set_value(now.to_string().as_str());
            } else if let Some(raw) = raw.dyn_ref::<web_sys::HtmlTextAreaElement>() {
                raw.set_value(now.to_string().as_str());
            } else {
                let _ = raw.set_attribute("value", now.to_string().as_str());
            }
        } else {
            let _ = raw.set_attribute(attr_name, now.to_string().as_str());
        }
    }

    fn keep_text(&self, prev: VText, now: VText, raw: &web_sys::Node) -> VEventListeners {
        if let Some(raw) = raw.dyn_ref::<web_sys::CharacterData>() {
            raw.replace_data(0, prev.text.len() as u32, &now.text);
        }
        HashMap::new()
    }

    fn create_element(&self, now: VElement) -> (VEventListeners, web_sys::Node) {
        let raw = self.document.create_element(&now.tag_name).unwrap();

        let (children, child_event_listeners) =
            self.render_nodes(VecDeque::new(), now.children, &raw);

        for (attr_name, attr_values) in now.attributes {
            Self::set_attribute(&attr_name, &attr_values, &raw);
        }

        let event_listeners = Self::create_event_listeners(now.events, child_event_listeners, &raw);

        (event_listeners, raw.into())
    }

    fn create_event_listeners(
        events: VEvents,
        mut child_event_listeners: VEventListeners,
        raw: &web_sys::Node,
    ) -> VEventListeners {
        let mut event_listeners = HashMap::new();
        for (event_type, event_handlers) in events {
            if let Some(child_event_listener) = child_event_listeners.remove(&event_type) {
                let raw = raw.clone();
                event_listeners.insert(
                    event_type,
                    Box::new(move |e: web_sys::Event| {
                        if raw.contains(
                            e.target()
                                .as_ref()
                                .and_then(|target| target.dyn_ref::<web_sys::Node>()),
                        ) {
                            let (stop_propagation, mut msgs) = child_event_listener(e.clone());

                            if !stop_propagation {
                                let (stop_propagation, mut additional_msgs) =
                                    Self::attach_events(&e, event_handlers);
                                msgs.append(&mut additional_msgs);
                                (stop_propagation, msgs)
                            } else {
                                (stop_propagation, msgs)
                            }
                        } else {
                            (false, VecDeque::new())
                        }
                    }) as VEventListener,
                );
            } else {
                let raw = raw.clone();
                event_listeners.insert(
                    event_type,
                    Box::new(move |e: web_sys::Event| {
                        if raw.contains(
                            e.target()
                                .as_ref()
                                .and_then(|target| target.dyn_ref::<web_sys::Node>()),
                        ) {
                            Self::attach_events(&e, event_handlers)
                        } else {
                            (false, VecDeque::new())
                        }
                    }) as VEventListener,
                );
            }
        }

        for (event_type, event_listener) in child_event_listeners {
            event_listeners.insert(event_type, event_listener);
        }

        event_listeners
    }

    fn attach_events(
        e: &web_sys::Event,
        event_handlers: Vec<VEventHandler>,
    ) -> (bool, VecDeque<Msg>) {
        let mut msgs = VecDeque::new();
        let stop_propagation = Rc::new(Cell::new(false));
        for event_handler in event_handlers {
            let v_event = VEvent::new(e.clone(), Rc::clone(&stop_propagation));
            let msg = event_handler(v_event);
            msgs.push_back(msg);
        }
        (stop_propagation.get(), msgs)
    }
}
