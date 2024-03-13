//! cat - read file and print on the standard output

use core::str::FromStr;

use alloc::vec::Vec;
use smoltcp::wire::IpAddress;

use crate::api::io::fs::FileIO;
use crate::api::process::ExitCode;
use crate::api::syscall;
use crate::sys::fs;
use crate::sys::fs::OpenFlag;
use crate::sys::fs::Resource;
use crate::sys::device::io::console;

use crate::sys::net::socket::tcp::TcpSocket;

use alloc::vec;

pub fn main(args: &[&str]) -> ExitCode {
    if args.len() == 0 {
        println!("Usage: cat <path>");
        return ExitCode::UsageError;
    }
        
    match args.len() {
        1 => {
            let path = args[0];
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
                            println!("No such file or directory: {}", path);
                            return ExitCode::ReadError;
                        }
                    }
                }
                e => {
                    println!("No such file or directory: {}", path);
                    return ExitCode::ReadError;
                },
            };
        }
        _ => {
            println!("Too many arguments");
            return ExitCode::UsageError;
        }
    }
    ExitCode::Success
}
