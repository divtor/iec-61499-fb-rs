use clap::Parser;
use iec_61499_fb_rs::{
    cli::args::{Args, Mode, Voter},
    fb_impl, run_time_impl,
};

fn main() {
    let args: Args = Args::parse();

    use Mode::*;
    use Voter::*;

    #[allow(unreachable_patterns)] // might be extended later
    match (args.voter, args.mode) {
        (Basic, Sequence) => {
            fb_impl::voter_basic::run_sequence(args.sequence);
        }
        (Typed, Sequence) => {
            fb_impl::voter_typed::run_sequence(args.sequence);
        }
        (Typed, Interactive) => {
            run_time_impl::interactive::simple_traited_runtime();
        }
        (DynamicInRuntime, TestConnectionParallel) => {
            run_time_impl::test_connections_par();
        }
        (DynamicInRuntime, TestConnectionSequential) => {
            run_time_impl::test_connections_seq();
        }
        (Basic | Typed, mode) => {
            println!("mode \"{mode}\" is not implemented yet!");
        }
        (implementation, _) => {
            println!("voter implementation \"{implementation}\" does not exist!");
        }
    }
}
