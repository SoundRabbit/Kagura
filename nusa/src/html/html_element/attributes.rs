use crate::v_node::v_element::{VAttributeValue, VAttributeValues, VAttributes};
use std::rc::Rc;

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

    pub fn index_id(mut self, key: String) -> Self {
        self.index_id = Some(key);
        self
    }

    pub fn insert_with_delimiter(
        mut self,
        name: impl Into<String>,
        value: VAttributeValue,
        delimiter: impl Into<String>,
    ) -> Self {
        let name = name.into();

        if let Some(attr_values) = self.data.get_mut(&name) {
            attr_values.values.push_back(value);
        } else {
            let attr_values = VAttributeValues {
                values: vec![value].into(),
                delimiter: delimiter.into(),
            };
            self.data.insert(name, attr_values);
        }

        self
    }

    pub fn insert(self, name: impl Into<String>, value: VAttributeValue) -> Self {
        self.insert_with_delimiter(name, value, " ")
    }

    pub fn delimit(mut self, name: impl Into<String>, delimiter: impl Into<String>) -> Self {
        let name = name.into();

        if let Some(attr_values) = self.data.get_mut(&name) {
            attr_values.delimiter = delimiter.into();
        } else {
            let attr_values = VAttributeValues {
                values: vec![].into(),
                delimiter: delimiter.into(),
            };
            self.data.insert(name, attr_values);
        }

        self
    }

    pub fn string(self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.insert(name, VAttributeValue::Str(Rc::new(value.into())))
    }

    pub fn nut(self, name: impl Into<String>, value: u64) -> Self {
        self.insert(name, VAttributeValue::Nut(value))
    }

    pub fn int(self, name: impl Into<String>, value: i64) -> Self {
        self.insert(name, VAttributeValue::Int(value))
    }

    pub fn num(self, name: impl Into<String>, value: f64) -> Self {
        self.insert(name, VAttributeValue::Num(value))
    }

    pub fn flag(self, name: impl Into<String>, value: bool) -> Self {
        if value {
            self.insert(name, VAttributeValue::None)
        } else {
            self
        }
    }

    pub fn style(self, style_name: impl Into<String>, value: impl Into<String>) -> Self {
        self.insert_with_delimiter(
            "style",
            VAttributeValue::Str(Rc::new(format!("{}:{}", style_name.into(), value.into()))),
            ";",
        )
    }
}

macro_rules! attr {
    ($name:tt : String / $dlm:tt as $f_name:ident) => {
        pub fn $f_name(self, value: impl Into<String>) -> Self {
            self.insert_with_delimiter($name, VAttributeValue::Str(Rc::new(value.into())), $dlm)
        }
    };

    ($name:tt : u64 / $dlm:tt as $f_name:ident) => {
        pub fn $f_name(self, value: u64) -> Self {
            self.insert_with_delimiter($name, VAttributeValue::Nut(value), $dlm)
        }
    };

    ($name:tt : i64 / $dlm:tt as $f_name:ident) => {
        pub fn $f_name(self, value: i64) -> Self {
            self.insert_with_delimiter($name, VAttributeValue::Int(value), $dlm)
        }
    };

    ($name:tt : f64 / $dlm:tt as $f_name:ident) => {
        pub fn $f_name(self, value: f64) -> Self {
            self.insert_with_delimiter($name, VAttributeValue::Num(value), $dlm)
        }
    };

    ($name:tt : bool / $dlm:tt as $f_name:ident) => {
        pub fn $f_name(self, value: bool) -> Self {
            if value {
                self.insert_with_delimiter($name, VAttributeValue::None, $dlm)
            } else {
                self
            }
        }
    };
}

impl Attributes {
    attr!("checked": bool / " " as checked);
    attr!("class": String / " " as class);
    attr!("draggable": String / " " as draggable);
    attr!("hidden": bool / " " as hidden);
    attr!("href": String / " " as href);
    attr!("id": String / " " as id);
    attr!("placeholder": String / "" as placeholder);
    attr!("src": String / " " as src);
    attr!("title": String / " " as title);
    attr!("type": String / " " as type_);
    attr!("value": String / " " as value);
}

impl std::default::Default for Attributes {
    fn default() -> Self {
        Self::new()
    }
}
