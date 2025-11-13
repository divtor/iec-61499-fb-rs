//! internal data fields in function blocks

use super::direction::{Direction, In, Out};

use ty::DataType;

/// Represents a data input or output.
#[derive(Clone, Debug, Default)]
pub struct Data<D: Direction, T: ty::DataType> {
    _direction_marker: std::marker::PhantomData<D>,
    value: T,
}

impl<D: Direction, T: ty::DataType> Data<D, T> {
    pub fn read(&self) -> &<T as ty::DataType>::Inner {
        self.value.get()
    }
}

impl<T: ty::DataType> Data<In, T> {
    pub fn update(&mut self, value: <T as ty::DataType>::Inner) {
        self.value.set(value);
    }
}

impl<T: ty::DataType> Data<Out, T> {
    pub fn write(&mut self, value: <T as ty::DataType>::Inner) {
        self.value.set(value);
    }
}

pub fn toggle<D: Direction>(data: &mut Data<D, ty::Bool>) {
    let old = *data.value.get();
    data.value.set(!old);
}

/// dynamic communication type system between function blocks
pub mod comm {
    use std::time::Duration;

    /// enum to enable a type-safe runtime communication of `IEC 61131-3` data types between function blocks
    #[derive(Default, Clone, Debug)]
    pub enum CommunicationData {
        SInt(i8),
        Int(i16),
        DInt(i32),
        LInt(i64),
        USInt(u8),
        UInt(u16),
        UDInt(u32),
        ULInt(u64),
        Real(f32),
        LReal(f64),
        Time(Duration),
        Date(Vec<u8>),
        TimeOfDay(Vec<u8>),
        DateTime(Vec<u8>),
        WString(Vec<u16>),
        String(Vec<u8>),
        Bool(bool),
        Byte(u8),
        Word(u16),
        DWord(u32),
        LWord(u64),
        #[default]
        Unassigned,
    }
}

/// static inner data type system of function blocks
pub mod ty {
    use std::time::Duration;

    /// `IEC 61131-3` data type markers
    pub enum DataTypeMarker {
        SInt,
        Int,
        DInt,
        LInt,
        USInt,
        UInt,
        UDInt,
        ULInt,
        Real,
        LReal,
        Time,
        Date,
        TimeOfDay,
        DateTime,
        String,
        WString,
        Bool,
        Byte,
        Word,
        DWord,
        LWord,
    }

    /// Enables usage of implementing structs in `Data<Direction, DataType>`
    pub trait DataType {
        type Inner;

        fn kind(&self) -> DataTypeMarker;
        fn get(&self) -> &Self::Inner;
        fn set(&mut self, value: Self::Inner) -> ();
    }

    /// Implements the `DataType` trait for a given struct.
    /// Requirement: the struct needs to have a field called `data`.
    macro_rules! impl_data_type {
        ($name:ident, $inner:ty) => {
            impl DataType for $name {
                type Inner = $inner;

                fn kind(&self) -> DataTypeMarker {
                    DataTypeMarker::$name
                }

                fn get(&self) -> &Self::Inner {
                    &self.data
                }

                fn set(&mut self, value: Self::Inner) {
                    self.data = value;
                }
            }
        };
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub struct SInt {
        data: i8,
    }
    impl_data_type!(SInt, i8);

    #[derive(Clone, Copy, Debug, Default)]
    pub struct Int {
        data: i16,
    }
    impl_data_type!(Int, i16);

    #[derive(Clone, Copy, Debug, Default)]
    pub struct DInt {
        data: i32,
    }
    impl_data_type!(DInt, i32);

    #[derive(Clone, Copy, Debug, Default)]
    pub struct LInt {
        data: i64,
    }
    impl_data_type!(LInt, i64);

    #[derive(Clone, Copy, Debug, Default)]
    pub struct USInt {
        data: u8,
    }
    impl_data_type!(USInt, u8);

    #[derive(Clone, Debug, Default)]
    pub struct UInt {
        data: u16,
    }
    impl_data_type!(UInt, u16);

    #[derive(Clone, Debug, Default)]
    pub struct UDInt {
        data: u32,
    }
    impl_data_type!(UDInt, u32);

    #[derive(Clone, Debug, Default)]
    pub struct ULInt {
        data: u64,
    }
    impl_data_type!(ULInt, u64);

    #[derive(Clone, Debug, Default)]
    pub struct Real {
        data: f32,
    }
    impl_data_type!(Real, f32);

    #[derive(Clone, Debug, Default)]
    pub struct LReal {
        data: f64,
    }
    impl_data_type!(LReal, f64);

    #[derive(Clone, Debug, Default)]
    pub struct Time {
        data: Duration,
    }
    impl_data_type!(Time, Duration);

    #[derive(Clone, Debug, Default)]
    pub struct Date {
        data: Vec<u8>, // NOTE: change this to custom implementation, use a "string" for now
    }
    impl_data_type!(Date, Vec<u8>);

    #[derive(Clone, Debug, Default)]
    pub struct TimeOfDay {
        data: Vec<u8>, // NOTE: change this to custom implementation, use a "string" for now
    }
    impl_data_type!(TimeOfDay, Vec<u8>);

    #[derive(Clone, Debug, Default)]
    pub struct DateTime {
        data: Vec<u8>, // NOTE: change this to custom implementation, use a "string" for now
    }
    impl_data_type!(DateTime, Vec<u8>);

    #[derive(Clone, Debug, Default)]
    pub struct String {
        data: Vec<u8>,
    }
    impl_data_type!(String, Vec<u8>);

    #[derive(Clone, Debug, Default)]
    pub struct WString {
        data: Vec<u16>,
    }
    impl_data_type!(WString, Vec<u16>);

    #[derive(Clone, Debug, Default)]
    pub struct Bool {
        data: bool,
    }
    impl_data_type!(Bool, bool);

    #[derive(Clone, Debug, Default)]
    pub struct Byte {
        data: u8,
    }
    impl_data_type!(Byte, u8);

    #[derive(Clone, Debug, Default)]
    pub struct Word {
        data: u16,
    }
    impl_data_type!(Word, u16);

    #[derive(Clone, Debug, Default)]
    pub struct DWord {
        data: u32,
    }
    impl_data_type!(DWord, u32);

    #[derive(Clone, Debug, Default)]
    pub struct LWord {
        data: u64,
    }
    impl_data_type!(LWord, u64);
}
