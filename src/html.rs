use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub mod attributes;
pub mod component;
pub mod component_node;
pub mod events;

pub use component::Component;

use component::AssembledChildComponent;

use crate::kagura::node;

pub enum Html<DemirootComp: Component> {
    ComponentNode(ComponentNode<DemirootComp>),
    TextNode {
        text: String,
        events: Events<DemirootComp::Msg>,
    },
    ElementNode {
        tag_name: String,
        children: Vec<Self>,
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        ref_marker: Vec<RefMarker<DemirootComp>>,
    },
    Fragment(Vec<Self>),
}

pub enum RefMarker<DemirootComp: Component> {
    Ref(Ref<DemirootComp>),
    WrappedRef(Box<dyn FnOnce(web_sys::Node)>),
}

pub struct Ref<DemirootComp: Component> {
    name: String,
    __phantom_demiroot: std::marker::PhantomData<DemirootComp>,
}

/// Attributes for Html<Msg>
#[derive(Clone)]
pub struct Attributes {
    attributes: node::Attributes,
}

/// Events for Html<Msg>
pub struct Events<Msg> {
    handler_table: HashMap<String, Vec<EventHandler<Msg>>>,
}

pub enum EventHandler<Msg> {
    Unrwapped(Box<dyn FnOnce(web_sys::Event) -> Msg>),
    Wrapped(Box<dyn FnOnce(web_sys::Event)>),
}

pub enum ComponentNode<DemirootComp: Component> {
    PackedComponentNode(Box<dyn PackedComponentNode<DemirootComp = DemirootComp>>),
    WrappedPackedComponentNode(Box<dyn Any>),
    AssembledComponentNode(AssembledComponentNode<DemirootComp>),
    WrappedAssembledComponentNode(Box<dyn Any>),
}

pub trait PackedComponentNode {
    type DemirootComp: Component;

    fn wrap(&mut self) -> Box<dyn Any>;

    fn assemble(
        &mut self,
        before: Option<Rc<RefCell<dyn AssembledChildComponent<DemirootComp = Self::DemirootComp>>>>,
    ) -> AssembledComponentNode<Self::DemirootComp>;
}

pub struct PackedComponentNodeInstance<ThisComp: Component, DemirootComp: Component> {
    data: Option<PackedComponentNodeInstanceData<ThisComp, DemirootComp>>,
}

struct PackedComponentNodeInstanceData<ThisComp: Component, DemirootComp: Component> {
    constructor: fn(&ThisComp::Props) -> ThisComp,
    props: ThisComp::Props,
    sub_mapper: component::Sub<ThisComp::Sub, DemirootComp::Msg>,
    children: Vec<Html<DemirootComp>>,
}

pub struct WrappedPackedComponentNode<SuperDemirootComp: Component> {
    data: Box<dyn PackedComponentNode<DemirootComp = SuperDemirootComp>>,
}

pub struct AssembledComponentNode<DemirootComp: Component> {
    data: Rc<RefCell<dyn AssembledChildComponent<DemirootComp = DemirootComp>>>,
    children: Vec<Html<DemirootComp>>,
}

pub struct WrappedAssembledComponentNode<SuperDemirootComp: Component> {
    data: Option<AssembledComponentNode<SuperDemirootComp>>,
}

impl<DemirootComp: Component> Ref<DemirootComp> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            __phantom_demiroot: std::marker::PhantomData,
        }
    }
}

impl<DemirootComp: Component> std::fmt::Debug for Html<DemirootComp> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ComponentNode(..) => {
                write!(f, "[ComponentNode]")
            }
            Self::TextNode { text, .. } => write!(f, "[text]\n{}", text),
            Self::ElementNode {
                tag_name, children, ..
            } => write!(f, "[element: {}]\n{:?}", tag_name, children),
            Self::Fragment(children) => write!(f, "[Fragment]\n{:?}", children),
        }
    }
}

impl<DemirootComp: Component> Html<DemirootComp> {
    pub fn fragment(children: Vec<Html<DemirootComp>>) -> Self {
        Self::Fragment(children)
    }

    /// Creates Html from a non-validated text
    pub fn text(text: impl Into<String>) -> Self {
        Html::TextNode {
            text: text.into(),
            events: Events::new(),
        }
    }

    pub fn text_with_events(text: impl Into<String>, events: Events<DemirootComp::Msg>) -> Self {
        Html::TextNode {
            text: text.into(),
            events: events,
        }
    }

    /// Creates Html from element
    pub fn node(
        tag_name: impl Into<String>,
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Html::ElementNode {
            tag_name: tag_name.into(),
            children,
            attributes,
            events,
            ref_marker: vec![],
        }
    }

    /// Creates Html which means there is no node
    pub fn none() -> Self {
        Html::Fragment(vec![])
    }

    pub fn ref_name(mut self, name: impl Into<String>) -> Self {
        if let Self::ElementNode { ref_marker, .. } = &mut self {
            ref_marker.push(RefMarker::Ref(Ref::new(name.into())));
        }
        self
    }

    pub fn a(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("a", attributes, events, children)
    }

    pub fn abbr(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("abbr", attributes, events, children)
    }

    pub fn address(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("address", attributes, events, children)
    }

    pub fn area(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("area", attributes, events, children)
    }

    pub fn article(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("article", attributes, events, children)
    }

    pub fn aside(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("aside", attributes, events, children)
    }

    pub fn audio(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("audio", attributes, events, children)
    }

    pub fn b(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("b", attributes, events, children)
    }

    pub fn bdi(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("bdi", attributes, events, children)
    }

    pub fn bdo(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("bdo", attributes, events, children)
    }

    pub fn blockquote(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("blockquote", attributes, events, children)
    }

