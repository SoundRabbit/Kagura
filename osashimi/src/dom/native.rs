extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;

use crate::dom;

pub type Render =
    fn(Option<dom::Node>, dom::Node, &web_sys::Document, web_sys::Element) -> dom::Node;

enum RenderingResult {
    Text(web_sys::Text),
    Element(web_sys::Element),
}

pub fn render(
    before: Option<dom::Node>,
    after: dom::Node,
    document: &web_sys::Document,
    root: web_sys::Element,
) -> dom::Node {
    match render_all(&after, document) {
        RenderingResult::Text(text) => {root.append_child(text.as_ref());},
        RenderingResult::Element(element) => {root.append_child(element.as_ref());}
    }
    after
}

fn render_all(node: &dom::Node, document: &web_sys::Document) -> RenderingResult {
    match node {
        dom::Node::Text(text) => RenderingResult::Text(document.create_text_node(&text)),
        dom::Node::Element {
            tag_name,
            attributes,
            events,
            children,
        } => {
            if let Ok(root) = document.create_element(&tag_name) {
                let class: Vec<&str> = attributes.class.iter().map(|id| &id as &str).collect();
                let class = class.join(" ");
                let id: Vec<&str> = attributes.id.iter().map(|id| &id as &str).collect();
                let id = id.join(" ");
                for (attribute, value) in &attributes.attributes {
                    root.set_attribute(&attribute, &value);
                }
                for child in children {
                    match render_all(child, document) {
                        RenderingResult::Text(text) => root.append_child(text.as_ref()),
                        RenderingResult::Element(element) => root.append_child(element.as_ref()),
                    };
                }
                RenderingResult::Element(root)
            } else {
                RenderingResult::Text(document.create_text_node(""))
            }
        }
    }
}
