//! Basic implementation of the `Voter` function block. Here event signals are
//! represented as booleans.

#[allow(dead_code)]
enum State {
    Ready,
    Vote,
    VotedPos,
    Reset,
}

#[allow(dead_code)]
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

// constructors
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
    pub fn invoke_ecc(&mut self) -> () {
        match self.ecc_state {
            State::Ready => {
                if self.ein_vote {
                    self.ecc_state = State::Vote;
                }
            }
            State::Vote => {
                self.vote_sequence();

                if self.dout_state {
                    self.ecc_state = State::VotedPos;
                } else {
                    self.ecc_state = State::Ready;
                }
            }
            State::VotedPos => {
                if self.ein_reset {
                    self.ecc_state = State::Reset;
                }
            }
            State::Reset => {
                self.reset_sequence();

                self.ecc_state = State::Ready;
            }
        }
    }
}

// sequences
impl Voter {
    fn vote_sequence(&mut self) -> () {
        self.dout_state =
            (self.din_a && self.din_b) || (self.din_a && self.din_c) || (self.din_b && self.din_c);
        self.eout_voted = true;
    }

    fn reset_sequence(&mut self) -> () {
        self.dout_state = false;
    }
}
