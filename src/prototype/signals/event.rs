use crate::prototype::signals::direction::Direction;

pub trait EventType {}

#[derive(Clone, Debug, Default)]
pub struct Event<D: Direction, T: EventType> {
    _direction: D,
    _event: T,
}

#[derive(Clone, Debug, Default)]
pub struct Signal {}

impl EventType for Signal {}
