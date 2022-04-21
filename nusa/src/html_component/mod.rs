use crate::html_node::BasicHtmlNode;
use crate::html_prefab::BasicHtmlPrefab;
use crate::{Html, HtmlNode};
use kagura::component::{Constructor, Render, Update};
use kagura::node::{BasicNodeMsg, Msg, SubHandler};
use kagura::Component;
use std::pin::Pin;

pub struct Sub {}

impl Sub {
    pub fn none<Event, TargetMsg>() -> Option<Box<dyn FnMut(Event) -> TargetMsg>> {
        None
    }

    pub fn map<Event, TargetMsg>(
        map: impl FnMut(Event) -> TargetMsg + 'static,
    ) -> Option<Box<dyn FnMut(Event) -> TargetMsg>> {
        Some(Box::new(map))
    }
}

pub trait HtmlComponent: Update + Render<Html> + Constructor + 'static {
    fn node_constructor(
        index_id: Option<String>,
        sub_handler: Option<SubHandler<Self>>,
        state: Pin<Box<Self>>,
        children: Self::Children,
    ) -> Box<dyn HtmlNode> {
        Box::new(BasicHtmlNode::new(index_id, sub_handler, state, children))
    }

    fn new<Target: Component + 'static>(
        target: &Target,
        index_id: Option<String>,
        props: Self::Props,
        sub_handler: Option<Box<dyn FnMut(Self::Event) -> Target::Msg>>,
        children: Self::Children,
    ) -> Html {
        let target_id = Msg::target_id(target);
        Html::Component(Box::new(BasicHtmlPrefab::new(
            Self::node_constructor,
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
        sub_handler: Option<Box<dyn FnMut(Self::Event) -> Target::Msg>>,
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
