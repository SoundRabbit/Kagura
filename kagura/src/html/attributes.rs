use crate::dom;

pub struct Attributes {
    pub attributes: dom::Attributes,
    last_attribute: String,
}

impl Attributes {
    pub fn new() -> Self{
        Self {
            attributes: dom::Attributes::new(),
            last_attribute: "".to_string()
        }
    }

    pub fn string(mut self, name: impl Into<String>, value: impl Into <String>) -> Self {
        let name = name.into();
        self.last_attribute = name.clone();
        self.attributes.add(name, dom::Value::Str(value.into()));
        self
    }

    pub fn nut(mut self, name: impl Into<String>, value: u64) -> Self {
        let name = name.into();
        self.last_attribute = name.clone();
        self.attributes.add(name, dom::Value::Nut(value));
        self
    }

    pub fn int(mut self, name: impl Into<String>, value: i64) -> Self {
        let name = name.into();
        self.last_attribute = name.clone();
        self.attributes.add(name, dom::Value::Int(value));
        self
    }

    pub fn flag(mut self, name: impl Into<String>) -> Self {
        let name = name.into();
        self.last_attribute = name.clone();
        self.attributes.set(name);
        self
    }

    pub fn delimit_with(mut self, dlm: impl Into<String>) -> Self {
        self.attributes.delimit(&self.last_attribute, dlm);
        self
    }

    pub fn checked(self) -> Self {
        self.flag("checked")
    }

    pub fn class(self, name: impl Into<String>) -> Self {
        self.string("class", name)
    }

    pub fn hidden(self) -> Self {
        self.flag("hidden")
    }
    
    pub fn href(self, uri: impl Into<String>) -> Self {
        self.string("href", uri)
    }

    pub fn id(self, name: impl Into<String>) -> Self {
        self.string("id", name)
    }

    pub fn placeholder(self, value: impl Into<String>) -> Self {
        self.string("placeholder", value)
    }

    pub fn selected(self) -> Self{
        self.flag("selected")
    }

    pub fn style(self, name: impl Into<String>, value: impl Into <String>) -> Self {
        self.string("style", name.into() + ":" + &value.into())
    }

    pub fn title(self, name: impl Into<String>) -> Self {
        self.string("title", name)
    }

    pub fn type_(self, name: impl Into<String>) -> Self {
        self.string("type", name)
    }

    pub fn value(self, value: impl Into<String>) -> Self {
        self.string("value", value)
    }
}