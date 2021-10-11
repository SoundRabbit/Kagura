pub struct Sub<ChildSub, Msg> {
    data: Mapper<ChildSub, Msg>,
}

pub enum Mapper<ChildSub, Msg> {
    None,
    Once(Box<dyn FnOnce(ChildSub) -> Msg>),
    Map(Box<dyn FnMut(ChildSub) -> Msg>),
}

impl<ChildSub, Msg> Sub<ChildSub, Msg> {
    pub fn none() -> Self {
        Self { data: Mapper::None }
    }

    pub fn once(f: impl FnOnce(ChildSub) -> Msg + 'static) -> Self {
        Self {
            data: Mapper::Once(Box::new(f)),
        }
    }

    pub fn map(f: impl FnMut(ChildSub) -> Msg + 'static) -> Self {
        Self {
            data: Mapper::Map(Box::new(f)),
        }
    }
}

impl<ChildSub, Msg> Mapper<ChildSub, Msg> {
    pub fn take_once(&mut self) -> Option<Box<dyn FnOnce(ChildSub) -> Msg>> {
        let mut this = Self::None;
        std::mem::swap(self, &mut this);
        match this {
            Self::Once(f) => Some(f),
            _ => None,
        }
    }

    pub fn ref_map(&mut self) -> Option<&mut Box<dyn FnMut(ChildSub) -> Msg>> {
        match self {
            Self::Map(f) => Some(f),
            _ => None,
        }
    }

    pub fn try_map(&mut self, sub: ChildSub) -> Option<Msg> {
        if let Some(f) = self.ref_map() {
            Some(f(sub))
        } else if let Some(f) = self.take_once() {
            Some(f(sub))
        } else {
            None
        }
    }
}

impl<ChildSub, Msg> From<Sub<ChildSub, Msg>> for Mapper<ChildSub, Msg> {
    fn from(sub: Sub<ChildSub, Msg>) -> Self {
        sub.data
    }
}
