use kagura::component::{Cmd, Constructor, Render, Update};
use kagura::Component;
use nusa::html::html_element::{Attributes, Events};
use nusa::{Html, HtmlComponent};
use std::pin::Pin;

pub struct Props {}

pub enum Msg {
    SetCount(usize),
}

pub enum On {}

pub struct TestComponent {
    count: usize,
}

impl Component for TestComponent {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl Constructor for TestComponent {
    fn constructor(_props: Self::Props) -> Self {
        Self { count: 0 }
    }
}

impl Update for TestComponent {
    fn update(mut self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::SetCount(count) => {
                self.count = count;
                Cmd::None
            }
        }
    }
}

impl Render<Html> for TestComponent {
    type Children = ();
    fn render(&self, _children: Self::Children) -> Html {
        Html::Fragment(vec![Html::h1(
            Attributes::new(),
            Events::new().on_click(self, {
                let count = self.count;
                move |_| Msg::SetCount(count + 1)
            }),
            vec![Html::text("count:"), Html::text(self.count.to_string())],
        )])
    }
}

impl HtmlComponent for TestComponent {}
