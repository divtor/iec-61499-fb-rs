use super::{
    data_type::DataType,
    direction::{Direction, In, Out},
};

#[allow(dead_code)]
/// Represents a data input or output.
#[derive(Clone, Debug, Default)]
pub struct Data<D: Direction, T: DataType> {
    direction: D,
    data: T,
}

/// impl for both directions
impl<D: Direction, T: DataType> Data<D, T> {
    pub fn read(&self) -> &<T as DataType>::Inner {
        self.data.get()
    }
}

/// impl for `In`
impl<T: DataType> Data<In, T> {
    pub fn update(&mut self) {}
}

/// impl for `Out`
impl<T: DataType> Data<Out, T> {
    pub fn write(&mut self, value: <T as DataType>::Inner) {
        self.data.set(value);
    }
}
