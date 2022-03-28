use crate::html_prefab::BasicHtmlPrefab;
use crate::Html;
use kagura::component::{Constructor, Render, Update};
use kagura::node::{BasicNodeMsg, Msg, SubHandler};
use kagura::Component;

pub trait HtmlComponent: Update + Render<Html> + Constructor + 'static {
    fn new<Target: Component + 'static>(
        target: &Target,
        index_id: Option<String>,
        props: Self::Props,
        sub_handler: Option<impl FnMut(Self::Sub) -> Target::Msg + 'static>,
        children: Self::Children,
    ) -> Html {
        let target_id = Msg::target_id(target);
        Html::Component(Box::new(BasicHtmlPrefab::new(
            Self::constructor,
            index_id,
            props,
            sub_handler.map(|mut x| {
                Box::new(move |e| {
                    let msg = x(e);
                    let msg = BasicNodeMsg::ComponentMsg::<Target>(msg);
                    Msg::new(target_id, Box::new(msg))
                }) as SubHandler<Self>
            }),
            children,
        )))
    }

    fn empty<Target: Component + 'static>(
        target: &Target,
        index_id: Option<String>,
        props: Self::Props,
        sub_handler: Option<impl FnMut(Self::Sub) -> Target::Msg + 'static>,
    ) -> Html {
        Self::new(
            target,
            index_id,
            props,
            sub_handler,
            Self::Children::default(),
        )
    }
}
