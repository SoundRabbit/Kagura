extern crate wasm_bindgen;

use crate::dom;
use crate::native;
use std::mem;
use wasm_bindgen::prelude::*;

pub struct Renderer {
    before: dom::Node,
    root: native::Node,
}

impl Renderer {
    pub fn new(virtual_node: dom::Node, root_node: native::Node) -> Self {
        let before = virtual_node.clone();
        let root = Self::render_all(virtual_node);
        root_node.parent_node().replace_child(&root, &root_node);
        Self { before, root }
    }

    pub fn update(&mut self, after: dom::Node) {
        let mut before = after.clone();
        mem::swap(&mut before, &mut self.before);
        let before = before;
        if let Some(root) = Self::render_component(after, &before, &self.root, &self.root.parent_node()) {
            self.root = root;
        }
    }

    fn render_all(virtual_node: dom::Node) -> native::Node {
        match virtual_node {
            dom::Node::Text(text) => native::create_text_node(&text).into(),
            dom::Node::Element(el) => {
                let root = native::create_element(&el.tag_name);

                Self::adapt_attribute_all(&root, &el.attributes);
                Self::adapt_event_all(&root, el.events);

                for child in el.children {
                    let child = Self::render_all(child);
                    root.append_child(&child);
                }

                root.into()
            }
        }
    }

    fn render_diff(
        after: dom::Node,
        before: Option<(&dom::Node, &native::Node)>,
        parent: &native::Node,
    ) -> Option<native::Node> {
        if let Some((before, root)) = before {
            if *before == after {
                None
            } else {
                let new_root = Self::render_all(after);
                native::console_log("1");
                parent.replace_child(&new_root, root);
                Some(new_root)
            }
        } else {
            let root = Self::render_all(after);
            parent.append_child(&root);
            Some(root)
        }
    }

    fn render_component(
        after: dom::Node,
        before: &dom::Node,
        root: &native::Node,
        parent: &native::Node,
    ) -> Option<native::Node> {
        match after {
            dom::Node::Text(_) => (None),
            dom::Node::Element(el) => {
                if el.need_rerendering {
                    Self::render_diff(dom::Node::Element(el), Some((before, root)), parent) 
                } else if let dom::Node::Element(before) = before{
                    let mut i: usize = 0;
                    for child in el.children {
                        if let Some(node) = root.children().item(i) {
                            if let Some(before) = before.children.get(i) {
                                Self::render_component(child, &before, &node, &root);
                            } else {
                                let child = Self::render_all(child);
                                native::console_log("3");
                                root.replace_child(&child, &node);
                            }
                        }
                        i += 1;
                    }
                    None
                } else {
                    let new_root = Self::render_all(dom::Node::Element(el));
                    native::console_log("4");
                    parent.replace_child(&new_root, root);
                    Some(new_root)
                }
            }
        }
    }

    fn adapt_attribute_all(element: &native::Element, attributes: &dom::Attributes) {
        for (attr, val) in &attributes.attributes {
            if val.is_empty() {
                element.set_attribute(&attr, "");
            } else {
                let val = val
                    .iter()
                    .map(|v| match v {
                        dom::Value::Str(s) => s.clone(),
                        dom::Value::Int(i) => i.to_string(),
                        dom::Value::Nut(i) => i.to_string(),
                    })
                    .collect::<Vec<String>>();
                if let Some(dlm) = attributes.delimiters.get(attr) {
                    element.set_attribute(
                        &attr,
                        &val.iter()
                            .map(|v| &v as &str)
                            .collect::<Vec<&str>>()
                            .join(dlm),
                    );
                } else {
                    element.set_attribute(
                        &attr,
                        &val.iter()
                            .map(|v| &v as &str)
                            .collect::<Vec<&str>>()
                            .join(""),
                    );
                }
            }
        }
    }

    fn adapt_event_all(element: &native::Element, events: dom::Events) {
        for (tp, hnd) in events.handlers {
            let a = Closure::wrap(hnd);
            element.add_event_listener(&tp, &a);
            a.forget();
        }
    }

    fn _remove_attribute_all(element: &native::Element, attributes: &dom::Attributes) {
        for (attr, _) in &attributes.attributes {
            element.remove_attribute(&attr);
        }
    }
}
