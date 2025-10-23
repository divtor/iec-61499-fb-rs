use iec_61499_fb_rs::prototype::basic::{self, Sequence::*};

fn main() {
    let sequence = VotedReset;
    basic::run_sequence(sequence);
}
