extern crate wasm_bindgen;

pub mod native;

use std::collections::HashMap;
use std::collections::HashSet;
use wasm_bindgen::JsCast;

pub use native::ClipboardEvent;
pub use native::DragEvent;
pub use native::Event;
pub use native::FocusEvent;
pub use native::KeyboardEvent;
pub use native::MouseEvent;

#[derive(Clone)]
pub enum Node {
    Element {
        tag_name: String,
        attributes: Attributes,
        events: Events,
        children: Vec<Node>,
        rerender: bool,
    },
    Text(String),
}

#[derive(Clone)]
pub struct Attributes {
    accept: HashSet<String>,
    accept_charset: HashSet<String>,
    action: Option<String>,
    align: Option<String>,
    alt: Option<String>,
    async_: bool,
    autocomplete: Option<String>,
    autofocus: bool,
    autoplay: bool,
    buffered: Option<String>,
    challenge: Option<String>,
    charset: Option<String>,
    checked: bool,
    cite: Option<String>,
    class: HashSet<String>,
    cols: Option<u64>,
    colspan: Option<u64>,
    controls: Option<String>,
    for_: Option<String>,
    href: Option<String>,
    id: HashSet<String>,
    label: Option<String>,
    name: Option<String>,
    placeholder: Option<String>,
    readonly: bool,
    rel: Option<String>,
    required: bool,
    rows: Option<u64>,
    rowspan: Option<u64>,
    selected: bool,
    style: HashSet<String>,
    tabindex: Option<u64>,
    target: Option<String>,
    title: Option<String>,
    type_: Option<String>,
    value: Option<String>,
    attributes: HashMap<String, String>,
}

pub struct Events {
    on_blur: Option<Box<FnMut(Event)>>,
    on_click: Option<Box<FnMut(Event)>>,
    on_copy: Option<Box<FnMut(Event)>>,
    on_cut: Option<Box<FnMut(Event)>>,
    on_drag: Option<Box<FnMut(Event)>>,
    on_dragend: Option<Box<FnMut(Event)>>,
    on_dragenter: Option<Box<FnMut(Event)>>,
    on_dragstart: Option<Box<FnMut(Event)>>,
    on_dragleave: Option<Box<FnMut(Event)>>,
    on_dragover: Option<Box<FnMut(Event)>>,
    on_drop: Option<Box<FnMut(Event)>>,
    on_keydown: Option<Box<FnMut(Event)>>,
    on_keypress: Option<Box<FnMut(Event)>>,
    on_keyup: Option<Box<FnMut(Event)>>,
    on_error: Option<Box<FnMut(Event)>>,
    on_focus: Option<Box<FnMut(Event)>>,
    on_input: Option<Box<FnMut(Event)>>,
    on_load: Option<Box<FnMut(Event)>>,
    on_mousedown: Option<Box<FnMut(Event)>>,
    on_mouseenter: Option<Box<FnMut(Event)>>,
    on_mouseleave: Option<Box<FnMut(Event)>>,
    on_mouseover: Option<Box<FnMut(Event)>>,
    on_mouseout: Option<Box<FnMut(Event)>>,
    on_mouseup: Option<Box<FnMut(Event)>>,
    on_paste: Option<Box<FnMut(Event)>>,
    on_reset: Option<Box<FnMut(Event)>>,
    on_select: Option<Box<FnMut(Event)>>,
    on_submit: Option<Box<FnMut(Event)>>,
    on_scroll: Option<Box<FnMut(Event)>>,
    on_wheel: Option<Box<FnMut(Event)>>,
}

impl Attributes {
    pub fn new() -> Attributes {
        Attributes {
            accept: HashSet::new(),
            accept_charset: HashSet::new(),
            action: None,
            align: None,
            alt: None,
            async_: false,
            autocomplete: None,
            autofocus: false,
            autoplay: false,
            buffered: None,
            challenge: None,
            charset: None,
            checked: false,
            cite: None,
            class: HashSet::new(),
            cols: None,
            colspan: None,
            controls: None,
            for_: None,
            href: None,
            id: HashSet::new(),
            label: None,
            name: None,
            placeholder: None,
            readonly: false,
            rel: None,
            required: false,
            rows: None,
            rowspan: None,
            selected: false,
            style: HashSet::new(),
            tabindex: None,
            target: None,
            title: None,
            type_: None,
            value: None,
            attributes: HashMap::new(),
        }
    }

