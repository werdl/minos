//! list <path> - List the contents of a directory


use crate::sys::fs::{self, Resource, OpenFlag};
use crate::api::process::ExitCode;
use alloc::string::String;

pub fn list_files(path: &str) -> Result<(), ()> {
    let path = fs::canonicalize(path).unwrap();
    let resource = fs::open(path.as_str(), OpenFlag::Dir as usize).ok_or(())?;
    match resource {
        Resource::Dir(dir) => {
            for entry in dir.entries() {
                println!("{}", entry.name());
            }

            Ok(())
        }
        _ => println!("Not a directory"),
    }
}

pub fn main(cwd: String, args: &[&str]) -> ExitCode {
    match args.len() {
        0 => {
            // list the current directory
            list_files(cwd.as_str());
        }
        1 => {
            list_files(args[0]);
        }
        _ => {
            println!("Too many arguments");
            return ExitCode::UsageError;
        }
    }
    ExitCode::Success
}