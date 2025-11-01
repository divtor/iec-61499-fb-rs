use clap::{Parser, ValueEnum};
use std::fmt;

#[derive(Clone, Debug, clap::ValueEnum, Default)]
pub enum Implementation {
    #[default]
    PrototypeBasic,
    PrototypeSignals,
    Runtime,
}

impl fmt::Display for Implementation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_possible_value().unwrap().get_name())
    }
}

#[derive(Clone, Debug, clap::ValueEnum, Default)]
pub enum Mode {
    #[default]
    Sequence,
    Interactive,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_possible_value().unwrap().get_name())
    }
}

#[derive(Clone, Debug, clap::ValueEnum, Default)]
pub enum Sequence {
    PositiveVote,
    NegativeVote,
    #[default]
    VotedReset,
    UnvotedReset,
}

impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_possible_value().unwrap().get_name())
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(
        short,
        long,
        default_value_t = Implementation::PrototypeBasic,
        help = "Choose implementation"
    )]
    pub implementation: Implementation,

    #[arg(short, long, default_value_t = Mode::Sequence, help = "Choose mode")]
    pub mode: Mode,

    #[arg(
        short,
        long,
        default_value_t = Sequence::VotedReset,
        help = "Choose sequence; only applied if mode=sequence"
    )]
    pub sequence: Sequence,
}
