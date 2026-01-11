use clap::{Parser, ValueEnum};
use std::fmt;

#[derive(Clone, Debug, clap::ValueEnum, Default)]
pub enum FunctionBlock {
    #[default]
    VoterBasic,
    VoterTyped,
    VoterDynamic,
    VoterDynamicInRuntime,
    Ctu,
    Sr,
    Switch,
}

impl fmt::Display for FunctionBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_possible_value().unwrap().get_name())
    }
}

#[derive(Clone, Debug, clap::ValueEnum, Default)]
pub enum Mode {
    #[default]
    Sequence,
    Interactive,
    TestConnectionParallel,
    TestConnectionSequential,
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
#[command(
    version,
    about = "CLI tool to test IEC 61499 implemenations.",
    long_about = "CLI tool to test IEC 61499 implemenations.",
    after_help = "-f only matters if -m is \"sequence\" or \"interactive\"\n-s only matters if -m is \"sequence\"",
    override_usage = "iec-61499-fb-rs.exe -m [MODE] -f [FUNCTION_BLOCK] -s [SEQUENCE]"
)]
pub struct Args {
    #[arg(
        short,
        long,
        default_value_t = FunctionBlock::VoterBasic,
    )]
    pub function_block: FunctionBlock,

    #[arg(short, long, default_value_t = Mode::Sequence)]
    pub mode: Mode,

    #[arg(
        short,
        long,
        default_value_t = Sequence::VotedReset
    )]
    pub sequence: Sequence,
}
