use std::time::Duration;

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

/// `IEC 61131-3` data types
pub enum Data {
    SINT(i8),
    INT(i16),
    DINT(i32),
    LINT(i64),
    USINT(u8),
    UINT(u16),
    UDINT(u32),
    ULINT(u64),
    REAL(f32),
    LREAL(f64),
    TIME(Duration),
    DATE, // TODO
    TOD,  // TODO
    DT,   // TODO
    STRING(Vec<u8>),
    WSTRING(Vec<u16>),
    BOOL(bool),
    BYTE(u8),
    WORD(u16),
    DWORD(u32),
    LWORD(u64),
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
