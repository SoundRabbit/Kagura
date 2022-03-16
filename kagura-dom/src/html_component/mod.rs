use crate::html_prefab::{BasicHtmlPrefab, HtmlPrefab};
use crate::Html;
use kagura::component::{Constructor, Render, Update};

pub trait HtmlComponent: Update + Render<Html> + Constructor + 'static {
    fn new(index_id: Option<String>, props: Self::Props, children: Self::Children) -> Html {
        Html::Component(Box::new(BasicHtmlPrefab::new(
            Self::constructor,
            index_id,
            props,
            children,
        )))
    }

    fn empty(index_id: Option<String>, props: Self::Props) -> Html {
        Html::Component(Box::new(BasicHtmlPrefab::new(
            Self::constructor,
            index_id,
            props,
            Self::Children::default(),
        )))
    }
}
