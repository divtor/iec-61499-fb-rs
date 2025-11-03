use std::io::Write;

use crate::fb::{
    self,
    traited::voter::{invoke_ecc_once, receive_signal, run_voter_until_stable, toggle_input_data},
};

pub fn simple_traited_runtime() {
    let mut voter = fb::traited::voter::Voter::default();

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut buf = String::new();

        if std::io::stdin().read_line(&mut buf).is_err() {
            println!("Error reading command.");
            continue;
        }

        let input = buf.trim();

        if input.is_empty() {
            continue;
        }

        let mut cmd_parts = input.split_whitespace();
        let cmd = cmd_parts.next().unwrap();
        let args: Vec<&str> = cmd_parts.collect();

        match cmd {
            "help" => println!(
                "Available commands: quit, run, step, receive_input_event <name>, toggle_input_data <name>"
            ),
            "run" => run_voter_until_stable(&mut voter),
            "step" => invoke_ecc_once(&mut voter),
            "receive_input_event" => {
                if let Some(event) = args.first() {
                    receive_signal(&mut voter, event);
                }
            }
            "toggle_input_data" => {
                if let Some(data) = args.first() {
                    toggle_input_data(&mut voter, data);
                }
            }
            "quit" => {
                break;
            }
            _ => println!("Unknown command \"{cmd}\""),
        }

        println!("{voter:?}");
    }
}

