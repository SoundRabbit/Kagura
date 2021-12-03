use std::ops::Deref;
use std::rc::Rc;

use super::*;

impl<DemirootComp: Component> Deref for Attributes<DemirootComp> {
    type Target = node::Attributes;
    fn deref(&self) -> &node::Attributes {
        &self.attributes
    }
}

impl<DemirootComp: Component> Into<node::Attributes> for Attributes<DemirootComp> {
    fn into(self) -> node::Attributes {
        self.attributes
    }
}

impl<DemirootComp: Component> Attributes<DemirootComp> {
    /// Creates new empty Attributs
    pub fn new() -> Self {
        Self {
            attributes: node::Attributes::new(),
            key: None,
            ref_marker: vec![],
        }
    }

    /// Adds attribute having string value
    pub fn string(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        let name = name.into();
        self.attributes
            .add(name, node::Value::Str(Rc::new(value.into())));
        self
    }

    /// Adds attribute having natural number
    pub fn nut(mut self, name: impl Into<String>, value: u64) -> Self {
        let name = name.into();
        self.attributes.add(name, node::Value::Nut(value));
        self
    }

    /// Adds attribute having integer
    pub fn int(mut self, name: impl Into<String>, value: i64) -> Self {
        let name = name.into();
        self.attributes.add(name, node::Value::Int(value));
        self
    }

    /// Adds attribute not hanving any value
    pub fn flag(mut self, name: impl Into<String>) -> Self {
        let name = name.into();
        self.attributes.set(name);
        self
    }

    /// Sets delimiter for last attribute
    pub fn delimit(mut self, attr: impl Into<String>, dlm: impl Into<String>) -> Self {
        self.attributes.delimit(attr, dlm);
        self
    }

    /// Sets key of Element
    pub fn key_name(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// Sets reference to Element
    pub fn ref_name(mut self, ref_name: impl Into<String>) -> Self {
        self.ref_marker
            .push(RefMarker::RefString(RefString::new(ref_name.into())));
        self
    }

    /// Gets attributes
    pub fn restricted<C: Component>(&self) -> Attributes<C> {
        Attributes {
            attributes: self.attributes.clone(),
            key: None,
            ref_marker: vec![],
        }
    }

    pub fn checked(self) -> Self {
        self.flag("checked")
    }

    pub fn class(self, name: impl Into<String>) -> Self {
        self.string("class", name)
    }

    pub fn draggable(self, val: bool) -> Self {
        if val {
            self.string("draggable", "true")
        } else {
            self.string("draggable", "false")
        }
    }

    pub fn height(self, val: u64) -> Self {
        self.nut("height", val)
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

    pub fn selected(self) -> Self {
        self.flag("selected")
    }

    pub fn style(self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.string("style", name.into() + ":" + &value.into())
    }

    pub fn src(self, value: impl Into<String>) -> Self {
        self.string("src", value)
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

    pub fn width(self, val: u64) -> Self {
        self.nut("width", val)
    }
}
