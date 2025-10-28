use std::time::Duration;

use crate::prototype::signals::direction::Direction;

/// Represents a data input or output.
#[derive(Clone, Debug, Default)]
pub struct Data<D: Direction, T: DataType> {
    pub direction: D,
    pub data: T,
}

pub trait DataType {
    const TYPE: DataTypes;
}

/// Implements the `DataType` trait for a struct.
/// An enum variant in `DataTypes` with the same identifier as the struct needs to exist.
macro_rules! impl_data_type {
    ($t:ident) => {
        impl DataType for $t {
            const TYPE: DataTypes = DataTypes::$t;
        }
    };
}

/// `IEC 61131-3` data types
pub enum DataTypes {
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

#[derive(Clone, Debug, Default)]
pub struct SInt(pub i8);
impl_data_type!(SInt);

#[derive(Clone, Debug, Default)]
pub struct Int(pub i16);
impl_data_type!(Int);

#[derive(Clone, Debug, Default)]
pub struct DInt(pub i32);
impl_data_type!(DInt);

#[derive(Clone, Debug, Default)]
pub struct LInt(pub i64);
impl_data_type!(LInt);

#[derive(Clone, Debug, Default)]
pub struct USInt(pub i8);
impl_data_type!(USInt);

#[derive(Clone, Debug, Default)]
pub struct UInt(pub i16);
impl_data_type!(UInt);

#[derive(Clone, Debug, Default)]
pub struct UDInt(pub i32);
impl_data_type!(UDInt);

#[derive(Clone, Debug, Default)]
pub struct ULInt(pub i64);
impl_data_type!(ULInt);

#[derive(Clone, Debug, Default)]
pub struct Real(pub f32);
impl_data_type!(Real);

#[derive(Clone, Debug, Default)]
pub struct LReal(pub f64);
impl_data_type!(LReal);

#[derive(Clone, Debug, Default)]
pub struct Time(pub Duration);
impl_data_type!(Time);

#[derive(Clone, Debug, Default)]
pub struct Date;
impl_data_type!(Date);

#[derive(Clone, Debug, Default)]
pub struct TimeOfDay;
impl_data_type!(TimeOfDay);

#[derive(Clone, Debug, Default)]
pub struct DateTime;
impl_data_type!(DateTime);

#[derive(Clone, Debug, Default)]
pub struct String(pub Vec<u8>);
impl_data_type!(String);

#[derive(Clone, Debug, Default)]
pub struct WString(pub Vec<u16>);
impl_data_type!(WString);

#[derive(Clone, Debug, Default)]
pub struct Bool(pub bool);
impl_data_type!(Bool);

#[derive(Clone, Debug, Default)]
pub struct Byte(pub u8);
impl_data_type!(Byte);

#[derive(Clone, Debug, Default)]
pub struct Word(pub u16);
impl_data_type!(Word);

#[derive(Clone, Debug, Default)]
pub struct DWord(pub u32);
impl_data_type!(DWord);

#[derive(Clone, Debug, Default)]
pub struct LWord(pub u64);
impl_data_type!(LWord);
