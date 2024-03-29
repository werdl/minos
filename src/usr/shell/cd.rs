//! cd - change directory

use crate::api::process::ExitCode;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
use crate::sys::process;

pub fn main(args: &[&str]) -> ExitCode {
    match args.len() {
        0 => {
            process::set_dir("/");
        }
        1 => {
            let path = crate::sys::fs::canonicalize(args[0]).unwrap();
            
            if crate::api::io::fs::exists(path.as_str()) {
                process::set_dir(path.as_str());
            } else {
                error!("Directory does not exist");
                return ExitCode::ExecError;
            }
        }
        _ => {
            error!("Too many arguments");
            return ExitCode::UsageError;
        }
    }
    ExitCode::Success
}