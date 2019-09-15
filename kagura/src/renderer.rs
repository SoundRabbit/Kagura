extern crate wasm_bindgen;

use crate::dom;
use crate::native;
use std::collections::HashSet;
use std::mem;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct Renderer {
    before: dom::Node,
    root: native::Node,
}

impl Renderer {
    pub fn new(virtual_node: dom::Node, root_node: native::Node) -> Self {
        let before = virtual_node.clone();
        let mut root: native::Node;
        if let Some(node) = render(virtual_node, Some(&root_node)) {
            root = node;
        } else {
            root = native::create_text_node("").into();
        }
        root_node.parent_node().replace_child(&root, &root_node);
        Self { before, root }
    }

    pub fn update(&mut self, after: dom::Node) {
        let mut before = after.clone();
        mem::swap(&mut before, &mut self.before);
        if let Some(root) = render(after, Some(&self.root)) {
            self.root.parent_node().replace_child(&root, &self.root);
            self.root = root;
        }
    }
}

impl native::Element {
    fn set_attribute_all(&self, attributes: &dom::Attributes) {
        for (a, v) in &attributes.attributes {
            if v.is_empty() {
                self.set_attribute(&a, "");
            } else if let Some(d) = attributes.delimiters.get(a) {
                self.set_attribute_set(a, v, d);
            } else {
                self.set_attribute_set(a, v, "");
            }
        }
    }

    fn set_attribute_set(&self, a: &str, v: &HashSet<dom::Value>, d: &str) {
        let v = v.iter().map(|v| v.into()).collect::<Vec<String>>();
        self.set_attribute(
            a,
            &v.iter().map(|v| &v as &str).collect::<Vec<&str>>().join(d),
        );
    }

    fn set_event_all(&self, events: dom::Events) {
        for (t, h) in events.handlers {
            let h = Closure::wrap(h);
            self.add_event_listener(&t, &h);
            h.forget();
        }
    }

    fn remove_attribute_all(&self, attributes: &dom::Attributes) {
        for (attr, _) in &attributes.attributes {
            self.remove_attribute(&attr);
        }
    }
}

fn render(
    after: dom::Node,
    root: Option<&native::Node>,
) -> Option<native::Node> {
    use dom::Node;
    match after {
        Node::Text(text) => Some(native::create_text_node(&text).into()),
        Node::Element(after) => {
            if after.need_rerendering {
                let el = new_element(&after.tag_name, &after.attributes, after.events);
                for child in after.children {
                    if let Some(node) = render(child, None) {
                        el.append_child(&node);
                    }
                }
                Some(el.into())
            } else {
                if let Some(root) = root {
                    let mut i = 0;
                    for child in after.children {
                        if let Some(b) = root.child_nodes().item(i) {
                            if let Some(a) = render(child, Some(&b)) {
                                root.replace_child(&a, &b);
                            }
                        } else {
                            if let Some(a) = render(child, None) {
                                root.append_child(&a);
                            }
                        }
                        i += 1;
                    }
                }
                None
            }
        }
    }
}

fn new_element(
    tag_name: &str,
    attributes: &dom::Attributes,
    events: dom::Events,
) -> native::Element {
    let element = native::create_element(tag_name);
    element.set_attribute_all(attributes);
    element.set_event_all(events);
    element
}
