use clap::Parser;
use iec_61499_fb_rs::{
    cli::args::{Args, FunctionBlock, Mode},
    fb_impl, run_time_impl,
};

fn main() {
    let args: Args = Args::parse();

    use FunctionBlock::*;
    use Mode::*;

    #[allow(unreachable_patterns)] // might be extended later
    match (args.function_block, args.mode) {
        (VoterBasic, Sequence) => {
            fb_impl::voter::basic::run_sequence(args.sequence);
        }
        (VoterTyped, Sequence) => {
            fb_impl::voter::typed::run_sequence(args.sequence);
        }
        (VoterTyped, Interactive) => {
            run_time_impl::interactive::simple_typed_runtime();
        }
        (_, TestConnectionParallel) => {
            run_time_impl::conn_test::test_rc_conn_par_voter();
        }
        (_, TestConnectionSequential) => {
            run_time_impl::conn_test::test_id_conn_seq_voter();
        }
        (implementation, mode) => {
            println!("combination of \"{implementation}\"  and \"{mode}\" is not configured.");
        }
    }
}
