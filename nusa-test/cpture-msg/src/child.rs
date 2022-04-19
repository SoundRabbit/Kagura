use kagura::component::{Cmd, Constructor, Render, Update};
use kagura::Component;
use nusa::html::html_element::{Attributes, Events};
use nusa::{Html, HtmlComponent};
use std::pin::Pin;

pub struct Props {
    pub count: usize,
}

pub enum Msg {
    Click,
}

pub enum On {
    Click,
}

pub struct Child {
    count: usize,
}

impl Component for Child {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl Constructor for Child {
    fn constructor(props: Self::Props) -> Self {
        Self { count: props.count }
    }
}

impl Update for Child {
    fn on_load(mut self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        self.count = props.count;
        Cmd::None
    }

    fn update(self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::Click => Cmd::Submit(On::Click),
        }
    }
}

impl Render<Html> for Child {
    type Children = ();
    fn render(&self, _children: Self::Children) -> Html {
        Html::Fragment(vec![Html::h1(
            Attributes::new(),
            Events::new().on_click(self, move |_| Msg::Click),
            vec![Html::text("count:"), Html::text(self.count.to_string())],
        )])
    }
}

impl HtmlComponent for Child {}
