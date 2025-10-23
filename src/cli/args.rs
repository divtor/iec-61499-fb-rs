use clap::Parser;
use std::fmt;

#[derive(Clone, Debug, clap::ValueEnum, Default)]
pub enum Implementation {
    #[default]
    PrototypeBasic,
    PrototypeEnum,
}

impl fmt::Display for Implementation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = Implementation::PrototypeBasic, help = "Choose implementation")]
    pub implementation: Implementation,
}
