pub enum Direction {
    In,
    Out,
}

pub trait SignalDirection: Clone {
    const DIRECTION: Direction;
}

#[derive(Default, Clone, Debug)]
pub struct In {}

#[derive(Default, Clone, Debug)]
pub struct Out {}

impl SignalDirection for In {
    const DIRECTION: Direction = Direction::In;
}

impl SignalDirection for Out {
    const DIRECTION: Direction = Direction::Out;
}

pub trait EventType {}

// look into suggested event types
pub enum Event {
    Invoke,
    Intermediate,
    Finalize,
}

impl EventType for Event {}

pub trait DataType {}

// TODO: look into valid types
pub enum Data {
    Boolean(bool),
    Integer(i32),
}

impl DataType for Data {}

#[derive(Clone, Debug)]
pub struct EventSignal<S: SignalDirection, E: EventType> {
    pub direction: S,
    pub event: E,
}

#[derive(Clone, Debug)]
pub struct DataSignal<S: SignalDirection, D: DataType> {
    pub direction: S,
    pub data: D,
}
