pub mod event;

use crate::dom;
use crate::native;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;

pub struct Renderer {
    before: dom::Node,
    root: web_sys::Node,
}

impl Renderer {
    pub fn new(virtual_node: dom::Node, root_node: web_sys::Node) -> Self {
        let root: web_sys::Node;
        if let Some(node) = render(&virtual_node, None, Some(&root_node)) {
            root = node;
        } else {
            root = native::create_text_node("").into();
        }
        let _ = root_node
            .parent_node()
            .expect("no parent node of root")
            .replace_child(&root, &root_node);
        Self {
            before: virtual_node,
            root,
        }
    }

    pub fn update(&mut self, after: dom::Node) {
        if let Some(root) = render(&after, Some(&self.before), Some(&self.root)) {
            let _ = self
                .root
                .parent_node()
                .expect("no parent node of root")
                .replace_child(&root, &self.root);
            self.before = after;
            self.root = root;
        }
    }
}

fn set_attribute_all(element: &web_sys::Element, attributes: &dom::Attributes) {
    for (a, v) in &attributes.attributes {
        if v.is_empty() {
            let _ = element.set_attribute(a, "");
        } else if let Some(d) = attributes.delimiters.get(a) {
            set_attribute_set(element, a, v, d);
        } else {
            set_attribute_set(element, a, v, "");
        }
    }
}

fn set_attribute_set(element: &web_sys::Element, a: &str, v: &HashSet<dom::Value>, d: &str) {
    let v = v.iter().map(|v| v.into()).collect::<Vec<String>>();
    let v = v.iter().map(|v| &v as &str).collect::<Vec<&str>>().join(d);
    if String::from("value") == String::from(a) {
        if let Some(element) = element.dyn_ref::<web_sys::HtmlInputElement>() {
            element.set_value(&v);
        } else {
            let _ = element.set_attribute(a, &v);
        }
    } else {
        let _ = element.set_attribute(a, &v);
    }
}

fn set_event_all(element: &web_sys::Element, events: &dom::Events) {
    for (t, hid) in &events.handlers {
        let hid = *hid;
        let h = Closure::once(move |e| {
            event::dispatch(hid, e);
        });
        let event_target: &web_sys::EventTarget = element.as_ref();
        let _ = event_target.add_event_listener_with_callback_and_add_event_listener_options(
            &t,
            h.as_ref().unchecked_ref(),
            web_sys::AddEventListenerOptions::new().once(true),
        );
        h.forget();
    }
}

fn set_attribute_diff(
    element: &web_sys::Element,
    after: &dom::Attributes,
    before: &dom::Attributes,
) {
    for (a, _) in &before.attributes {
        if after.attributes.get(a).is_none() {
            let _ = element.remove_attribute(a);
        }
    }
    set_attribute_all(element, after);
}

fn render(
    after: &dom::Node,
    before: Option<&dom::Node>,
    root: Option<&web_sys::Node>,
) -> Option<web_sys::Node> {
    use dom::Node;
    match after {
        Node::Text(text) => Some(native::create_text_node(&text).into()),
        Node::Element(after) => {
            if after.need_rerendering {
                if let (Some(Node::Element(before)), Some(root)) = (before, root) {
                    if let Some(root) = root.dyn_ref::<web_sys::Element>() {
                        if after.tag_name == before.tag_name {
                            render_element_diff(after, before, root);
                            None
                        } else {
                            Some(render_element_force(after))
                        }
                    } else {
                        None
                    }
                } else {
                    Some(render_element_force(after))
                }
            } else {
                if let (Some(Node::Element(before)), Some(root)) = (before, root) {
                    render_element_lazy(after, before, root);
                    None
                } else {
                    None
                }
            }
        }
    }
}

fn render_element_lazy(after: &dom::Element, before: &dom::Element, root: &web_sys::Node) {
    let mut i: usize = 0;
    for child in &after.children {
        if let Some(b) = root.child_nodes().item(i as u32) {
            if let Some(a) = render(&child, before.children.get(i), Some(&b)) {
                let _ = root.replace_child(&a, &b);
            }
        }
        i += 1;
    }
}

fn render_element_force(after: &dom::Element) -> web_sys::Node {
    let el = new_element(&after.tag_name, &after.attributes, &after.events);
    for child in &after.children {
        if let Some(node) = render(&child, None, None) {
            let _ = el.append_child(&node);
        }
    }
    el.into()
}

fn render_element_diff(after: &dom::Element, before: &dom::Element, root: &web_sys::Element) {
    for (_, hid) in &before.events.handlers {
        event::remove(hid);
    }
    set_attribute_diff(&root, &after.attributes, &before.attributes);
    set_event_all(&root, &after.events);
    let mut i = ((before.children.len() as i64) - (after.children.len() as i64)) as usize;
    while i > 0 {
        if let Some(node) = root
            .child_nodes()
            .item((after.children.len() + i - 1) as u32)
        {
            let _ = root.remove_child(&node);
        }
        i -= 1;
    }
    let mut i: usize = 0;
    for child in &after.children {
        if let Some(old) = root.child_nodes().item(i as u32) {
            if let Some(new) = render(&child, before.children.get(i), Some(&old)) {
                let _ = root.replace_child(&new, &old);
            }
        } else {
            if let Some(new) = render(&child, None, None) {
                let _ = root.append_child(&new);
            }
        }
        i += 1;
    }
}

fn new_element(
    tag_name: &str,
    attributes: &dom::Attributes,
    events: &dom::Events,
) -> web_sys::Element {
    let element = native::create_element(tag_name);
    set_attribute_all(&element, attributes);
    set_event_all(&element, events);
    element
}
