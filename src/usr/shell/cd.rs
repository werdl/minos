//! cd - change directory

use crate::api::process::ExitCode;
use alloc::string::String;
use alloc::format;
use crate::sys::process;

pub fn main(args: &[&str]) -> ExitCode {
    match args.len() {
        0 => {
            process::set_dir("/");
        }
        1 => {
            let path = crate::sys::fs::canonicalize(args[0]).unwrap();
            let is_root = process::dir().as_str() == "/";
            process::set_dir(format!("{}{}{}", process::dir(), if is_root { "" } else { "/" }, path).as_str());
        }
        _ => {
            println!("Too many arguments");
            return ExitCode::UsageError;
        }
    }
    ExitCode::Success
}