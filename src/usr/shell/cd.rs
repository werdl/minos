//! cd - change directory

use crate::api::process::ExitCode;
use alloc::string::String;

pub fn main(cwd: &mut String, args: &[&str]) -> ExitCode {
    match args.len() {
        0 => {
            *cwd = String::from("/");
        }
        1 => {
            let path = crate::sys::fs::canonicalize(args[0]).unwrap();
            *cwd = path;
        }
        _ => {
            println!("Too many arguments");
            return ExitCode::UsageError;
        }
    }
    ExitCode::Success
}