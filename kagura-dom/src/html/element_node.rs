pub struct ElementNode {
    tag_name: String,
    children: Vec<Self>,
    attributes: Attributes,
    events: Events,
}
