extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    /*型 */

    pub type EventTarget;

    #[wasm_bindgen(extends = EventTarget)]
    pub type Node;

    #[wasm_bindgen(extends = Node)]
    pub type Element;

    #[wasm_bindgen(extends = Node)]
    pub type Text;

    /*直下 */

    #[wasm_bindgen(js_namespace = console, js_name="log")]
    pub fn console_log(message: &str);

    #[wasm_bindgen(js_namespace = document, js_name="getElementById")]
    pub fn get_element_by_id(id: &str) -> Element;

    #[wasm_bindgen(js_namespace = document, js_name="createElement")]
    fn create_element(tag_name: &str) -> Element;

    #[wasm_bindgen(js_namespace = document, js_name="createTextNode")]
    fn create_text_node(text: &str) -> Text;

    /* EventTargetのメソッド */

    #[wasm_bindgen(method, js_name = "addEventListener")]
    fn add_event_listener(this: &EventTarget, type_: &str, closure: &Closure<FnMut()>);

    /* Nodeのメソッド */

    #[wasm_bindgen(method, js_name = "appendChild")]
    fn append_child(this: &Node, a_child: &Node);

    #[wasm_bindgen(method, getter = parentNode)]
    fn parent_node(this: &Node) -> Node;

    /* Elementのメソッド */

    #[wasm_bindgen(method, js_name = "remove")]
    fn remove(this: &Element);

    #[wasm_bindgen(method, js_name = "setAttribute")]
    fn set_attribute(this: &Element, name: &str, value: &str);

    #[wasm_bindgen(method, setter = parentNode)]
    fn set_id(this: &Element, id :&str);
}

use crate::dom;

enum NodeKind {
    Text(Text),
    Element(Element)
}

pub fn render(after: dom::Node, root: &Element) -> Option<Element>{
    match render_all(after) {
        NodeKind::Text(text) => None,
        NodeKind::Element(element) => {
            root.append_child(&element);
            Some(element)
        }
    }
}

pub fn rerender(after: dom::Node, root: &Element) -> Option<Element>{
    let parent = root.parent_node();
    match render_all(after) {
        NodeKind::Text(text) => None,
        NodeKind::Element(element) => {
            parent.append_child(&element);
            root.remove();
            Some(element)
        }
    }
}

fn render_all(node: dom::Node) -> NodeKind {
    match node {
        dom::Node::Text(text) => NodeKind::Text(create_text_node(&text)),
        dom::Node::Element {
            tag_name,
            attributes,
            events,
            children,
            rerender,
        } => {
            let root = create_element(&tag_name);
            let class: Vec<&str> = attributes.class.iter().map(|id| &id as &str).collect();
            let class = class.join(" ");
            let id: Vec<&str> = attributes.id.iter().map(|id| &id as &str).collect();
            let id = id.join(" ");
            root.set_attribute("id", &id);
            root.set_attribute("class", &class);
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
                    NodeKind::Text(text) => {root.append_child(&text);}
                    NodeKind::Element(element) => {root.append_child(&element);}
                }
            }
            NodeKind::Element(root)
        }
    }
}
