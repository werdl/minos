//! cat - read file and print on the standard output

use crate::api::io::fs::FileIO;
use crate::api::process::ExitCode;
use crate::sys::fs;
use crate::sys::fs::OpenFlag;
use crate::sys::fs::Resource;

use alloc::vec;

pub fn main(args: &[&str]) -> ExitCode {
    if args.len() == 0 {
        error!("Usage: cat <path>");
        return ExitCode::UsageError;
    }
        
    match args.len() {
        1 => {
            let path = fs::canonicalize(args[0]).expect("Failed to canoicalize path");

            let path = path.as_str();

            let resource = fs::open(path, OpenFlag::Read as usize);
            match resource {
                Some(Resource::File(mut file)) => {
                    let mut buffer = [0; 1024];
                    loop {
                        let len = file.read(&mut buffer).unwrap();
                        if len == 0 {
                            break;
                        }
                        println!("{}", core::str::from_utf8(&buffer[..len]).unwrap());
                    }
                }
                Some(res) => {
                    error!("Is a {}", res);
                    return ExitCode::ReadError;
                }
                None => {
                    // possibly a device, try to open it
                    let device = fs::Device::open(path);
                    match device {
                        Some(mut device) => {
                            // read from the device
                            let mut buffer = vec![0; 512];

                            device.read(&mut buffer).expect("Failed to read from device");

                            println!("{}", core::str::from_utf8(&buffer).unwrap_or(crate::api::hex::to_string(&buffer).as_str()));
                        }
                        None => {
                            error!("No such file or directory: {}", path);
                            return ExitCode::ReadError;
                        }
                    }
                }

            };
        }
        _ => {
            error!("Too many arguments");
            return ExitCode::UsageError;
        }
    }
    ExitCode::Success
}
