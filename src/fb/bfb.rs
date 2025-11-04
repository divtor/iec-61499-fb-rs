pub trait BasicFunctionBlock {
    /// Advances the current ecc state.
    /// Returns `true` if the state has changed.
    fn invoke_ecc(&mut self) -> bool;

    fn run_ecc(&mut self);
    fn update_input(&mut self, in_event: &str);
    fn write_output(&mut self, out_event: &str);
}
