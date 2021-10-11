pub enum Cmd<Sub> {
    None,
    Sub(Sub),
}

impl<Sub> Cmd<Sub> {
    pub fn none() -> Self {
        Self::None
    }
}
