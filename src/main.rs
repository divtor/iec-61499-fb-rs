use clap::Parser;
use iec_61499_fb_rs::{
    cli::args::{Args, Mode, Voter},
    fb, run_time,
};

fn main() {
    let args: Args = Args::parse();

    use Mode::*;
    use Voter::*;

    #[allow(unreachable_patterns)] // might be extended later
    match (args.voter, args.mode) {
        (Basic, Sequence) => {
            fb::basic::run_sequence(args.sequence);
        }
        (Traited, Sequence) => {
            fb::traited::run_sequence(args.sequence);
        }
        (Traited, Interactive) => {
            run_time::interactive::simple_traited_runtime();
        }
        (Basic | Traited, mode) => {
            println!("mode \"{mode}\" is not implemented yet!");
        }
        (implementation, _) => {
            println!("voter implementation \"{implementation}\" does not exist!");
        }
    }
}
