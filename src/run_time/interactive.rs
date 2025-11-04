use std::io::Write;

use crate::{fb::bfb::BasicFunctionBlock, fb_impl::voter::Voter};

pub fn simple_traited_runtime() {
    let mut voter = Voter::default();

    loop {
        println!("{voter}");
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
            "help" => {
                println!(
                    "Available commands: quit, run, step, rs <signal_name>, tid <input_data_name>"
                );
            }
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
