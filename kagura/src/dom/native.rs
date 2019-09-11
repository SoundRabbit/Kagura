extern crate wasm_bindgen;

use crate::dom;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /*型 */

    pub type EventTarget;

    #[wasm_bindgen(extends = EventTarget)]
    pub type Node;

    #[wasm_bindgen(extends = Node)]
    pub type Element;

    #[wasm_bindgen(extends = Element)]
    pub type HtmlElement;

    #[wasm_bindgen(extends = HtmlElement)]
    pub type HtmlInputElement;

    #[wasm_bindgen(extends = Node)]
    pub type Text;

    pub type HTMLCollection;

    pub type Event;

    #[wasm_bindgen(extends = Event)]
    pub type FocusEvent;

    #[wasm_bindgen(extends = Event)]
    pub type ClipboardEvent;

    #[wasm_bindgen(extends = Event)]
    pub type KeyboardEvent;

    #[wasm_bindgen(extends = Event)]
    pub type MouseEvent;

    #[wasm_bindgen(extends = Event)]
    pub type DragEvent;

    /*直下 */

    #[wasm_bindgen(js_namespace = console, js_name="log")]
    pub fn console_log(message: &str);

    #[wasm_bindgen(js_namespace = document, js_name="getElementById")]
    pub fn get_element_by_id(id: &str) -> Element;

    #[wasm_bindgen(js_namespace = document, js_name="createElement")]
    pub fn create_element(tag_name: &str) -> Element;

    #[wasm_bindgen(js_namespace = document, js_name="createTextNode")]
    pub fn create_text_node(text: &str) -> Text;

    /* EventTargetのメソッド */

    #[wasm_bindgen(method, js_name = "addEventListener")]
    pub fn add_event_listener(this: &EventTarget, type_: &str, closure: &Closure<FnMut(Event)>);

    /* Nodeのメソッド */

    #[wasm_bindgen(method, js_name = "appendChild")]
    pub fn append_child(this: &Node, a_child: &Node);

    #[wasm_bindgen(method, js_name = "replaceChild")]
    pub fn replace_child(this: &Node, new_child: &Node, old_child: &Node);

    #[wasm_bindgen(method, getter = parentNode)]
    pub fn parent_node(this: &Node) -> Node;

    #[wasm_bindgen(method, getter = children)]
    pub fn children(this: &Node) -> HTMLCollection;

    /* Elementのメソッド */

    #[wasm_bindgen(method, js_name = "remove")]
    pub fn remove(this: &Element);

    #[wasm_bindgen(method, js_name = "setAttribute")]
    pub fn set_attribute(this: &Element, name: &str, value: &str);

    #[wasm_bindgen(method, setter = id)]
    pub fn set_id(this: &Element, id: &str);

    /* HtmlInputElementのメソッド */

    #[wasm_bindgen(method, getter = value)]
    pub fn value(this: &HtmlInputElement) -> String;

    #[wasm_bindgen(method, setter = value)]
    pub fn set_value(this: &HtmlInputElement) -> String;

    /* HTMLCollectionのメソッド */

    #[wasm_bindgen(method, js_name = "item")]
    pub fn item(this: &HTMLCollection, index: usize) -> Option<Node>;

    /* Eventのメソッド */

    #[wasm_bindgen(method, getter = target)]
    pub fn target(this: &Event) -> EventTarget;

    /* KeyboardEventのメソッド */

    #[wasm_bindgen(method, getter = altKey)]
    pub fn alt_key(this: &KeyboardEvent) -> bool;

    #[wasm_bindgen(method, getter = code)]
    pub fn code(this: &KeyboardEvent) -> String;

    #[wasm_bindgen(method, getter = key)]
    pub fn key(this: &KeyboardEvent) -> String;

    #[wasm_bindgen(method, getter = shiftKey)]
    pub fn shift_key(this: &KeyboardEvent) -> bool;

    /* MouseEventのメソッド */

    #[wasm_bindgen(method, getter = altKey)]
    pub fn alt_key(this: &MouseEvent) -> bool;

    #[wasm_bindgen(method, getter = buttons)]
    pub fn buttons(this: &MouseEvent) -> u64;

    #[wasm_bindgen(method, getter = clientX)]
    pub fn client_x(this: &MouseEvent) -> i64;

    #[wasm_bindgen(method, getter = clientY)]
    pub fn client_y(this: &MouseEvent) -> i64;

    #[wasm_bindgen(method, getter = ctrlKey)]
    pub fn ctrl_key(this: &MouseEvent) -> bool;

    #[wasm_bindgen(method, getter = metaKey)]
    pub fn meta_key(this: &MouseEvent) -> bool;

    #[wasm_bindgen(method, getter = movementX)]
    pub fn movement_x(this: &MouseEvent) -> i64;

    #[wasm_bindgen(method, getter = movementY)]
    pub fn movement_y(this: &MouseEvent) -> i64;

    #[wasm_bindgen(method, getter = offsetX)]
    pub fn offset_x(this: &MouseEvent) -> i64;

    #[wasm_bindgen(method, getter = offsetY)]
    pub fn offset_y(this: &MouseEvent) -> i64;

    #[wasm_bindgen(method, getter = pageX)]
    pub fn page_x(this: &MouseEvent) -> i64;

    #[wasm_bindgen(method, getter = pageY)]
    pub fn page_y(this: &MouseEvent) -> i64;

    #[wasm_bindgen(method, getter = screenX)]
    pub fn screen_x(this: &MouseEvent) -> i64;

    #[wasm_bindgen(method, getter = screenY)]
    pub fn screen_y(this: &MouseEvent) -> i64;

    #[wasm_bindgen(method, getter = shiftKey)]
    pub fn shift_key(this: &MouseEvent) -> bool;
}

