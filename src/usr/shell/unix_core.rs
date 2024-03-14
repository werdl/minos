//! unix_core.rs - core unix commands
//! List:
//! - cp
//! - rm
//! - mv
//! - mkdir
//! - rmdir
//! - touch

use crate::api::io::fs::FileIO;
use crate::api::process::ExitCode;
use crate::sys::fs;
use crate::sys::fs::OpenFlag;
use crate::sys::fs::Resource;

use alloc::vec;

pub fn cp(args: &[&str]) -> ExitCode {
    // PROBLEM: copies but then hangs
    if args.len() != 2 {
        error!("Usage: cp <source> <destination>");
        return ExitCode::UsageError;
    }

    // step 1 - open source file
    let source = fs::open(fs::canonicalize(args[0]).expect("Failed to canonicalize path").as_str(), OpenFlag::Read as usize);

    match source {
        Some(Resource::File(mut src)) => {
            // step 2 - open destination file
            let destination = fs::open(
                fs::canonicalize(args[1]).expect("Failed to canonicalize path").as_str(),
                OpenFlag::Write as usize + OpenFlag::Create as usize,
            );

            match destination {
                Some(Resource::File(mut dest)) => {
                    // step 3 - read from source and write to destination
                    let mut buffer = vec![0; 1];
                    loop {
                        let len = src.read(&mut buffer).expect("Failed to read from source");
                        if len == 0 {
                            break;
                        }
                        dest.write(&buffer).expect("Failed to write to destination");
                    }

                    ExitCode::Success
                }
                Some(res) => {
                    error!("Is a {}", res);
                    ExitCode::DataError
                }
                None => {
                    // possibly a device, try to open it
                    let device = fs::Device::open(args[1]);
                    match device {
                        Some(mut device) => {
                            // read from the device
                            let mut buffer = vec![0; 1];
                            loop {
                                let len = src.read(&mut buffer).expect("Failed to read from device");
                                if len == 0 {
                                    break;
                                }
                                device.write(&buffer).expect("Failed to write to device");
                            }

                            ExitCode::Success
                        }
                        None => {
                            // failed to open destination, which means we couldn't create it
                            error!("No such file or directory: {}", args[1]);
                            ExitCode::DataError
                        }
                    }
                }
            }
        }

        Some(Resource::Device(mut src)) => {
            // open destination file
            let destination = fs::open(args[1], OpenFlag::Write as usize);

            match destination {
                Some(Resource::File(mut dest)) => {
                    // read from source and write to destination
                    let mut buffer = vec![0; 512];

                    src.read(&mut buffer).expect("Failed to read from device");

                    dest.write(&buffer).expect("Failed to write to device");
                    ExitCode::Success
                }
                Some(res) => {
                    error!("Is a {}", res);
                    ExitCode::DataError
                }
                None => {
                    // possibly a device, try to open it
                    let device = fs::Device::open(args[1]);
                    match device {
                        Some(mut device) => {
                            // read from the device
                            let mut buffer = vec![0; 512];

                            src.read(&mut buffer).expect("Failed to read from device");

                            device.write(&buffer).expect("Failed to write to device");
                            ExitCode::Success
                        }
                        None => {
                            error!("No such file or directory: {}", args[1]);
                            ExitCode::DataError
                        }
                    }
                }
            }
        }

        Some(res) => {
            error!("Is a {}", res);
            ExitCode::DataError
        }

        None => {
            error!("No such file or directory: {}", args[0]);
            ExitCode::DataError
        }
    }
}

pub fn rm(args: &[&str]) -> ExitCode {
    if args.len() < 1 {
        error!("Usage: rm <paths ...>");
        return ExitCode::UsageError;
    }

    for path in args {
        let result = fs::delete(path);
        if result.is_err() {
            error!("No such file or directory: {}", path);
            return ExitCode::DataError;
        }
    }

    ExitCode::Success
}

pub fn mv(args: &[&str]) -> ExitCode {
    if args.len() != 2 {
        error!("Usage: mv <source> <destination>");
        return ExitCode::UsageError;
    }

    // step 1 - copy source to destination
    let cp_result = cp(args);

    if cp_result != ExitCode::Success {
        return cp_result;
    }

    // step 2 - remove source (and return the result)
    rm(&[args[0]])
    
}

pub fn mkdir(args: &[&str]) -> ExitCode {
    if args.len() != 1 {
        error!("Usage: mkdir <path>");
        return ExitCode::UsageError;
    }

    let result = crate::api::io::fs::create_dir(args[0]);
    if result.is_none() {
        error!("Failed to create directory: {}", args[0]);
        return ExitCode::DataError;
    }

    ExitCode::Success
}

pub fn rmdir(args: &[&str]) -> ExitCode {
    rm(args)
}

pub fn touch(args: &[&str]) -> ExitCode {
    if args.len() != 1 {
        error!("Usage: touch <path>");
        return ExitCode::UsageError;
    }

    // exists? error
    if fs::open(
        fs::canonicalize(args[0]).expect("Failed to canonicalize path").as_str(),
        OpenFlag::Read as usize,
    ).is_some() {
        error!("File already exists: {}", args[0]);
        return ExitCode::DataError;
    }

    let result = fs::open(
        fs::canonicalize(args[0]).expect("Failed to canonicalize path").as_str(),
        OpenFlag::Create as usize,
    );

    match result {
        Some(Resource::File(mut file)) => {
            // touch the file
            file.write(&[0]).expect("Failed to write to file");
            ExitCode::Success
        }
        Some(res) => {
            error!("Is a {}", res);
            ExitCode::DataError
        }
        None => {
            // possibly a device, try to open it
            let device = fs::Device::open(args[0]);
            match device {
                Some(mut device) => {
                    // touch the device
                    device.write(&[0]).expect("Failed to write to device");
                    ExitCode::Success
                }
                None => {
                    error!("No such file or directory: {}", args[0]);
                    ExitCode::DataError
                }
            }
        }
    }
    
}