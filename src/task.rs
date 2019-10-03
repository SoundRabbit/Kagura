use crate::bin;
use std::any::Any;

pub fn dispatch(component_id: u128, msg: Box<Any>) {
    bin::update(component_id, msg);
}
