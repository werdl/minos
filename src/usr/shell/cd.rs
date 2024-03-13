//! cd - change directory

use crate::api::process::ExitCode;
use alloc::string::String;
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
            let is_root = process::dir().as_str() == "/";

            if args[0].contains("..") {
                let path = process::dir();
                let mut path = path.split("/").collect::<Vec<&str>>();
                path.pop();
                let path = path.join("/");

                if path == "" {
                    process::set_dir("/");
                    return ExitCode::Success;
                }

                process::set_dir(path.as_str());
                return ExitCode::Success;
            }

            let new_path = format!("{}{}{}", process::dir(), if is_root { "" } else { "/" }, path);

            if crate::api::io::fs::exists(new_path.as_str()) {
                process::set_dir(new_path.as_str());
            } else {
                println!("Directory does not exist");
                return ExitCode::ExecError;
            }
        }
        _ => {
            println!("Too many arguments");
            return ExitCode::UsageError;
        }
    }
    ExitCode::Success
}