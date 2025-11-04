pub trait EventType {
    type Inner;

    fn get(&self) -> Self::Inner;
    fn set(&mut self, value: Self::Inner);
}

#[derive(Clone, Debug, Default)]
pub struct Signal {
    active: bool,
}

impl EventType for Signal {
    type Inner = bool;

    fn get(&self) -> Self::Inner {
        self.active
    }

    fn set(&mut self, value: Self::Inner) {
        self.active = value;
    }
}
