use super::data::Data;
use super::data_type::Bool;
use super::direction::{In, Out};
use super::event::{Event, Signal};

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
    ecc: VoterState,
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
    #[allow(dead_code, clippy::nonminimal_bool)]
    fn vote_algorithm(&mut self) {
        let a = *self.a.read();
        let b = *self.b.read();
        let c = *self.c.read();

        let vote = (a && b) || (b && c) || (a && c);

        self.state.write(vote);
    }

    #[allow(dead_code)]
    fn reset_algorithm(&mut self) {
        self.state.write(false);
    }
}

impl Voter {
    #[allow(dead_code, unreachable_code)]
    /// Advances the current ecc state.
    /// Returns `true` if the state has changed.
    fn invoke_ecc(&mut self) -> bool {
        let _ecc_changed = false;

        match self.ecc {
            VoterState::Ready => {
                todo!("implement events")
            }
            VoterState::Vote => {
                self.vote_algorithm();
                todo!("implement events")
            }
            VoterState::VotedPos => {
                todo!("implement events")
            }
            VoterState::Reset => {
                self.reset_algorithm();
                todo!("implement events")
            }
        }

        _ecc_changed
    }
}
