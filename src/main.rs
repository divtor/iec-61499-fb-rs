use clap::Parser;
use iec_61499_fb_rs::{
    cli::args::{Args, Implementation, Mode},
    prototype,
};

fn main() {
    let args: Args = Args::parse();

    use Implementation::*;
    use Mode::*;

    match (args.implementation, args.mode) {
        (PrototypeBasic, Sequence) => {
            prototype::basic::run_sequence(args.sequence);
        }
        (PrototypeSignals, Sequence) => {
            prototype::signals::voter::execute_sequence(args.sequence);
        }
        (PrototypeBasic | PrototypeSignals, mode) => {
            println!("mode \"{mode}\" is not implemented yet!");
        }
        (implementation, _) => {
            println!("implementation \"{implementation}\" does not exist!");
        }
    }
}
