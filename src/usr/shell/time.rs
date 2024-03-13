//! time.rs - fetch dates and times
//! usage: time [OPTION]
//! options: rtc, uptime, realtime

use crate::api::process::ExitCode;
use crate::sys::time::cmos::CMOS;

pub fn main(args: &[&str]) -> ExitCode {
    if args.len() == 0 {
        error!("Usage: time [OPTION]");
        return ExitCode::UsageError;
    }

    match args[0] {
        "rtc" => {
            let mut cmos = CMOS::new();
            let datetime = cmos.rtc();
            println!("{}-{:02}-{:02} {:02}:{:02}:{:02}", datetime.year, datetime.month, datetime.day, datetime.hour, datetime.minute, datetime.second);
        },
        "up" => {
            let raw_uptime = crate::sys::time::clock::uptime();

            let days = (raw_uptime / 86400.0) as u64;
            let hours = ((raw_uptime / 3600.0) % 24.0) as u64;
            let minutes = ((raw_uptime / 60.0) % 60.0) as u64;
            let seconds = raw_uptime % 60.0;

            println!("{}d, {}h, {}m, {:.3}s ({:.3}s)", days, hours, minutes, seconds, raw_uptime);
        },
        "unix" => {
            println!("{}", crate::sys::time::clock::realtime());
        },
        _ => {
            error!("Unknown option: {}", args[0]);
            return ExitCode::UsageError;
        }
    }

    ExitCode::Success
}