use kagura::component::{Constructor, Render, Update};
use kagura::Component;
use nusa::{Html, HtmlComponent};

pub struct Props {}

pub enum Msg {}

pub enum On {}

pub struct Middle {}

impl Component for Middle {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl Constructor for Middle {
    fn constructor(_: Self::Props) -> Self {
        Self {}
    }
}

impl Update for Middle {}

impl Render<Html> for Middle {
    type Children = Vec<Html>;
    fn render(&self, children: Self::Children) -> Html {
        Html::fragment(children)
    }
}

impl HtmlComponent for Middle {}
