//! Basic implementation of the `Voter` function block. Here event signals are
//! represented as booleans.

#[allow(dead_code)]
#[derive(Debug, Default)]
enum State {
    #[default]
    Ready,
    Vote,
    VotedPos,
    Reset,
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

// behaviour instances
impl Voter {
    pub fn new() -> Self {
        Voter {
            ecc_state: State::Ready,
            ein_vote: false,
            ein_reset: false,
            eout_voted: false,
            eout_ready: false,
            din_a: false,
            din_b: false,
            din_c: false,
            dout_state: false,
        }
    }
}

// ECC
impl Voter {
    pub fn invoke_ecc(&mut self) {
        self.eout_voted = false;
        self.eout_ready = false;

        match self.ecc_state {
            State::Ready => {
                if self.ein_vote {
                    self.ecc_state = State::Vote;
                }
            }
            State::Vote => {
                self.vote_algorithm();

                self.eout_voted = true;

                if self.dout_state {
                    self.ecc_state = State::VotedPos;
                    return;
                }

                self.ecc_state = State::Ready;
            }
            State::VotedPos => {
                if self.ein_reset {
                    self.ecc_state = State::Reset;
                }
            }
            State::Reset => {
                self.reset_algorithm();

                self.eout_ready = true;
                self.ecc_state = State::Ready;
            }
        }

        self.ein_vote = false;
        self.ein_reset = false;
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

pub enum Sequence {
    PositiveVote,
    NegativeVote,
    VotedReset,
    UnvotedReset,
}

pub fn run_sequence(sequence: Sequence) {
    let mut voter = Voter::new();

    match sequence {
        Sequence::PositiveVote => {
            voter.set_input_data("a", true);
            voter.set_input_data("c", true);
            println!();

            voter.receive_input_event("vote");

            println!("Ready: {voter:?}\n");

            voter.invoke_ecc();

            println!("Ready -> Vote: {voter:?}\n");

            voter.invoke_ecc();

            println!("Vote -> VotedPos: {voter:?}\n");
        }
        Sequence::NegativeVote => {
            voter.set_input_data("a", true);

            voter.receive_input_event("vote");

            println!("Ready: {voter:?}");

            voter.invoke_ecc();

            println!("Ready -> Vote: {voter:?}");

            voter.invoke_ecc();

            println!("Vote -> Ready: {voter:?}");
        }
        Sequence::VotedReset => {
            voter.set_input_data("a", true);
            voter.set_input_data("c", true);
            println!();

            voter.receive_input_event("vote");

            println!("Ready: {voter:?}\n");

            voter.invoke_ecc();

            println!("Ready -> Vote: {voter:?}\n");

            voter.invoke_ecc();

            println!("Vote -> VotedPos: {voter:?}\n");

            voter.receive_input_event("reset");

            voter.invoke_ecc();

            println!("VotedPos -> Reset: {voter:?}\n");

            voter.invoke_ecc();

            println!("Reset -> Ready: {voter:?}\n");
        }
        Sequence::UnvotedReset => {
            voter.receive_input_event("reset");

            println!("Ready: {voter:?}");

            voter.invoke_ecc();

            println!("State after unvoted reset: {voter:?}");
        }
    }
}
