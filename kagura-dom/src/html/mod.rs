use crate::HtmlPrefab;

pub enum Html {
    Component(Box<dyn HtmlPrefab>),
}
