use crate::v_node::v_element::VAttributes;

pub struct Attributes {
    index_id: Option<String>,
    data: VAttributes,
}

impl Attributes {
    pub fn into_attributes(self) -> (Option<String>, VAttributes) {
        (self.index_id, self.data)
    }
}
