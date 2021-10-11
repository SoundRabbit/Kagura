use crate::event;
use crate::native;
use crate::task;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;

pub struct Renderer {
    before: super::Node,
    root: web_sys::Node,
}

impl Renderer {
    pub fn new(mut virtual_node: super::Node, root_node: web_sys::Node) -> Self {
        let root: web_sys::Node;
        if let Some(node) = render(&mut virtual_node, None, Some(&root_node)) {
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

    pub fn update(&mut self, mut after: super::Node) {
        if let Some(root) = render(&mut after, Some(&mut self.before), Some(&self.root)) {
            let _ = self
                .root
                .parent_node()
                .expect("no parent node of root")
                .replace_child(&root, &self.root);
            self.root = root;
        }
        self.before = after;
    }
}

fn set_attribute_all(element: &web_sys::Element, attributes: &super::Attributes) {
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

fn set_attribute_set(element: &web_sys::Element, a: &str, v: &Vec<super::Value>, d: &str) {
    let v = v
        .iter()
        .map(|v| v.as_rc_string())
        .collect::<Vec<Rc<String>>>();
    let v = v.iter().map(|v| v.as_str()).collect::<Vec<&str>>().join(d);
    if a == "value" {
        if let Some(element) = element.dyn_ref::<web_sys::HtmlInputElement>() {
            element.set_value(&v);
        } else if let Some(element) = element.dyn_ref::<web_sys::HtmlTextAreaElement>() {
            element.set_value(&v);
        } else {
            let _ = element.set_attribute(a, &v);
        }
    } else {
        let _ = element.set_attribute(a, &v);
    }
}

fn set_event_all(element: &web_sys::Element, after: &mut super::Events, before: &super::Events) {
    for (event_name, handlers) in &before.handlers {
        let mut idx = 0;
        for handler in handlers {
            if let super::Event::HandlerId(handler_id) = handler {
                let handler_id = *handler_id;
                event::remove(&handler_id);
                if let Some(handler) = after
                    .handlers
                    .get_mut(event_name)
                    .and_then(|handlers| handlers.get_mut(idx))
                    .and_then(|e| e.take_with_id(handler_id))
                {
                    event::add(handler_id, handler);
                } else {
                    if let Some(handlers) = after.handlers.get_mut(event_name) {
                        handlers.push(super::Event::HandlerId(handler_id));
                    } else {
                        after.handlers.insert(
                            event_name.clone(),
                            vec![super::Event::HandlerId(handler_id)],
                        );
                    }
                }
            }
            idx += 1;
        }
    }

    for (event_name, evs) in &mut after.handlers {
        for ev in evs {
            if ev.is_handler() {
                let handler_id = event::new_handler_id();
                let handler = ev.take_with_id(handler_id).unwrap();

                event::add(handler_id, handler);

                let a = Closure::wrap(Box::new(move |e| {
                    event::dispatch(handler_id, e);
                }) as Box<dyn FnMut(web_sys::Event)>);
                let _ = element
                    .add_event_listener_with_callback(event_name, a.as_ref().unchecked_ref());
                a.forget();
            }
        }
    }

    if let Some(rendered) = after.rendered.take() {
        let element = element.clone();
        task::add(move || rendered(element));
    }
}

fn set_attribute_diff(
    element: &web_sys::Element,
    after: &super::Attributes,
    before: &super::Attributes,
) {
    for (a, _) in &before.attributes {
        if !after.attributes.contains_key(a) {
            let _ = element.remove_attribute(a);
        }
    }

    let mut diff_attribute = super::Attributes::new();

    for (name, values) in &after.attributes {
        let value_is_changed = before
            .attributes
            .get(name)
            .map(|before| {
                if values.len() == before.len() {
                    for i in 0..values.len() {
                        if values[i] != before[i] {
                            return true;
                        }
                    }
                    false
                } else {
                    true
                }
            })
            .unwrap_or(true);

        let value_is_changed = if value_is_changed {
            true
        } else {
            let (a, b) = (after, before);
            let a = a.delimiters.get(name).map(|a| a.as_str()).unwrap_or("");
            let b = b.delimiters.get(name).map(|b| b.as_str()).unwrap_or("");
            a != b
        };

        if value_is_changed {
            for value in values {
                diff_attribute.add(name, value.clone());
            }
            if let Some(d) = after.delimiters.get(name) {
                diff_attribute.delimit(name, d);
            }
        }
    }

    set_attribute_all(element, &diff_attribute);
}

fn render(
    after: &mut super::Node,
    before: Option<&mut super::Node>,
    root: Option<&web_sys::Node>,
) -> Option<web_sys::Node> {
    use super::Node;
    match after {
        Node::Text(text) => {
            if let Some(Node::Text(before)) = before {
                if before == text {
                    None
                } else {
                    Some(native::create_text_node(&text).into())
                }
            } else {
                Some(native::create_text_node(&text).into())
            }
        }
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

fn render_element_lazy(
    after: &mut super::Element,
    before: &mut super::Element,
    root: &web_sys::Node,
) {
    let mut i: usize = 0;
    std::mem::swap(&mut after.events, &mut before.events);
    for child in &mut after.children {
        if let Some(b) = root.child_nodes().item(i as u32) {
            if let Some(a) = render(child, before.children.get_mut(i), Some(&b)) {
                let _ = root.replace_child(&a, &b);
            }
        }
        i += 1;
    }
}

fn render_element_force(after: &mut super::Element) -> web_sys::Node {
    let el = new_element(&after.tag_name, &after.attributes, &mut after.events);
    for child in &mut after.children {
        if let Some(node) = render(child, None, None) {
            let _ = el.append_child(&node);
        }
    }
    el.into()
}

fn render_element_diff(
    after: &mut super::Element,
    before: &mut super::Element,
    root: &web_sys::Element,
) {
    set_attribute_diff(&root, &after.attributes, &before.attributes);
    set_event_all(&root, &mut after.events, &before.events);
    let mut i = (root.child_nodes().length() as i64) - (after.children.len() as i64);
    while i > 0 {
        if let Some(node) = root
            .child_nodes()
            .item((after.children.len() as i64 + i - 1) as u32)
        {
            let _ = root.remove_child(&node);
        }
        i -= 1;
    }
    let mut i: usize = 0;
    for child in &mut after.children {
        if let Some(old) = root.child_nodes().item(i as u32) {
            if let Some(new) = render(child, before.children.get_mut(i), Some(&old)) {
                let _ = root.replace_child(&new, &old);
            }
        } else {
            if let Some(new) = render(child, None, None) {
                let _ = root.append_child(&new);
            }
        }
        i += 1;
    }
}

fn new_element(
    tag_name: &str,
    attributes: &super::Attributes,
    events: &mut super::Events,
) -> web_sys::Element {
    let element = native::create_element(tag_name);
    set_attribute_all(&element, attributes);
    set_event_all(&element, events, &super::Events::new());
    element
}
