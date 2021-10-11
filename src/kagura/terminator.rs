use crate::html::Component;

pub struct Props {}

pub enum Msg {}

pub enum On {}

pub struct Terminator {}

impl Component for Terminator {
    type Props = Props;
    type Msg = Msg;
    type Sub = On;
}