    pub fn button(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("button", attributes, events, children)
    }

    pub fn br(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("br", attributes, events, children)
    }

    pub fn cite(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("cite", attributes, events, children)
    }

    pub fn caption(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("caption", attributes, events, children)
    }

    pub fn canvas(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("canvas", attributes, events, children)
    }

    pub fn code(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("code", attributes, events, children)
    }

    pub fn col(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("col", attributes, events, children)
    }

    pub fn colgroup(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("colgroup", attributes, events, children)
    }

    pub fn datalist(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("datalist", attributes, events, children)
    }

    pub fn details(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("details", attributes, events, children)
    }

    pub fn dd(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("dd", attributes, events, children)
    }

    pub fn dfn(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("dfn", attributes, events, children)
    }

    pub fn div(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("div", attributes, events, children)
    }

    pub fn data(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("data", attributes, events, children)
    }

    pub fn del(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("del", attributes, events, children)
    }

    pub fn dl(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("dl", attributes, events, children)
    }

    pub fn dt(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("dt", attributes, events, children)
    }

    pub fn em(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("em", attributes, events, children)
    }

    pub fn embed(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("embed", attributes, events, children)
    }

    pub fn fieldset(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("fieldset", attributes, events, children)
    }

    pub fn figcaption(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("figcaption", attributes, events, children)
    }

    pub fn figure(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("figure", attributes, events, children)
    }

    pub fn footer(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("footer", attributes, events, children)
    }

    pub fn form(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("form", attributes, events, children)
    }

    pub fn h1(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("h1", attributes, events, children)
    }

    pub fn h2(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("h2", attributes, events, children)
    }

    pub fn h3(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("h3", attributes, events, children)
    }

    pub fn h4(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("h4", attributes, events, children)
    }

    pub fn h5(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("h5", attributes, events, children)
    }

    pub fn h6(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("h6", attributes, events, children)
    }

    pub fn header(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("header", attributes, events, children)
    }

    pub fn hr(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("hr", attributes, events, children)
    }

    pub fn i(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("i", attributes, events, children)
    }

    pub fn iframe(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("iframe", attributes, events, children)
    }

    pub fn img(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("img", attributes, events, children)
    }

    pub fn input(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("input", attributes, events, children)
    }

    pub fn ins(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("ins", attributes, events, children)
    }

    pub fn kbd(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("kbd", attributes, events, children)
    }

    pub fn label(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("label", attributes, events, children)
    }

    pub fn legend(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("legend", attributes, events, children)
    }

    pub fn li(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("li", attributes, events, children)
    }

    pub fn main(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("main", attributes, events, children)
    }

    pub fn mark(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("mark", attributes, events, children)
    }

    pub fn map(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("map", attributes, events, children)
    }

    pub fn menu(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("menu", attributes, events, children)
    }

    pub fn menuitem(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("menuitem", attributes, events, children)
    }

    pub fn meter(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("meter", attributes, events, children)
    }

    pub fn nav(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("nav", attributes, events, children)
    }

    pub fn object(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("object", attributes, events, children)
    }

    pub fn ol(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("ol", attributes, events, children)
    }

    pub fn optgroup(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("optgroup", attributes, events, children)
    }

    pub fn option(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("option", attributes, events, children)
    }

    pub fn output(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("output", attributes, events, children)
    }

    pub fn p(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("p", attributes, events, children)
    }

    pub fn param(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("param", attributes, events, children)
    }

    pub fn picture(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("picture", attributes, events, children)
    }

    pub fn pre(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("pre", attributes, events, children)
    }

    pub fn progress(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("progress", attributes, events, children)
    }

    pub fn q(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("q", attributes, events, children)
    }

    pub fn rb(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("rb", attributes, events, children)
    }

    pub fn rp(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("rp", attributes, events, children)
    }

    pub fn rt(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("rt", attributes, events, children)
    }

    pub fn rtc(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("rtc", attributes, events, children)
    }

    pub fn rubu(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("ruby", attributes, events, children)
    }

    pub fn s(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("s", attributes, events, children)
    }

    pub fn samp(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("samp", attributes, events, children)
    }

    pub fn section(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("section", attributes, events, children)
    }

    pub fn select(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("select", attributes, events, children)
    }

    pub fn small(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("small", attributes, events, children)
    }

    pub fn source(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("source", attributes, events, children)
    }

    pub fn span(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("span", attributes, events, children)
    }

    pub fn strong(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("strong", attributes, events, children)
    }

    pub fn sub(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("sub", attributes, events, children)
    }

    pub fn summary(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("summary", attributes, events, children)
    }

    pub fn sup(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("sup", attributes, events, children)
    }

    pub fn table(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("table", attributes, events, children)
    }

    pub fn tbody(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("tbody", attributes, events, children)
    }

    pub fn td(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("td", attributes, events, children)
    }

    pub fn textarea(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("textarea", attributes, events, children)
    }

    pub fn tfoot(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("tfoot", attributes, events, children)
    }

    pub fn th(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("th", attributes, events, children)
    }

    pub fn thead(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("thead", attributes, events, children)
    }

    pub fn time(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("time", attributes, events, children)
    }

    pub fn tr(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("tr", attributes, events, children)
    }

    pub fn track(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("track", attributes, events, children)
    }

    pub fn u(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("u", attributes, events, children)
    }

    pub fn ul(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("ul", attributes, events, children)
    }

    pub fn var(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("var", attributes, events, children)
    }

    pub fn video(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("video", attributes, events, children)
    }

    pub fn wbr(
        attributes: Attributes,
        events: Events<DemirootComp::Msg>,
        children: Vec<Html<DemirootComp>>,
    ) -> Self {
        Self::node("wbr", attributes, events, children)
    }
}
