use super::{
    data_type::{self, DataType},
    direction::{Direction, In, Out},
};

/// Represents a data input or output.
#[derive(Clone, Debug, Default)]
pub struct Data<D: Direction, T: DataType> {
    _direction: D,
    value: T,
}

/// impl for both directions
impl<D: Direction, T: DataType> Data<D, T> {
    pub fn read(&self) -> &<T as DataType>::Inner {
        self.value.get()
    }
}

/// impl for `In`
impl<T: DataType> Data<In, T> {
    // TODO: discuss how to implement this (might not be callable from voter instance)
    pub fn update(&mut self) {}
}

/// impl for `Out`
impl<T: DataType> Data<Out, T> {
    pub fn write(&mut self, value: <T as DataType>::Inner) {
        self.value.set(value);
    }
}

pub fn toggle<D: Direction>(data: &mut Data<D, data_type::Bool>) {
    let old = *data.value.get();
    data.value.set(!old);
}
