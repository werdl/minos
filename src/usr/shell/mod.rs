pub mod cat;
pub mod cd;
pub mod list;
pub mod install;
pub mod time;
pub mod unix_core;

use alloc::vec::Vec;
use alloc::format;

use crate::api;
use crate::sys::process;

fn inner(args: Vec<&str>) -> api::process::ExitCode {
    match args[0] {
        "ls" => list::main(&args[1..]),
        "install" => install::main(&args[1..]),
        "cd" => cd::main(&args[1..]),
        "cat" => cat::main(&args[1..]),
        "time" => time::main(&args[1..]),
        "cp" => unix_core::cp(&args[1..]),
        "mv" => unix_core::mv(&args[1..]),
        "rm" => unix_core::rm(&args[1..]),
        "mkdir" => unix_core::mkdir(&args[1..]),
        "canon" => {
            if args.len() != 2 {
                println!("Usage: canon <path>");
                return api::process::ExitCode::UsageError;
            }

            let path = crate::sys::fs::canonicalize(args[1]).unwrap();
            println!("{}", path);
            api::process::ExitCode::Success
        },


        "cwd" => {
            println!("{}", process::dir());
            api::process::ExitCode::Success
        },

        _ => {
            println!("Unknown command: {}", args[0]);

            api::process::ExitCode::ExecError
        },
    }
}

pub fn main() {
    // main shell instance
    let mut prompter = api::prompt::Prompt::new();

    let mut last_exit_code = api::process::ExitCode::Success;

    // execute /ini/boot.sh
    let boot_script = crate::api::io::fs::read_to_string("/ini/boot.sh").expect("Failed to read /ini/boot.sh");

    for line in boot_script.lines() {
        let args: Vec<&str> = line.split_whitespace().collect();
        last_exit_code = inner(args);
    }

    loop {
        let input = prompter.input(format!("\x1b[34m{}\x1b[0m $ ", process::dir()).as_str()).expect("Failed to read input");
        let args: Vec<&str> = input.split_whitespace().collect();

        if args.is_empty() {
            continue;
        }

        last_exit_code = match args[0] {
            "exit" | "quit" => {
                break;
            },
            "$?" => {
                println!("{}", last_exit_code as usize);
                api::process::ExitCode::Success
            },
            _ => {
                inner(args)
            }
        };
    }

    println!("Goodbye!");
    
    crate::sys::acpi::shutdown();
}