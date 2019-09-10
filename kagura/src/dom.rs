use std::collections::HashMap;
use std::collections::HashSet;

pub mod native;

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
    class: HashSet<String>,
    id: HashSet<String>,
    style: HashSet<String>,
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
    cols: Option<u64>,
    colspan: Option<u64>,
    controls: Option<String>,
    for_: Option<String>,
    href: Option<String>,
    label: Option<String>,
    name: Option<String>,
    placeholder: Option<String>,
    readonly: bool,
    rel: Option<String>,
    required: bool,
    rows: Option<u64>,
    rowspan: Option<u64>,
    selected: bool,
    tabindex: Option<u64>,
    target: Option<String>,
    title: Option<String>,
    type_: Option<String>,
    value: Option<String>,
    attributes: HashMap<String, String>,
}

pub struct Events {
    on_click: Option<Box<FnMut()>>,
}

impl Attributes {
    pub fn new() -> Attributes {
        Attributes {
            class: HashSet::new(),
            id: HashSet::new(),
            style: HashSet::new(),
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
            cols: None,
            colspan: None,
            controls: None,
            for_: None,
            href: None,
            label: None,
            name: None,
            placeholder: None,
            readonly: false,
            rel: None,
            required: false,
            rows: None,
            rowspan: None,
            selected: false,
            tabindex: None,
            target: None,
            title: None,
            type_: None,
            value: None,
            attributes: HashMap::new(),
        }
    }

    pub fn with_class(mut self, class_name: impl Into<String>) -> Self {
        self.class.insert(class_name.into());
        self
    }

    pub fn with_id(mut self, id_name: impl Into<String>) -> Self {
        self.id.insert(id_name.into());
        self
    }

    pub fn with_style(mut self, style: impl Into<String>) -> Self {
        self.style.insert(style.into());
        self
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
        Events { on_click: None }
    }
    pub fn with_on_click(mut self, handler: impl FnMut() + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl Clone for Events {
    fn clone(&self) -> Self {
        Self::new()
    }
}
