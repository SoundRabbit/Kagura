use crate::html::component::{Render, Update};
use crate::html::Component;
use crate::Html;

pub struct Props {}

pub enum Msg {}

pub enum On {}

pub struct Document {}

impl Document {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for Document {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}

impl Update for Document {}
impl Render for Document {
    fn render(&self, _: &Props, children: Vec<Html<Self>>) -> Html<Self> {
        Html::Fragment(children)
    }
}
