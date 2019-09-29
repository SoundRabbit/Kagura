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
        if let Some(node) = render(virtual_node, None, Some(&root_node)) {
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
        if let Some(root) = render(after, Some(&before), Some(&self.root)) {
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
        let v = v.iter().map(|v| &v as &str).collect::<Vec<&str>>().join(d);
        if String::from("value") == String::from(a) {
            if let Some(el) = self.dyn_ref::<native::HTMLInputElement>() {
                el.set_value(&v);
            } else {
                self.set_attribute(a, &v);
            }
        } else {
            self.set_attribute(a, &v);
        }
    }

    fn set_event_all(&self, events: dom::Events) {
        for (t, h) in events.handlers {
            let h = Closure::wrap(h);
            let option = native::EventOption::new().once(true);
            if let Ok(option) = JsValue::from_serde(&option) {
                self.add_event_listener(&t, &h, &option);
                h.forget();
            }
        }
    }

    fn set_attribute_diff(&self, after: &dom::Attributes, before: &dom::Attributes) {
        for (a, _) in &before.attributes {
            if let Some(_) = after.attributes.get(a) {

            } else {
                self.remove_attribute(a);
            }
        }
        self.set_attribute_all(after);
    }
}

fn render(
    after: dom::Node,
    before: Option<&dom::Node>,
    root: Option<&native::Node>,
) -> Option<native::Node> {
    use dom::Node;
    match after {
        Node::Text(text) => Some(native::create_text_node(&text).into()),
        Node::Element(after) => {
            if after.need_rerendering {
                if let (Some(Node::Element(before)), Some(root)) = (before, root) {
                    if let Some(root) = root.dyn_ref::<native::Element>() {
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

fn render_element_lazy(after: dom::Element, before: &dom::Element, root: &native::Node) {
    let mut i = 0;
    for child in after.children {
        if let Some(b) = root.child_nodes().item(i) {
            if let Some(a) = render(child, before.children.get(i), Some(&b)) {
                root.replace_child(&a, &b);
            }
        }
        i += 1;
    }
}

fn render_element_force(after: dom::Element) -> native::Node {
    let el = new_element(&after.tag_name, &after.attributes, after.events);
    for child in after.children {
        if let Some(node) = render(child, None, None) {
            el.append_child(&node);
        }
    }
    el.into()
}

fn render_element_diff(after: dom::Element, before: &dom::Element, root: &native::Element) {
    root.set_attribute_diff(&after.attributes, &before.attributes);
    root.set_event_all(after.events);
    let mut i = (before.children.len() as i64) - (after.children.len() as i64);
    while i > 0 {
        if let Some(node) = root
            .child_nodes()
            .item(after.children.len() + (i as usize) - 1)
        {
            root.remove_child(&node);
        }
        i -= 1;
    }
    let mut i = 0;
    for child in after.children {
        if let Some(old) = root.child_nodes().item(i) {
            if let Some(new) = render(child, before.children.get(i), Some(&old)) {
                root.replace_child(&new, &old);
            }
        } else {
            if let Some(new) = render(child, None, None) {
                root.append_child(&new);
            }
        }
        i += 1;
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
