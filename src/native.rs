use wasm_bindgen;
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
    pub type HTMLElement;

    #[wasm_bindgen(extends = HTMLElement)]
    pub type HTMLInputElement;

    #[wasm_bindgen(extends = HTMLElement)]
    pub type HTMLCanvasElement;

    #[wasm_bindgen(extends = Node)]
    pub type Text;

    pub type HTMLCollection;

    pub type NodeList;

    pub type CanvasContext;

    #[wasm_bindgen(extends = CanvasContext)]
    pub type CanvasRenderingContext2D;

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
    pub fn add_event_listener(
        this: &EventTarget,
        type_: &str,
        closure: &Closure<FnMut(Event)>,
        option: &JsValue,
    );

    /* Nodeのメソッド */

    #[wasm_bindgen(method, js_name = "appendChild")]
    pub fn append_child(this: &Node, a_child: &Node);

    #[wasm_bindgen(method, js_name = "replaceChild")]
    pub fn replace_child(this: &Node, new_child: &Node, old_child: &Node);

    #[wasm_bindgen(method, getter = parentNode)]
    pub fn parent_node(this: &Node) -> Node;

    #[wasm_bindgen(method, getter = childNodes)]
    pub fn child_nodes(this: &Node) -> NodeList;
    #[wasm_bindgen(method, js_name = "cloneNode")]
    pub fn clone_node(this: &Node, deep: bool) -> Node;

    #[wasm_bindgen(method, js_name = "removeChild")]
    pub fn remove_child(this: &Node, child: &Node) -> Node;

    /* Elementのメソッド */

    #[wasm_bindgen(method, js_name = "remove")]
    pub fn remove(this: &Element);

    #[wasm_bindgen(method, js_name = "setAttribute")]
    pub fn set_attribute(this: &Element, name: &str, value: &str);

    #[wasm_bindgen(method, js_name = "removeAttribute")]
    pub fn remove_attribute(this: &Element, name: &str);

    #[wasm_bindgen(method, setter = id)]
    pub fn set_id(this: &Element, id: &str);

    #[wasm_bindgen(method, getter = tagName)]
    pub fn tag_name(this: &Element) -> String;

    #[wasm_bindgen(method, getter = children)]
    pub fn children(this: &Element) -> HTMLCollection;

    /* HTMLInputElementのメソッド */

    #[wasm_bindgen(method, getter = value)]
    pub fn value(this: &HTMLInputElement) -> String;

    #[wasm_bindgen(method, setter = value)]
    pub fn set_value(this: &HTMLInputElement, val: &str);

    /* HTMLCanvasElementのメソッド */

    #[wasm_bindgen(method, getter = height)]
    pub fn height(this: &HTMLCanvasElement) -> u32;

    #[wasm_bindgen(method, setter = height)]
    pub fn set_height(this: &HTMLCanvasElement, pxl: u32);

    #[wasm_bindgen(method, getter = height)]
    pub fn width(this: &HTMLCanvasElement) -> u32;

    #[wasm_bindgen(method, setter = height)]
    pub fn set_width(this: &HTMLCanvasElement, pxl: u32);

    #[wasm_bindgen(method, js_name = "getContext")]
    pub fn get_context(this: &HTMLCanvasElement, context_type: &str) -> CanvasContext;

    #[wasm_bindgen(method, js_name = "toDataURL")]
    pub fn to_data_url(this: &HTMLCanvasElement) -> String;

    /* HTMLCollectionのメソッド */

    #[wasm_bindgen(method, js_name = "item")]
    pub fn item(this: &HTMLCollection, index: usize) -> Option<Element>;

    /* NodeListのメソッド */

    #[wasm_bindgen(method, js_name = "item")]
    pub fn item(this: &NodeList, index: usize) -> Option<Node>;

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

    #[wasm_bindgen(method, getter = locale)]
    pub fn locale(this: &KeyboardEvent) -> String;

    #[wasm_bindgen(method, getter = location)]
    pub fn location(this: &KeyboardEvent) -> u64;

    #[wasm_bindgen(method, getter = metaKey)]
    pub fn meta_key(this: &KeyboardEvent) -> bool;

    #[wasm_bindgen(method, getter = repeat)]
    pub fn repeat(this: &KeyboardEvent) -> bool;

    #[wasm_bindgen(method, getter = shiftKey)]
    pub fn shift_key(this: &KeyboardEvent) -> bool;

    /* MouseEventのメソッド */

    #[wasm_bindgen(method, getter = altKey)]
    pub fn alt_key(this: &MouseEvent) -> bool;

    #[wasm_bindgen(method, getter = buttons)]
    pub fn buttons(this: &MouseEvent) -> u32;

    #[wasm_bindgen(method, getter = clientX)]
    pub fn client_x(this: &MouseEvent) -> i32;

    #[wasm_bindgen(method, getter = clientY)]
    pub fn client_y(this: &MouseEvent) -> i32;

    #[wasm_bindgen(method, getter = ctrlKey)]
    pub fn ctrl_key(this: &MouseEvent) -> bool;

    #[wasm_bindgen(method, getter = metaKey)]
    pub fn meta_key(this: &MouseEvent) -> bool;

    #[wasm_bindgen(method, getter = movementX)]
    pub fn movement_x(this: &MouseEvent) -> i32;

    #[wasm_bindgen(method, getter = movementY)]
    pub fn movement_y(this: &MouseEvent) -> i32;

    #[wasm_bindgen(method, getter = offsetX)]
    pub fn offset_x(this: &MouseEvent) -> i32;

    #[wasm_bindgen(method, getter = offsetY)]
    pub fn offset_y(this: &MouseEvent) -> i32;

    #[wasm_bindgen(method, getter = pageX)]
    pub fn page_x(this: &MouseEvent) -> i32;

    #[wasm_bindgen(method, getter = pageY)]
    pub fn page_y(this: &MouseEvent) -> i32;

    #[wasm_bindgen(method, getter = screenX)]
    pub fn screen_x(this: &MouseEvent) -> i32;

    #[wasm_bindgen(method, getter = screenY)]
    pub fn screen_y(this: &MouseEvent) -> i32;

    #[wasm_bindgen(method, getter = shiftKey)]
    pub fn shift_key(this: &MouseEvent) -> bool;
}

#[derive(Serialize)]
pub struct EventOption {
    capture: bool,
    once: bool,
    passive: bool,
}

impl EventOption {
    pub fn new() -> Self {
        Self {
            capture: false,
            once: false,
            passive: false,
        }
    }

    pub fn capture(mut self, capture: bool) -> Self {
        self.capture = capture;
        self
    }

    pub fn once(mut self, once: bool) -> Self {
        self.once = once;
        self
    }

    pub fn passive(mut self, passive: bool) -> Self {
        self.passive = passive;
        self
    }
}
