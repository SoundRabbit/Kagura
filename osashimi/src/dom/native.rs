extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use crate::dom;

#[wasm_bindgen]
extern "C" {
    /*型 */

    pub type EventTarget_;

    #[wasm_bindgen(extends = EventTarget_)]
    pub type Node_;

    #[wasm_bindgen(extends = Node_)]
    pub type Element_;

    #[wasm_bindgen(extends = Node_)]
    pub type Text_;

    pub type HTMLCollection_;

    /*直下 */

    #[wasm_bindgen(js_namespace = console, js_name="log")]
    pub fn console_log(message: &str);

    #[wasm_bindgen(js_namespace = document, js_name="getElementById")]
    pub fn get_element_by_id(id: &str) -> Element_;

    #[wasm_bindgen(js_namespace = document, js_name="createElement")]
    pub fn create_element(tag_name: &str) -> Element_;

    #[wasm_bindgen(js_namespace = document, js_name="createTextNode")]
    pub fn create_text_node(text: &str) -> Text_;

    /* EventTargetのメソッド */

    #[wasm_bindgen(method, js_name = "addEventListener")]
    pub fn add_event_listener(this: &EventTarget_, type_: &str, closure: &Closure<FnMut()>);

    /* Nodeのメソッド */

    #[wasm_bindgen(method, js_name = "appendChild")]
    pub fn append_child(this: &Node_, a_child: &Node_);

    #[wasm_bindgen(method, js_name = "replaceChild")]
    pub fn replace_child(this: &Node_, new_child: &Node_, old_child: &Node_);

    #[wasm_bindgen(method, getter = parentNode)]
    pub fn parent_node(this: &Node_) -> Node_;

    #[wasm_bindgen(method, getter = children)]
    pub fn children(this: &Node_) -> HTMLCollection_;

    /* Elementのメソッド */

    #[wasm_bindgen(method, js_name = "remove")]
    pub fn remove(this: &Element_);

    #[wasm_bindgen(method, js_name = "setAttribute")]
    pub fn set_attribute(this: &Element_, name: &str, value: &str);

    #[wasm_bindgen(method, setter = parentNode)]
    pub fn set_id(this: &Element_, id: &str);

    /* HTMLCollectionのメソッド */
    #[wasm_bindgen(method, js_name = "item")]
    pub fn item(this: &HTMLCollection_, index: usize) -> Option<Node_>;
}

pub enum Node {
    Text(Text_),
    Element(Element_)
}

pub struct Renderer {
    before: dom::Node,
    root: Node
}

impl Renderer {
    pub fn new(node: dom::Node, root: &Node_) -> Self {
        let before = node.clone();
        let parent = root.parent_node();
        let new_root = Self::render_all(node);
        match &new_root {
            Node::Text(text) => {parent.replace_child(text, root);},
            Node::Element(element) => {parent.replace_child(element, root);}
        }
        Self {
            before,
            root: new_root
        }
    }

    pub fn update(&mut self, after: dom::Node) {
        let before = after.clone();
        let root: &Node_ = match &self.root {
            Node::Text(text) => text,
            Node::Element(element) => element
        };
        let parent = root.parent_node();
        let new_root = Self::render_all(after);
        match &new_root {
            Node::Text(text) => {parent.replace_child(text, root);},
            Node::Element(element) => {parent.replace_child(element, root);}
        }
        self.root = new_root;
        self.before = before;
    }

    fn render_all(node :dom::Node) -> Node {
        match node {
            dom::Node::Text(text) => Node::Text(create_text_node(&text)),
            dom::Node::Element {
                tag_name,
                attributes,
                events,
                children,
                rerender,
            } => {
                let root = create_element(&tag_name);
                let class = attributes.class.iter().map(|id| &id as &str).collect::<Vec<&str>>().join(" ");
                let id = attributes.id.iter().map(|id| &id as &str).collect::<Vec<&str>>().join(" ");
                root.set_attribute("id", &id);
                root.set_attribute("class", &class);
                for (attribute, value) in &attributes.attributes {
                    root.set_attribute(&attribute, &value);
                }
                if let Some(on_click) = events.on_click {
                    let a = Closure::wrap(on_click);
                    root.add_event_listener("click", &a);
                    a.forget();
                }
                for child in children {
                    match Self::render_all(child) {
                        Node::Text(text) => {root.append_child(&text);},
                        Node::Element(element) => {root.append_child(&element);}
                    }
                }
                Node::Element(root)
            }
        }
    }
}