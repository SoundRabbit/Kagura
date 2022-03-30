use crate::Html;
use kagura::component::{Constructor, Render, Update};
use kagura::Component;

pub struct BasicDomComponent {}

impl Component for BasicDomComponent {
    type Props = ();
    type Msg = ();
    type Event = ();
}

impl Constructor for BasicDomComponent {
    fn constructor(_props: ()) -> Self {
        Self {}
    }
}

impl BasicDomComponent {
    pub fn new() -> Self {
        Self::constructor(())
    }
}

impl Update for BasicDomComponent {}

impl Render<Html> for BasicDomComponent {
    type Children = Vec<Html>;
    fn render(&self, children: Self::Children) -> Html {
        Html::Fragment(children)
    }
}
