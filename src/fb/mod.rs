use std::{
    any::Any,
    fmt::{Debug, Display},
};

use data::comm::DataBuffer;

use crate::fb::data::ty::DataKind;

pub mod data;
pub mod direction;
pub mod event;

/// trait to enable structs to be handled as basic function blocks by the run time
pub trait Bfb: Any + Debug + Display {
    /// enables dynamic dispatch downcasting (e.g. dyn Fb -> Voter)
    fn as_any(&self) -> &dyn Any;

    /// returns the instance name of the function block
    fn instance_name(&self) -> &'static str;

    /// returns the `DataKind` variant of the queried data field
    fn data_kind(&self, data: &str) -> DataKind;

    /// set an input event to active
    fn set_event_in(&mut self, event: &str);

    /// returns the currently active in event, if there is any
    fn active_event_in(&self) -> Option<&'static str>;

    /// returns the currently active out event, if there is any
    fn active_event_out(&self) -> Option<&'static str>;

    /// clears the current active out event
    fn clear_event_out(&mut self);

    /// returns the field names of data in- or output
    /// associated WITH the given event
    fn with_for_event(&self, event: &str) -> Vec<&'static str>;

    /// gets the current value of output data as a buffer value
    fn read_data_out(&self, data: &str) -> DataBuffer;

    /// sets the value of an input data to the value inside given buffer
    fn write_data_in(&mut self, data: &str, buf: &DataBuffer);

    /// executes a single step of the function block execution control,
    /// returns a flag whether the state after the step is unstable
    fn invoke_execution_control(&mut self) -> bool;
}
