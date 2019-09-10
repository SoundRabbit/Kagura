pub struct Events<Msg> {
    pub on_click: Option<Box<FnMut() -> Msg>>,
}

impl<Msg> Events<Msg> {
    pub fn new() -> Self{
        Self {
            on_click: None,
        }
    }

    pub fn with_on_click(mut self, handler: impl FnMut() -> Msg + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}