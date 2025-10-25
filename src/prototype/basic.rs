//! Basic implementation of the `Voter` function block. Here event signals are
//! represented as booleans.

use core::fmt;
use std::fmt::Display;

use crate::cli::{args::Sequence, output::voter_fb_string};

#[allow(dead_code)]
#[derive(Debug, Default)]
enum State {
    #[default]
    Ready,
    Vote,
    VotedPos,
    Reset,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Voter {
    ecc_state: State,

    // Event Input
    ein_vote: bool,
    ein_reset: bool,

    // Event Output
    eout_voted: bool,
    eout_ready: bool,

    // Data Input
    din_a: bool,
    din_b: bool,
    din_c: bool,

    // Data Output
    dout_state: bool,
}

impl Display for Voter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let buf = voter_fb_string(
            self.ecc_state.to_string().as_str(),
            self.ein_vote.to_string().as_str(),
            self.ein_reset.to_string().as_str(),
            self.eout_voted.to_string().as_str(),
            self.eout_ready.to_string().as_str(),
            self.din_a.to_string().as_str(),
            self.din_b.to_string().as_str(),
            self.din_c.to_string().as_str(),
            self.dout_state.to_string().as_str(),
        );

        write!(f, "{buf}")
    }
}

// ECC
impl Voter {
    fn invoke_ecc(&mut self) -> bool {
        let mut state_changed = false;

        self.eout_voted = false;
        self.eout_ready = false;

        match self.ecc_state {
            State::Ready => {
                if self.ein_vote {
                    self.ecc_state = State::Vote;
                    state_changed = true;
                }
            }
            State::Vote => {
                self.vote_algorithm();

                self.eout_voted = true;

                if self.dout_state {
                    self.ecc_state = State::VotedPos;
                } else {
                    self.ecc_state = State::Ready;
                }

                state_changed = true;
            }
            State::VotedPos => {
                if self.ein_reset {
                    self.ecc_state = State::Reset;
                    state_changed = true;
                }
            }
            State::Reset => {
                self.reset_algorithm();

                self.eout_ready = true;
                self.ecc_state = State::Ready;
                state_changed = true;
            }
        }

        self.ein_vote = false;
        self.ein_reset = false;

        state_changed
    }

    pub fn invoke_until_stable(&mut self) {
        let mut invoke = true;

        while invoke {
            invoke = self.invoke_ecc();
        }
    }
}

// sequences
impl Voter {
    /// Definition in `IEC 61131-3 Structured Text`:
    /// ```
    /// ALGORITHM VoteAlg IN ST:
    ///     State := (A AND B) OR (A AND C) OR (B AND C);
    /// END_ALGORITHM
    /// ```
    #[allow(clippy::nonminimal_bool)]
    fn vote_algorithm(&mut self) {
        self.dout_state =
            (self.din_a && self.din_b) || (self.din_a && self.din_c) || (self.din_b && self.din_c);
    }

    /// Definition in `IEC 61131-3 Structured Text`:
    /// ```
    /// ALGORITHM ResetAlg IN ST:
    ///     State := FALSE;
    /// END_ALGORITHM
    /// ```
    fn reset_algorithm(&mut self) {
        self.dout_state = false;
    }
}

// * probably abstractable in a cleaner way, look into this in other versions
//  - events: Event<In/Out, EventType>
//  - data: Data<In/Out, DataType>
//
// * problem: might not be clean easy to generate from structured text
impl Voter {
    pub fn receive_input_event(&mut self, event_str: &str) {
        let mut unkown = false;

        match event_str.to_lowercase().as_str() {
            "vote" => self.ein_vote = true,
            "reset" => self.ein_reset = true,
            _ => unkown = true,
        }

        if unkown {
            println!("unkown input event \"{event_str}\"");
            return;
        }

        println!("received input event \"{event_str}\"");
    }

    pub fn check_output_event(&self, event_str: &str) -> Option<bool> {
        match event_str.to_lowercase().as_str() {
            "voted" => Some(self.eout_voted),
            "ready" => Some(self.eout_ready),
            _ => None,
        }
    }

    pub fn set_input_data(&mut self, data_str: &str, value: bool) {
        let mut unknown = false;

        match data_str.to_lowercase().as_str() {
            "a" => self.din_a = value,
            "b" => self.din_b = value,
            "c" => self.din_c = value,
            _ => unknown = true,
        }

        if unknown {
            println!("unknown input data \"{data_str}\"");
        }

        println!("set input data \"{data_str}\" to {value}");
    }

    pub fn get_output_data(&self, data_str: &str) -> Option<bool> {
        match data_str.to_lowercase().as_str() {
            "state" => Some(self.dout_state),
            _ => None,
        }
    }
}

pub fn run_sequence(sequence: Sequence) {
    let mut voter = Voter::default();

    match sequence {
        Sequence::PositiveVote => {
            voter.set_input_data("a", true);
            voter.set_input_data("c", true);
            println!();

            voter.receive_input_event("vote");

            println!("PositiveVote setup\n {voter}");

            voter.invoke_until_stable();

            println!("Stable state after\n {voter}");
        }
        Sequence::NegativeVote => {
            voter.set_input_data("a", true);
            println!();

            voter.receive_input_event("vote");

            println!("Negative Vote setup\n {voter}");

            voter.invoke_until_stable();

            println!("Stable state after\n {voter}");
        }
        Sequence::VotedReset => {
            voter.set_input_data("a", true);
            voter.set_input_data("c", true);
            println!();

            voter.receive_input_event("vote");

            println!("PositiveVote setup\n {voter}");

            voter.invoke_until_stable();

            println!("Stable state after\n {voter}");

            voter.receive_input_event("reset");

            println!("Reset setup\n {voter}");

            voter.invoke_until_stable();

            println!("Stable state after\n {voter}");
        }
        Sequence::UnvotedReset => {
            voter.receive_input_event("reset");

            println!("Unvoted Reset setup\n {voter}");

            voter.invoke_until_stable();

            println!("Stable state after\n {voter}");
        }
    }
}
