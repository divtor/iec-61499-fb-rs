#[derive(Clone, Debug)]
pub enum Dir {
    In,
    Out,
}

pub trait Direction: Clone {
    const DIR: Dir;
}

#[derive(Default, Clone, Debug)]
pub struct In {}

#[derive(Default, Clone, Debug)]
pub struct Out {}

impl Direction for In {
    const DIR: Dir = Dir::In;
}

impl Direction for Out {
    const DIR: Dir = Dir::Out;
}
