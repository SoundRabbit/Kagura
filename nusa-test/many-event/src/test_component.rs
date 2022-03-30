use kagura::component::{Cmd, Constructor, Render, Update};
use kagura::Component;
use nusa::html::html_element::{Attributes, Events};
use nusa::{Html, HtmlComponent};
use std::pin::Pin;

pub struct Props {}

pub enum Msg {
    CountUp,
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
            Msg::CountUp => {
                self.count += 1;
                Cmd::None
            }
        }
    }
}

impl Render<Html> for TestComponent {
    type Children = ();
    fn render(&self, _children: Self::Children) -> Html {
        let mut many_events = Events::new();
        for _ in 0..1000 {
            many_events = many_events.on_click(self, move |_| Msg::CountUp);
        }

        Html::h1(
            Attributes::new(),
            many_events,
            vec![Html::text("count:"), Html::text(self.count.to_string())],
        )
    }
}

impl HtmlComponent for TestComponent {}
