//! list <path> - List the contents of a directory


use crate::sys::fs::{self, Resource, OpenFlag};
use crate::api::process::ExitCode;
use alloc::string::String;

pub fn list_files(path: &str) -> Result<(), ()> {
    let mut path = fs::canonicalize(path.trim_end_matches("/")).unwrap();

    if path == "" {
        path = String::from("/");
    }

    println!("Listing {}", path.as_str());

    let resource = fs::open(path.as_str(), OpenFlag::Dir as usize).ok_or(())?;


    match resource {
        Resource::Dir(dir) => {
            for entry in dir.entries() {
                // first, determine if entry is a directory, or a file
                let name = entry.name();
                let is_dir = entry.is_dir();

                // if dir, print in blue
                if is_dir {
                    println!("\x1b[34m{}\x1b[0m", name);
                } else {
                    println!("{}", name);
                }
            }
        }
        _ => println!("Not a directory"),
    };

    Ok(())
}

pub fn main(args: &[&str]) -> ExitCode {
    match args.len() {
        0 => {
            // list the current directory
            list_files(crate::sys::process::dir().as_str());
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