    pub fn with_accept(mut self, accept: impl Into<String>) -> Self {
        self.accept.insert(accept.into());
        self
    }

    pub fn with_accept_charset(mut self, accept_charset: impl Into<String>) -> Self {
        self.accept_charset.insert(accept_charset.into());
        self
    }

    pub fn with_action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }

    pub fn with_align(mut self, align: impl Into<String>) -> Self {
        self.align = Some(align.into());
        self
    }

    pub fn with_alt(mut self, alt: impl Into<String>) -> Self {
        self.alt = Some(alt.into());
        self
    }

    pub fn with_async(mut self) -> Self {
        self.async_ = true;
        self
    }

    pub fn with_autocomplete(mut self, autocomplete: impl Into<String>) -> Self {
        self.autocomplete = Some(autocomplete.into());
        self
    }

    pub fn with_autofocus(mut self) -> Self {
        self.autofocus = true;
        self
    }

    pub fn with_autoplay(mut self) -> Self {
        self.autoplay = true;
        self
    }

    pub fn with_buffered(mut self, buffered: impl Into<String>) -> Self {
        self.buffered = Some(buffered.into());
        self
    }

    pub fn with_challenge(mut self, challenge: impl Into<String>) -> Self {
        self.challenge = Some(challenge.into());
        self
    }

    pub fn with_charset(mut self, charset: impl Into<String>) -> Self {
        self.charset = Some(charset.into());
        self
    }

    pub fn with_checked(mut self) -> Self {
        self.checked = true;
        self
    }

    pub fn with_cite(mut self, cite: impl Into<String>) -> Self {
        self.cite = Some(cite.into());
        self
    }

    pub fn with_class(mut self, class_name: impl Into<String>) -> Self {
        self.class.insert(class_name.into());
        self
    }

    pub fn with_cols(mut self, cols: u64) -> Self {
        self.cols = Some(cols);
        self
    }

    pub fn with_colspan(mut self, colspan: u64) -> Self {
        self.colspan = Some(colspan);
        self
    }

    pub fn with_controls(mut self, controls: impl Into<String>) -> Self {
        self.controls = Some(controls.into());
        self
    }

    pub fn with_for(mut self, for_: impl Into<String>) -> Self {
        self.for_ = Some(for_.into());
        self
    }

    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    pub fn with_id(mut self, id_name: impl Into<String>) -> Self {
        self.id.insert(id_name.into());
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn with_readonly(mut self) -> Self {
        self.readonly = true;
        self
    }

    pub fn with_rel(mut self, rel: impl Into<String>) -> Self {
        self.rel = Some(rel.into());
        self
    }

    pub fn with_required(mut self) -> Self {
        self.required = true;
        self
    }

    pub fn with_rows(mut self, rows: u64) -> Self {
        self.rows = Some(rows);
        self
    }

    pub fn with_rowspan(mut self, rowspan: u64) -> Self {
        self.rowspan = Some(rowspan);
        self
    }

    pub fn with_selected(mut self) -> Self {
        self.selected = true;
        self
    }

    pub fn with_style(mut self, style: impl Into<String>) -> Self {
        self.style.insert(style.into());
        self
    }

    pub fn with_tabindex(mut self, tabindex: u64) -> Self {
        self.tabindex = Some(tabindex);
        self
    }

    pub fn with_target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_type(mut self, title: impl Into<String>) -> Self {
        self.type_ = Some(title.into());
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn with_attribute(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(name.into(), value.into());
        self
    }
}

impl Events {
    pub fn new() -> Events {
        Events {
            on_blur: None,
            on_click: None,
            on_copy: None,
            on_cut: None,
            on_drag: None,
            on_dragend: None,
            on_dragenter: None,
            on_dragstart: None,
            on_dragleave: None,
            on_dragover: None,
            on_drop: None,
            on_keydown: None,
            on_keypress: None,
            on_keyup: None,
            on_error: None,
            on_focus: None,
            on_input: None,
            on_load: None,
            on_mousedown: None,
            on_mouseenter: None,
            on_mouseleave: None,
            on_mouseover: None,
            on_mouseout: None,
            on_mouseup: None,
            on_paste: None,
            on_reset: None,
            on_select: None,
            on_submit: None,
            on_scroll: None,
            on_wheel: None,
        }
    }
    pub fn with_on_blur(mut self, mut handler: impl FnMut(FocusEvent) + 'static) -> Self {
        self.on_blur = Some(Box::new(move |event| {
            if let Ok(e) = event.dyn_into::<FocusEvent>() {
                handler(e)
            }
        }));
        self
    }

    pub fn with_on_click(mut self, mut handler: impl FnMut(MouseEvent) + 'static) -> Self {
        self.on_click = Some(Box::new(move |event| {
            if let Ok(e) = event.dyn_into::<MouseEvent>() {
                handler(e)
            }
        }));
        self
    }

    pub fn with_on_copy(mut self, mut handler: impl FnMut(ClipboardEvent) + 'static) -> Self {
        self.on_copy = Some(Box::new(move |event| {
            if let Ok(e) = event.dyn_into::<ClipboardEvent>() {
                handler(e)
            }
        }));
        self
    }

    pub fn with_on_cut(mut self, mut handler: impl FnMut(ClipboardEvent) + 'static) -> Self {
        self.on_cut = Some(Box::new(move |event| {
            if let Ok(e) = event.dyn_into::<ClipboardEvent>() {
                handler(e)
            }
        }));
        self
    }

    pub fn with_on_drag(mut self, mut handler: impl FnMut(DragEvent) + 'static) -> Self {
        self.on_drag = Some(Box::new(move |event| {
            if let Ok(e) = event.dyn_into::<DragEvent>() {
                handler(e)
            }
        }));
        self
    }

    pub fn with_on_dragend(mut self, mut handler: impl FnMut(DragEvent) + 'static) -> Self {
        self.on_dragend = Some(Box::new(move |event| {
            if let Ok(e) = event.dyn_into::<DragEvent>() {
                handler(e)
            }
        }));
        self
    }

    pub fn with_on_dragenter(mut self, mut handler: impl FnMut(DragEvent) + 'static) -> Self {
        self.on_dragenter = Some(Box::new(move |event| {
            if let Ok(e) = event.dyn_into::<DragEvent>() {
                handler(e)
            }
        }));
        self
    }

    pub fn with_on_dragstart(mut self, mut handler: impl FnMut(DragEvent) + 'static) -> Self {
        self.on_dragstart = Some(Box::new(move |event| {
            if let Ok(e) = event.dyn_into::<DragEvent>() {
                handler(e)
            }
        }));
        self
    }

    pub fn with_on_dragleave(mut self, mut handler: impl FnMut(DragEvent) + 'static) -> Self {
        self.on_dragleave = Some(Box::new(move |event| {
            if let Ok(e) = event.dyn_into::<DragEvent>() {
                handler(e)
            }
        }));
        self
    }

    pub fn with_on_dragover(mut self, mut handler: impl FnMut(DragEvent) + 'static) -> Self {
        self.on_dragover = Some(Box::new(move |event| {
            if let Ok(e) = event.dyn_into::<DragEvent>() {
                handler(e)
            }
        }));
        self
    }

    pub fn with_on_drop(mut self, mut handler: impl FnMut(DragEvent) + 'static) -> Self {
        self.on_drop = Some(Box::new(move |event| {
            if let Ok(e) = event.dyn_into::<DragEvent>() {
                handler(e)
            }
        }));
        self
    }

    pub fn with_on_keydown(mut self, mut handler: impl FnMut(KeyboardEvent) + 'static) -> Self {
        self.on_keydown = Some(Box::new(move |event| {
            if let Ok(e) = event.dyn_into::<KeyboardEvent>() {
                handler(e)
            }
        }));
        self
    }

    pub fn with_on_keypress(mut self, mut handler: impl FnMut(KeyboardEvent) + 'static) -> Self {
        self.on_keypress = Some(Box::new(move |event| {
            if let Ok(e) = event.dyn_into::<KeyboardEvent>() {
                handler(e)
            }
        }));
        self
    }

    pub fn with_on_keyup(mut self, mut handler: impl FnMut(KeyboardEvent) + 'static) -> Self {
        self.on_keyup = Some(Box::new(move |event| {
            if let Ok(e) = event.dyn_into::<KeyboardEvent>() {
                handler(e)
            }
        }));
        self
    }

    pub fn with_on_error(mut self, handler: impl FnMut(Event) + 'static) -> Self {
        self.on_error = Some(Box::new(handler));
        self
    }

    pub fn with_on_focus(mut self, mut handler: impl FnMut(FocusEvent) + 'static) -> Self {
        self.on_focus = Some(Box::new(move |event| {
            if let Ok(e) = event.dyn_into::<FocusEvent>() {
                handler(e)
            }
        }));
        self
    }

    pub fn with_on_input(mut self, handler: impl FnMut(Event) + 'static) -> Self {
        self.on_input = Some(Box::new(handler));
        self
    }

    pub fn with_on_load(mut self, handler: impl FnMut(Event) + 'static) -> Self {
        self.on_load = Some(Box::new(handler));
        self
    }

    pub fn with_on_mousedown(mut self, mut handler: impl FnMut(MouseEvent) + 'static) -> Self {
        self.on_mousedown = Some(Box::new(move |event| {
            if let Ok(e) = event.dyn_into::<MouseEvent>() {
                handler(e)
            }
        }));
        self
    }

    pub fn with_on_mouseenter(mut self, mut handler: impl FnMut(MouseEvent) + 'static) -> Self {
        self.on_mouseenter = Some(Box::new(move |event| {
            if let Ok(e) = event.dyn_into::<MouseEvent>() {
                handler(e)
            }
        }));
        self
    }

    pub fn with_on_mouseleave(mut self, mut handler: impl FnMut(MouseEvent) + 'static) -> Self {
        self.on_mouseleave = Some(Box::new(move |event| {
            if let Ok(e) = event.dyn_into::<MouseEvent>() {
                handler(e)
            }
        }));
        self
    }

    pub fn with_on_mouseover(mut self, mut handler: impl FnMut(MouseEvent) + 'static) -> Self {
        self.on_mouseover = Some(Box::new(move |event| {
            if let Ok(e) = event.dyn_into::<MouseEvent>() {
                handler(e)
            }
        }));
        self
    }

    pub fn with_on_mouseout(mut self, mut handler: impl FnMut(MouseEvent) + 'static) -> Self {
        self.on_mouseout = Some(Box::new(move |event| {
            if let Ok(e) = event.dyn_into::<MouseEvent>() {
                handler(e)
            }
        }));
        self
    }

    pub fn with_on_mouseup(mut self, mut handler: impl FnMut(MouseEvent) + 'static) -> Self {
        self.on_mouseup = Some(Box::new(move |event| {
            if let Ok(e) = event.dyn_into::<MouseEvent>() {
                handler(e)
            }
        }));
        self
    }

    pub fn with_on_paste(mut self, mut handler: impl FnMut(ClipboardEvent) + 'static) -> Self {
        self.on_paste = Some(Box::new(move |event| {
            if let Ok(e) = event.dyn_into::<ClipboardEvent>() {
                handler(e)
            }
        }));
        self
    }

    pub fn with_on_reset(mut self, handler: impl FnMut(Event) + 'static) -> Self {
        self.on_reset = Some(Box::new(handler));
        self
    }

    pub fn with_on_select(mut self, mut handler: impl FnMut(MouseEvent) + 'static) -> Self {
        self.on_select = Some(Box::new(move |event| {
            if let Ok(e) = event.dyn_into::<MouseEvent>() {
                handler(e)
            }
        }));
        self
    }

    pub fn with_on_submit(mut self, handler: impl FnMut(Event) + 'static) -> Self {
        self.on_submit = Some(Box::new(handler));
        self
    }

    pub fn with_on_scroll(mut self, handler: impl FnMut(Event) + 'static) -> Self {
        self.on_scroll = Some(Box::new(handler));
        self
    }

    pub fn with_on_wheel(mut self, handler: impl FnMut(Event) + 'static) -> Self {
        self.on_wheel = Some(Box::new(handler));
        self
    }
}

impl Clone for Events {
    fn clone(&self) -> Self {
        Self::new()
    }
}
