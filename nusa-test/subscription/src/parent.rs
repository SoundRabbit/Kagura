use super::child::{self, Child};
use kagura::component::{Cmd, Constructor, Render, Update};
use kagura::Component;
use nusa::html_component::Sub;
use nusa::{Html, HtmlComponent};
use std::pin::Pin;

pub struct Props {}

pub enum Msg {
    SetCount(usize),
}

pub enum On {}

pub struct Parent {
    count: usize,
}

impl Component for Parent {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl Constructor for Parent {
    fn constructor(_props: Self::Props) -> Self {
        Self { count: 0 }
    }
}

impl Update for Parent {
    fn update(mut self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::SetCount(count) => {
                self.count = count;
                Cmd::None
            }
        }
    }
}

impl Render<Html> for Parent {
    type Children = ();
    fn render(&self, _children: Self::Children) -> Html {
        Child::empty(
            self,
            None,
            child::Props { count: self.count },
            Sub::map({
                let count = self.count;
                move |e| match e {
                    child::On::Click => Msg::SetCount(count + 1),
                }
            }),
        )
    }
}

impl HtmlComponent for Parent {}
