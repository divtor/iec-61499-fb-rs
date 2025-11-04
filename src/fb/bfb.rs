pub trait BasicFunctionBlock {
    /// Advances the current ecc state once.
    /// Returns `true` if the state has changed -> unstable.
    fn invoke_ecc(&mut self) -> bool;

    /// Advances the current ecc state until it is stable.
    fn run_ecc(&mut self);

    /// Updates the associated input data of given input event.
    fn update_input(&mut self, in_event: &str);

    /// Publishes the associated output data of given output event.
    fn publish_output(&mut self, out_event: &str);
}
