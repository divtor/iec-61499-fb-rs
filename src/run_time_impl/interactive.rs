//! Interactive test bench of a single `voter:typed::Voter` instance.

use std::io::Write;

use crate::fb_impl::voter::typed;

pub fn simple_typed_runtime() {
    let mut voter = typed::Voter::default();

    loop {
        println!("{voter}");
        println!("commands: quit, run, step, rs <signal_name>, tid <input_data_name>");
        print!("> ");

        std::io::stdout().flush().unwrap();

        let mut buf = String::new();

        if std::io::stdin().read_line(&mut buf).is_err() {
            println!("Error reading command.");
            continue;
        }

        let buf = buf.trim();

        if buf.is_empty() {
            continue;
        }

        let mut cmd = buf.split_whitespace();

        match cmd.next().unwrap() {
            "run" => {
                voter.run_ecc();
            }
            "step" => {
                _ = voter.invoke_ecc();
            }
            "rs" => {
                if let Some(event) = cmd.next() {
                    voter.receive_signal(event);
                }
            }
            "tid" => {
                if let Some(data) = cmd.next() {
                    voter.toggle_input_data(data);
                }
            }
            "quit" => {
                break;
            }
            unknown => println!("Unknown command \"{unknown}\""),
        }
    }
}
