use clap::Parser;
use iec_61499_fb_rs::{
    cli::args::{Args, Implementation, Mode},
    prototype,
};

fn main() {
    let args: Args = Args::parse();

    match args.implementation {
        Implementation::PrototypeBasic => match args.mode {
            Mode::Sequence => {
                prototype::basic::run_sequence(args.sequence);
            }
            _ => println!("Mode \"{}\" is not implemented yet!", args.mode),
        },
        _ => println!(
            "Implementation \"{}\" does not exist yet!",
            args.implementation
        ),
    }
}
