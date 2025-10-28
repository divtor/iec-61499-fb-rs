use data::{Bool, Data};
use direction::{In, Out};
use event::{Event, Signal};

pub mod data;
pub mod direction;
pub mod event;

// TODO: figure out:
// - how to associate `Data` to `Event` (WITH qualifier)
// - how to fetch new values on associated `Data`
// - how to represent `Event` invocation (Needs to be discussed)
// - implement the Voter Block with signals

#[derive(Clone, Debug, Default)]
pub enum VoterState {
    #[default]
    Ready,
    Vote,
    VotedPos,
    Reset,
}

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub struct Voter {
    ecc_state: VoterState,
    vote: Event<In, Signal>,
    reset: Event<In, Signal>,
    voted: Event<Out, Signal>,
    ready: Event<Out, Signal>,
    a: Data<In, Bool>,
    b: Data<In, Bool>,
    c: Data<In, Bool>,
    state: Data<Out, Bool>,
}

impl Voter {
    #[allow(dead_code)]
    fn vote_algorithm(&mut self) {
        let _tmp = self.a.data.0;
    }
}
