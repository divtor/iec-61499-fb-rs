use std::time::Duration;

/// Enables generic utilization of defined `IEC 61131-3` data types
pub trait DataType {
    type Inner;

    fn kind(&self) -> DataTypeKind;
    fn get(&self) -> &Self::Inner;
    fn set(&mut self, value: Self::Inner) -> ();
}

/// `IEC 61131-3` data types
pub enum DataTypeKind {
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

/// Implements the `DataType` trait for a given struct.
/// Requirement: the struct needs to have a field called `data`.
macro_rules! impl_data_type {
    ($name:ident, $inner:ty) => {
        impl DataType for $name {
            type Inner = $inner;

            fn kind(&self) -> DataTypeKind {
                DataTypeKind::$name
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
