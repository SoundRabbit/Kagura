use crate::v_node::v_element::VAttributes;

pub struct Attributes {
    index_id: Option<String>,
    data: VAttributes,
}

impl Attributes {
    pub fn new() -> Self {
        Self {
            index_id: None,
            data: VAttributes::new(),
        }
    }

    pub fn into_attributes(self) -> (Option<String>, VAttributes) {
        (self.index_id, self.data)
    }
}
