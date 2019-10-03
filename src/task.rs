use crate::bin;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;

pub fn dispatch(component_id: u128, msg: Box<Any>) {
    bin::update(component_id, msg);
}
