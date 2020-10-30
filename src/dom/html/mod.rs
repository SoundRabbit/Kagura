use std::any::Any;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub mod attributes;
pub mod events;

pub use attributes::Attributes;
pub use events::Events;

use super::component::{
    Component, ComponentBuilder, Composed, ComposedComponent, Constructor, Subscription,
};

/// viritual html element
pub enum Html {
    ComponentBuilder {
        builder: Option<
            Box<
                dyn FnOnce(
                    Option<Rc<RefCell<Box<dyn Composed + 'static>>>>,
                ) -> Rc<RefCell<Box<dyn Composed>>>,
            >,
        >,
        parent: Option<Weak<RefCell<Box<dyn Composed + 'static>>>>,
        children: Vec<Html>,
    },
    ComponentNode(Rc<RefCell<Box<dyn Composed>>>),
    TextNode(String),
    ElementNode {
        tag_name: String,
        children: Vec<Html>,
        attributes: Attributes,
        events: Events,
        parent: Option<Weak<RefCell<Box<dyn Composed + 'static>>>>,
    },
    Fragment(Vec<Html>),
    None,
}

impl std::fmt::Debug for Html {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ComponentBuilder { children, .. } => {
                write!(f, "[ComponentBuilder]\n{:?}", children)
            }
            Self::ComponentNode(..) => write!(f, "[ComponentNode]"),
            Self::TextNode(text) => write!(f, "[text]\n{}", text),
            Self::ElementNode {
                tag_name, children, ..
            } => write!(f, "[element: {}]\n{:?}", tag_name, children),
            Self::Fragment(children) => write!(f, "[Fragment]\n{:?}", children),
            Self::None => write!(f, "[None"),
        }
    }
}

impl Clone for Html {
    fn clone(&self) -> Self {
        match self {
            Self::ComponentBuilder { .. } => Self::ComponentBuilder {
                builder: None,
                parent: None,
                children: Vec::new(),
            },
            Self::ComponentNode(component_node) => Self::ComponentNode(Rc::clone(&component_node)),
            Self::TextNode(text) => Self::TextNode(text.clone()),
            Self::ElementNode {
                tag_name,
                children,
                attributes,
                events,
                parent,
            } => Self::ElementNode {
                tag_name: tag_name.clone(),
                children: children.clone(),
                attributes: attributes.clone(),
                events: events.clone(),
                parent: parent.as_ref().map(|x| Weak::clone(x)),
            },
            Self::Fragment(children) => Self::Fragment(children.clone()),
            Self::None => Self::None,
        }
    }
}

impl Html {
    pub fn component<C: 'static, P: 'static, M: 'static, S: 'static>(
        props: P,
        sub_map: Subscription<S>,
        children: Vec<Html>,
    ) -> Html
    where
        C: Component<Props = P, Msg = M, Sub = S> + Constructor<Props = P>,
    {
        Html::ComponentBuilder {
            builder: Some(Box::new(move |before| {
                if let Some(before) = before {
                    if let Some(component) =
                        Any::downcast_mut::<ComposedComponent<P, M, S>>(&mut (*before.borrow_mut()))
                    {
                        component.init(props);
                        return Rc::clone(&before);
                    }
                }
                let mut builder = ComponentBuilder::new();
                let component = C::constructor(props, &mut builder);
                ComposedComponent::new(component, builder, sub_map)
            })),
            parent: None,
            children: children,
        }
    }

    pub fn fragment(children: Vec<Html>) -> Self {
        Self::Fragment(children)
    }

    /// Creates Html from a non-validated text
    pub fn text(text: impl Into<String>) -> Self {
        Html::TextNode(text.into())
    }

    /// Creates Html from element
    pub fn node(
        tag_name: impl Into<String>,
        attributes: Attributes,
        events: Events,
        children: Vec<Html>,
    ) -> Self {
        Html::ElementNode {
            tag_name: tag_name.into(),
            children,
            attributes,
            events,
            parent: None,
        }
    }

    /// Creates Html which means there is no node
    pub fn none() -> Self {
        Html::None
    }

    pub fn a(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("a", attributes, events, children)
    }

    pub fn abbr(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("abbr", attributes, events, children)
    }

    pub fn address(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("address", attributes, events, children)
    }

    pub fn area(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("area", attributes, events, children)
    }

    pub fn article(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("article", attributes, events, children)
    }

    pub fn aside(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("aside", attributes, events, children)
    }

    pub fn audio(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("audio", attributes, events, children)
    }

    pub fn b(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("b", attributes, events, children)
    }

    pub fn bdi(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("bdi", attributes, events, children)
    }

    pub fn bdo(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("bdo", attributes, events, children)
    }

    pub fn blockquote(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("blockquote", attributes, events, children)
    }

    pub fn button(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("button", attributes, events, children)
    }

    pub fn br(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("br", attributes, events, children)
    }

    pub fn cite(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("cite", attributes, events, children)
    }

    pub fn caption(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("caption", attributes, events, children)
    }

    pub fn canvas(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("canvas", attributes, events, children)
    }

    pub fn code(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("code", attributes, events, children)
    }

    pub fn col(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("col", attributes, events, children)
    }

