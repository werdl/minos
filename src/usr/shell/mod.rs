pub mod cd;
pub mod list;
pub mod install;

use alloc::string::String;
use alloc::vec::Vec;

use crate::api;

pub fn main() {
    // main shell instance

    println!("minos shell - v0.1.0");

    let mut prompter = api::prompt::Prompt::new();

    let mut last_exit_code = api::process::ExitCode::Success;

    let mut cwd = String::from("/");


    loop {
        let input = prompter.input(">").expect("Failed to read input");
        let args: Vec<&str> = input.split_whitespace().collect();

        if args.is_empty() {
            continue;
        }

        last_exit_code = match args[0] {
            "ls" => list::main(cwd.clone(), &args[1..]),
            "install" => install::main(cwd.clone(), &args[1..]),
            "cd" => cd::main(&mut cwd, &args[1..]),
            "cwd" => {
                println!("{}", cwd);
                api::process::ExitCode::Success
            },
            "$?" => {
                println!("{}", last_exit_code as usize);
                api::process::ExitCode::Success
            },
            _ => {
                println!("Unknown command: {}", args[0]);

                api::process::ExitCode::ExecError
            },
        };
    }
}