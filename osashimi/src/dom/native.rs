extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Element;
    type Text;
    type Node;

    #[wasm_bindgen(js_namespace = console, js_name="log")]
    pub fn console_log(message: &str);

    #[wasm_bindgen(js_namespace = document, js_name="getElementById")]
    pub fn get_element_by_id(id: &str) -> Element;

    #[wasm_bindgen(js_namespace = document, js_name="createElement")]
    fn create_element(tag_name: &str) -> Element;

    #[wasm_bindgen(method, js_name = "appendChild")]
    fn append_child_element(this: &Element, element: Element);

    #[wasm_bindgen(method, js_name = "addEventListener")]
    fn add_event_listener(this: &Element, type_ : &str, closure: &Closure<FnMut()>);

    #[wasm_bindgen(method, getter, structural, js_name = "parentNode")]
    fn parent_node(this: &Element) -> Node;

    #[wasm_bindgen(method, js_name = "remove")]
    fn remove(this: &Element);

    #[wasm_bindgen(method, js_name = "addEventListener")]
    fn set_attribute(this: &Element, name: &str, value: &str);

    #[wasm_bindgen(js_namespace = document, js_name="createTextNode")]
    fn create_text_node(text: &str) -> Text;

    #[wasm_bindgen(method, js_name = "appendChild")]
    fn append_child_text_node(this: &Element, text: Text);
}

use crate::dom;

pub type Render =
    fn(Option<dom::Node>, dom::Node, String) -> dom::Node;

enum RenderingResult {
    Text(Text),
    Element(Element),
}

pub fn render(
    after: dom::Node,
    root: &Element
) {
    match render_all(after) {
        RenderingResult::Text(text) => {
            root.append_child_text_node(text);
        }
        RenderingResult::Element(element) => {
            root.append_child_element(element);
        }
    }
}

fn render_all(node: dom::Node) -> RenderingResult {
    match node {
        dom::Node::Text(text) => RenderingResult::Text(create_text_node(&text)),
        dom::Node::Element {
            tag_name,
            attributes,
            events,
            children,
        } => {
            let root = create_element(&tag_name);
            let class: Vec<&str> = attributes.class.iter().map(|id| &id as &str).collect();
            let class = class.join(" ");
            let id: Vec<&str> = attributes.id.iter().map(|id| &id as &str).collect();
            let id = id.join(" ");
            for (attribute, value) in &attributes.attributes {
                root.set_attribute(&attribute, &value);
            }
            if let Some(on_click) = events.on_click {
                dom::native::console_log("on_click");
                root.add_event_listener("click", &on_click);
                on_click.forget();
            }
            for child in children {
                match render_all(child) {
                    RenderingResult::Text(text) => root.append_child_text_node(text),
                    RenderingResult::Element(element) => root.append_child_element(element),
                };
            }
            RenderingResult::Element(root)
        }
    }
}
