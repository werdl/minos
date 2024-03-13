pub mod cat;
pub mod cd;
pub mod list;
pub mod install;
pub mod time;

use alloc::vec::Vec;
use alloc::format;

use crate::api;
use crate::sys::process;

pub fn main() {
    // main shell instance

    println!("minos shell - v0.1.0");

    let mut prompter = api::prompt::Prompt::new();

    let mut last_exit_code = api::process::ExitCode::Success;



    loop {
        let input = prompter.input(format!("{} $ ", process::dir()).as_str()).expect("Failed to read input");
        let args: Vec<&str> = input.split_whitespace().collect();

        if args.is_empty() {
            continue;
        }

        last_exit_code = match args[0] {
            "ls" => list::main(&args[1..]),
            "install" => install::main(&args[1..]),
            "cd" => cd::main(&args[1..]),
            "cat" => cat::main(&args[1..]),
            "time" => time::main(&args[1..]),


            "cwd" => {
                println!("{}", process::dir());
                api::process::ExitCode::Success
            },
            "$?" => {
                println!("{}", last_exit_code as usize);
                api::process::ExitCode::Success
            },
            "quit" | "exit" => {
                break;
            },
            _ => {
                println!("Unknown command: {}", args[0]);

                api::process::ExitCode::ExecError
            },
        };
    }

    println!("Goodbye!");
    
    crate::sys::acpi::shutdown();
}