    pub fn colgroup(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("colgroup", attributes, events, children)
    }

    pub fn datalist(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("datalist", attributes, events, children)
    }

    pub fn details(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("details", attributes, events, children)
    }

    pub fn dd(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("dd", attributes, events, children)
    }

    pub fn dfn(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("dfn", attributes, events, children)
    }

    pub fn div(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("div", attributes, events, children)
    }

    pub fn data(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("data", attributes, events, children)
    }

    pub fn del(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("del", attributes, events, children)
    }

    pub fn dl(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("dl", attributes, events, children)
    }

    pub fn dt(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("dt", attributes, events, children)
    }

    pub fn em(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("em", attributes, events, children)
    }

    pub fn embed(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("embed", attributes, events, children)
    }

    pub fn fieldset(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("fieldset", attributes, events, children)
    }

    pub fn figcaption(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("figcaption", attributes, events, children)
    }

    pub fn figure(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("figure", attributes, events, children)
    }

    pub fn footer(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("footer", attributes, events, children)
    }

    pub fn form(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("form", attributes, events, children)
    }

    pub fn h1(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("h1", attributes, events, children)
    }

    pub fn h2(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("h2", attributes, events, children)
    }

    pub fn h3(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("h3", attributes, events, children)
    }

    pub fn h4(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("h4", attributes, events, children)
    }

    pub fn h5(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("h5", attributes, events, children)
    }

    pub fn h6(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("h6", attributes, events, children)
    }

    pub fn header(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("header", attributes, events, children)
    }

    pub fn hr(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("hr", attributes, events, children)
    }

    pub fn i(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("i", attributes, events, children)
    }

    pub fn iframe(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("iframe", attributes, events, children)
    }

    pub fn img(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("img", attributes, events, children)
    }

    pub fn input(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("input", attributes, events, children)
    }

    pub fn ins(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("ins", attributes, events, children)
    }

    pub fn kbd(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("kbd", attributes, events, children)
    }

    pub fn label(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("label", attributes, events, children)
    }

    pub fn legend(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("legend", attributes, events, children)
    }

    pub fn li(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("li", attributes, events, children)
    }

    pub fn main(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("main", attributes, events, children)
    }

    pub fn mark(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("mark", attributes, events, children)
    }

    pub fn map(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("map", attributes, events, children)
    }

    pub fn menu(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("menu", attributes, events, children)
    }

    pub fn menuitem(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("menuitem", attributes, events, children)
    }

    pub fn meter(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("meter", attributes, events, children)
    }

    pub fn nav(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("nav", attributes, events, children)
    }

    pub fn object(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("object", attributes, events, children)
    }

    pub fn ol(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("ol", attributes, events, children)
    }

    pub fn optgroup(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("optgroup", attributes, events, children)
    }

    pub fn option(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("option", attributes, events, children)
    }

    pub fn output(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("output", attributes, events, children)
    }

    pub fn p(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("p", attributes, events, children)
    }

    pub fn param(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("param", attributes, events, children)
    }

    pub fn picture(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("picture", attributes, events, children)
    }

    pub fn pre(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("pre", attributes, events, children)
    }

    pub fn progress(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("progress", attributes, events, children)
    }

    pub fn q(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("q", attributes, events, children)
    }

    pub fn rb(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("rb", attributes, events, children)
    }

    pub fn rp(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("rp", attributes, events, children)
    }

    pub fn rt(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("rt", attributes, events, children)
    }

    pub fn rtc(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("rtc", attributes, events, children)
    }

    pub fn rubu(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("ruby", attributes, events, children)
    }

    pub fn s(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("s", attributes, events, children)
    }

    pub fn samp(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("samp", attributes, events, children)
    }

    pub fn section(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("section", attributes, events, children)
    }

    pub fn select(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("select", attributes, events, children)
    }

    pub fn small(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("small", attributes, events, children)
    }

    pub fn source(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("source", attributes, events, children)
    }

    pub fn span(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("span", attributes, events, children)
    }

    pub fn strong(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("strong", attributes, events, children)
    }

    pub fn sub(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("sub", attributes, events, children)
    }

    pub fn summary(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("summary", attributes, events, children)
    }

    pub fn sup(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("sup", attributes, events, children)
    }

    pub fn table(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("table", attributes, events, children)
    }

    pub fn tbody(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("tbody", attributes, events, children)
    }

    pub fn td(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("td", attributes, events, children)
    }

    pub fn textarea(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("textarea", attributes, events, children)
    }

    pub fn tfoot(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("tfoot", attributes, events, children)
    }

    pub fn th(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("th", attributes, events, children)
    }

    pub fn thead(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("thead", attributes, events, children)
    }

    pub fn time(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("time", attributes, events, children)
    }

    pub fn tr(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("tr", attributes, events, children)
    }

    pub fn track(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("track", attributes, events, children)
    }

    pub fn u(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("u", attributes, events, children)
    }

    pub fn ul(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("ul", attributes, events, children)
    }

    pub fn var(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("var", attributes, events, children)
    }

    pub fn video(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("video", attributes, events, children)
    }

    pub fn wbr(attributes: Attributes, events: Events, children: Vec<Html>) -> Self {
        Html::node("wbr", attributes, events, children)
    }
}
