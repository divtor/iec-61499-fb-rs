use std::{any::Any, fmt::Debug};

use data::comm::DataBuffer;

pub mod data;
pub mod direction;
pub mod event;

// NOTE:
// - Current implementation of the execution control handling is NOT flexible enough
// - Future implementations need to enable flexible handling and communication with the scheduler
// - For a PoC of function blocks and communication this suffices

// currently only used by `voter_dynamic`

/// trait to enable structs to be handled as function blocks by the run time
pub trait Fb: Any + Debug {
    /// enables dynamic dispatch downcasting (e.g. dyn Fb -> Voter)
    fn as_any(&self) -> &dyn Any;

    /// returns the instance name of the function block
    fn instance_name(&self) -> &'static str;

    /// set an input event to active
    fn receive_event(&mut self, event: &str);

    /// returns the currently active in event, if there is any
    fn active_in_event(&self) -> Option<&'static str>;

    /// returns the currently active out event, if there is any
    fn active_out_event(&self) -> Option<&'static str>;

    /// clears the current active out event
    fn clear_out_event(&mut self);

    /// returns the field names of data in- or output associated WITH the given event
    fn event_associations(&self, event_str: &str) -> Vec<&'static str>;

    /// gets the current value of output data as a buffer value
    fn read_out_data(&self, data_str: &str) -> DataBuffer;

    /// sets the value of an input data to the value inside given buffer
    fn write_in_data(&mut self, data_str: &str, buf: &DataBuffer);

    /// executes a single step of the function block execution control, returns a flag whether the state after the step is unstable
    fn invoke_execution_control(&mut self) -> bool;
}
