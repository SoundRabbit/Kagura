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
        if let Some(node) = render(virtual_node, None, Some(&root_node), false) {
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
        if let Some(root) = render(after, Some(&before), Some(&self.root), false) {
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
    before: Option<&dom::Node>,
    root: Option<&native::Node>,
    cloned: bool,
) -> Option<native::Node> {
    use dom::Node;
    match after {
        Node::Text(text) => Some(native::create_text_node(&text).into()),
        Node::Element(after) => {
            if after.need_rerendering {
                if let (Some(Node::Element(before)), Some(root)) = (before, root) {
                    if before.tag_name == after.tag_name {
                        let clone = if !cloned { Some(root.clone_node(true)) } else {None};
                        let root = if let Some(c) = &clone {c} else {root};
                        let cloned = true;
                        if let Some(root) = root.dyn_ref::<native::Element>() {
                            if before.attributes != after.attributes {
                                root.remove_attribute_all(&before.attributes);
                                root.set_attribute_all(&after.attributes);
                            }
                            root.set_event_all(after.events);
                            let mut i = before.children.len() - after.children.len();
                            while i > 0 {
                                if let Some(a) = root.child_nodes().item(i) {
                                    root.remove_child(&a);
                                }
                                i -= 1;
                            }
                            let mut i = 0;
                            for child in after.children {
                                if let Some(b) = root.child_nodes().item(i) {
                                    if let Some(a) = render(child, before.children.get(i), Some(&b), cloned) {
                                        root.replace_child(&a, &b);
                                    }
                                } else {
                                    if let Some(a) = render(child, before.children.get(i), None, cloned) {
                                        root.append_child(&a);
                                    }
                                }
                                i += 1;
                            }
                            return clone;
                        }
                    }
                }
                let el = new_element(&after.tag_name, &after.attributes, after.events);
                for child in after.children {
                    if let Some(node) = render(child, None, None, cloned) {
                        el.append_child(&node);
                    }
                }
                Some(el.into())
            } else {
                if let (Some(root), Some(Node::Element(before))) = (root, before){
                    let mut i = 0;
                    for child in after.children {
                        if let Some(b) = root.child_nodes().item(i) {
                            if let Some(a) = render(child, before.children.get(i),Some(&b), cloned) {
                                root.replace_child(&a, &b);
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