pub struct Renderer {
    before: dom::Node,
    root: Node,
}

impl Renderer {
    pub fn new(virtual_node: dom::Node, root_node: Node) -> Self {
        let before = virtual_node.clone();
        let root = Self::render_all(virtual_node);
        root_node.parent_node().replace_child(&root, &root_node);
        Self { before, root }
    }

    pub fn update(&mut self, after: dom::Node) {
        self.before = after.clone();
        if let Some(root) = Self::render_component(after, &self.root, &self.root.parent_node()) {
            self.root = root;
        }
    }

    fn render_all(virtual_node: dom::Node) -> Node {
        match virtual_node {
            dom::Node::Text(text) => create_text_node(&text).into(),
            dom::Node::Element {
                tag_name,
                attributes,
                events,
                children,
                rerender: _,
            } => {
                let root = create_element(&tag_name);
                let root = Self::adapt_attribute_accept(root, &attributes);
                let root = Self::adapt_attribute_class(root, &attributes);
                let root = Self::adapt_attribute_id(root, &attributes);
                let root = Self::adapt_attribute_style(root, &attributes);
                let root = Self::adapt_attribute_value(root, &attributes);
                
                for (attribute, value) in &attributes.attributes {
                    root.set_attribute(&attribute, &value);
                }
                if let Some(on_click) = events.on_click {
                    let a = Closure::wrap(on_click);
                    root.add_event_listener("click", &a);
                    a.forget();
                }
                if let Some(on_input) = events.on_input {
                    let a = Closure::wrap(on_input);
                    root.add_event_listener("input", &a);
                    a.forget();
                }
                for child in children {
                    let child = Self::render_all(child);
                    root.append_child(&child);
                }
                root.into()
            }
        }
    }

    fn render_component(virtual_node: dom::Node, root: &Node, parent: &Node) -> Option<Node> {
        match virtual_node {
            dom::Node::Text(_) => (None),
            dom::Node::Element {
                tag_name,
                attributes,
                events,
                children,
                rerender,
            } => {
                if rerender {
                    let new_root = Self::render_all(dom::Node::Element {
                        tag_name,
                        attributes,
                        events,
                        children,
                        rerender,
                    });
                    parent.replace_child(&new_root, root);
                    Some(new_root)
                } else {
                    let mut i: usize = 0;
                    for child in children {
                        if let Some(node) = root.children().item(i) {
                            Self::render_component(child, &node, &root);
                        }
                        i += 1;
                    }
                    None
                }
            }
        }
    }

    fn adapt_attribute_accept(root: Element, attributes: &dom::Attributes) -> Element {
        if !attributes.accept.is_empty() {
            root.set_attribute(
                "accept",
                &attributes
                    .accept
                    .iter()
                    .map(|accept| &accept as &str)
                    .collect::<Vec<&str>>()
                    .join(","),
            );
        }
        root
    }

    fn adapt_attribute_class(root: Element, attributes: &dom::Attributes) -> Element {
        if !attributes.class.is_empty() {
            root.set_attribute(
                "class",
                &attributes
                    .class
                    .iter()
                    .map(|class| &class as &str)
                    .collect::<Vec<&str>>()
                    .join(" "),
            );
        }
        root
    }

    fn adapt_attribute_id(root: Element, attributes: &dom::Attributes) -> Element {
        if !attributes.id.is_empty() {
            root.set_attribute(
                "id",
                &attributes
                    .id
                    .iter()
                    .map(|id| &id as &str)
                    .collect::<Vec<&str>>()
                    .join(" "),
            );
        }
        root
    }

    fn adapt_attribute_style(root: Element, attributes: &dom::Attributes) -> Element {
        if !attributes.style.is_empty() {
            root.set_attribute(
                "style",
                &attributes
                    .style
                    .iter()
                    .map(|style| &style as &str)
                    .collect::<Vec<&str>>()
                    .join(";"),
            );
        }
        root
    }

    fn adapt_attribute_value(root: Element, attributes: &dom::Attributes) -> Element {
        if let Some(value) = &attributes.value {
            root.set_attribute("value", value);
        }
        root
    }
}
