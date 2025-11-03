pub trait Direction: Clone {}

#[derive(Default, Clone, Debug)]
pub struct In {}

impl Direction for In {}

#[derive(Default, Clone, Debug)]
pub struct Out {}

impl Direction for Out {}
