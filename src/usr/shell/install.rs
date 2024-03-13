use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::string::ToString;

use crate::api::io::console::Style;
use crate::api::process::ExitCode;
use crate::sys::fs::DeviceType;
use crate::api::syscall;
use crate::sys::fs;
use crate::api::io;
use crate::sys;
use crate::api;

pub fn copy_files(verbose: bool) {
    create_dir("/bin", verbose); // Binaries
    create_dir("/dev", verbose); // Devices
    create_dir("/ini", verbose); // Initializers
    create_dir("/lib", verbose); // Libraries
    create_dir("/net", verbose); // Network
    create_dir("/src", verbose); // Sources
    create_dir("/tmp", verbose); // Temporaries
    create_dir("/usr", verbose); // User directories
    create_dir("/var", verbose); // Variables

    copy_file("/bin/clear", include_bytes!("../../../dsk/bin/clear"), verbose);
    //copy_file("/bin/exec", include_bytes!("../../../dsk/bin/exec"), verbose);
    copy_file("/bin/halt", include_bytes!("../../../dsk/bin/halt"), verbose);
    //copy_file("/bin/hello", include_bytes!("../../../dsk/bin/hello"), verbose);
    copy_file("/bin/ntp", include_bytes!("../../../dsk/bin/ntp"), verbose);
    copy_file("/bin/print", include_bytes!("../../../dsk/bin/print"), verbose);
    copy_file(
        "/bin/reboot",
        include_bytes!("../../../dsk/bin/reboot"),
        verbose,
    );
    copy_file("/bin/sleep", include_bytes!("../../../dsk/bin/sleep"), verbose);

    create_dir("/dev/ata", verbose); // Drives
    create_dir("/dev/ata/0", verbose);
    create_dev("/dev/ata/0/0", DeviceType::Drive, verbose);
    create_dev("/dev/ata/0/1", DeviceType::Drive, verbose);
    create_dir("/dev/ata/1", verbose);
    create_dev("/dev/ata/1/0", DeviceType::Drive, verbose);
    create_dev("/dev/ata/1/1", DeviceType::Drive, verbose);
    create_dir("/dev/clk", verbose); // Clock
    create_dev("/dev/clk/uptime", DeviceType::Uptime, verbose);
    create_dev("/dev/clk/realtime", DeviceType::Realtime, verbose);
    create_dev("/dev/rtc", DeviceType::RTC, verbose);
    create_dev("/dev/null", DeviceType::Null, verbose);
    create_dev("/dev/random", DeviceType::Random, verbose);
    create_dev("/dev/console", DeviceType::Console, verbose);
    create_dir("/dev/net", verbose); // Network
    create_dev("/dev/net/tcp", DeviceType::TcpSocket, verbose);
    create_dev("/dev/net/udp", DeviceType::UdpSocket, verbose);

    copy_file(
        "/ini/banner.txt",
        include_bytes!("../../../dsk/ini/banner.txt"),
        verbose,
    );
    copy_file(
        "/ini/boot.sh",
        include_bytes!("../../../dsk/ini/boot.sh"),
        verbose,
    );
    copy_file(
        "/ini/lisp.lsp",
        include_bytes!("../../../dsk/ini/lisp.lsp"),
        verbose,
    );
    copy_file(
        "/ini/shell.sh",
        include_bytes!("../../../dsk/ini/shell.sh"),
        verbose,
    );
    copy_file(
        "/ini/version.txt",
        include_bytes!("../../../dsk/ini/version.txt"),
        verbose,
    );

    create_dir("/ini/palettes", verbose);
    copy_file(
        "/ini/palettes/gruvbox-dark.sh",
        include_bytes!("../../../dsk/ini/palettes/gruvbox-dark.sh"),
        verbose,
    );
    copy_file(
        "/ini/palettes/gruvbox-light.sh",
        include_bytes!("../../../dsk/ini/palettes/gruvbox-light.sh"),
        verbose,
    );

    create_dir("/ini/fonts", verbose);
    /*
    copy_file(
        "/ini/fonts/lat15-terminus-8x16.psf",
        include_bytes!("../../../dsk/ini/fonts/lat15-terminus-8x16.psf"),
        verbose
    );
    */
    copy_file(
        "/ini/fonts/zap-light-8x16.psf",
        include_bytes!("../../../dsk/ini/fonts/zap-light-8x16.psf"),
        verbose,
    );
    copy_file(
        "/ini/fonts/zap-vga-8x16.psf",
        include_bytes!("../../../dsk/ini/fonts/zap-vga-8x16.psf"),
        verbose,
    );

    create_dir("/lib/lisp", verbose);
    copy_file(
        "/lib/lisp/alias.lsp",
        include_bytes!("../../../dsk/lib/lisp/alias.lsp"),
        verbose,
    );
    copy_file(
        "/lib/lisp/core.lsp",
        include_bytes!("../../../dsk/lib/lisp/core.lsp"),
        verbose,
    );
    copy_file(
        "/lib/lisp/file.lsp",
        include_bytes!("../../../dsk/lib/lisp/file.lsp"),
        verbose,
    );
    /*
    copy_file(
        "/lib/lisp/legacy.lsp",
        include_bytes!("../../../dsk/lib/lisp/legacy.lsp"),
        verbose
    );
    */

    copy_file(
        "/tmp/alice.txt",
        include_bytes!("../../../dsk/tmp/alice.txt"),
        verbose,
    );
    copy_file(
        "/tmp/machines.txt",
        include_bytes!("../../../dsk/tmp/machines.txt"),
        verbose,
    );

    create_dir("/tmp/lisp", verbose);
    copy_file(
        "/tmp/lisp/colors.lsp",
        include_bytes!("../../../dsk/tmp/lisp/colors.lsp"),
        verbose,
    );
    copy_file(
        "/tmp/lisp/doc.lsp",
        include_bytes!("../../../dsk/tmp/lisp/doc.lsp"),
        verbose,
    );
    copy_file(
        "/tmp/lisp/factorial.lsp",
        include_bytes!("../../../dsk/tmp/lisp/factorial.lsp"),
        verbose,
    );
    /*
    copy_file(
        "/tmp/lisp/fetch.lsp",
        include_bytes!("../../../dsk/tmp/lisp/fetch.lsp"),
        verbose
    );
    */
    copy_file(
        "/tmp/lisp/fibonacci.lsp",
        include_bytes!("../../../dsk/tmp/lisp/fibonacci.lsp"),
        verbose,
    );
    copy_file(
        "/tmp/lisp/geotime.lsp",
        include_bytes!("../../../dsk/tmp/lisp/geotime.lsp"),
        verbose,
    );
    copy_file(
        "/tmp/lisp/pi.lsp",
        include_bytes!("../../../dsk/tmp/lisp/pi.lsp"),
        verbose,
    );
    copy_file(
        "/tmp/lisp/sum.lsp",
        include_bytes!("../../../dsk/tmp/lisp/sum.lsp"),
        verbose,
    );

    create_dir("/tmp/life", verbose);
    copy_file(
        "/tmp/life/centinal.cells",
        include_bytes!("../../../dsk/tmp/life/centinal.cells"),
        verbose,
    );
    copy_file(
        "/tmp/life/flower-of-eden.cells",
        include_bytes!("../../../dsk/tmp/life/flower-of-eden.cells"),
        verbose,
    );
    copy_file(
        "/tmp/life/garden-of-eden.cells",
        include_bytes!("../../../dsk/tmp/life/garden-of-eden.cells"),
        verbose,
    );
    copy_file(
        "/tmp/life/glider-gun.cells",
        include_bytes!("../../../dsk/tmp/life/glider-gun.cells"),
        verbose,
    );
    copy_file(
        "/tmp/life/pentadecathlon.cells",
        include_bytes!("../../../dsk/tmp/life/pentadecathlon.cells"),
        verbose,
    );
    copy_file(
        "/tmp/life/queen-bee-shuttle.cells",
        include_bytes!("../../../dsk/tmp/life/queen-bee-shuttle.cells"),
        verbose,
    );
    copy_file(
        "/tmp/life/ship-in-a-bottle.cells",
        include_bytes!("../../../dsk/tmp/life/ship-in-a-bottle.cells"),
        verbose,
    );
    copy_file(
        "/tmp/life/thunderbird.cells",
        include_bytes!("../../../dsk/tmp/life/thunderbird.cells"),
        verbose,
    );
    copy_file(
        "/tmp/life/wing.cells",
        include_bytes!("../../../dsk/tmp/life/wing.cells"),
        verbose,
    );

    create_dir("/tmp/beep", verbose);
    copy_file(
        "/tmp/beep/tetris.sh",
        include_bytes!("../../../dsk/tmp/beep/tetris.sh"),
        verbose,
    );
    copy_file(
        "/tmp/beep/starwars.sh",
        include_bytes!("../../../dsk/tmp/beep/starwars.sh"),
        verbose,
    );
    copy_file(
        "/tmp/beep/mario.sh",
        include_bytes!("../../../dsk/tmp/beep/mario.sh"),
        verbose,
    );

    create_dir("/var/log", verbose);

    create_dir("/var/www", verbose);
    copy_file(
        "/var/www/index.html",
        include_bytes!("../../../dsk/var/www/index.html"),
        verbose,
    );

    // finally, create a marker file so we know the installation is complete
    api::io::fs::write("/var/installed", b"installation complete").ok();
}

fn parse_disk_path(pathname: &str) -> Result<(u8, u8), String> {
    let path: Vec<_> = pathname.split('/').collect();
    if !pathname.starts_with("/dev/ata/") || path.len() != 5 {
        return Err(format!("Could not find disk at '{}'", pathname));
    }
    let bus = path[3].parse().or(Err("Could not parse <bus>".to_string()))?;
    let dsk = path[4].parse().or(Err("Could not parse <dsk>".to_string()))?;
    Ok((bus, dsk))
}

fn format(pathname: &str) -> Result<(), ExitCode> {
    match parse_disk_path(pathname) {
        Ok((bus, dsk)) => {
            fs::mount_ata(bus, dsk);
            fs::format_ata();
            println!("Disk successfully formatted");
            println!("MFS is now mounted to '/'");
            Ok(())
        }
        Err(msg) => {
            error!("{}", msg);
            Err(ExitCode::Failure)
        }
    }
}

fn list() {
    println!("Path            Name (Size)");
    for drive in sys::device::disk::ata::list() {
        println!("/dev/ata/{}/{}    {}", drive.bus, drive.dsk, drive);
    }
}

pub fn main(args: &[&str]) -> ExitCode {
    let csi_color = Style::color("Yellow");
    let csi_reset = Style::reset();
    println!(
        "{}Welcome to minos v{} installation program!{}",
        csi_color,
        env!("CARGO_PKG_VERSION"),
        csi_reset
    );
    println!();

    let mut has_confirmed = false;
    for &arg in args {
        match arg {
            "-y" | "--yes" => has_confirmed = true,
            _ => continue,
        }
    }
    if !has_confirmed {
        print!("Proceed? [y/N] ");
        has_confirmed = io::stdin().read_line().trim() == "y";
        println!();
    }

    if has_confirmed {
        if !fs::is_mounted() {
            println!("{}Listing disks ...{}", csi_color, csi_reset);
            list();
            println!();

            println!("{}Formatting disk ...{}", csi_color, csi_reset);
            print!("Enter path of disk to format: ");
            let pathname = io::stdin().read_line();
            if format(&pathname.trim_end()).is_err() {
                return ExitCode::Failure;
            }
            println!();
        }

        println!("{}Populating filesystem...{}", csi_color, csi_reset);
        let verbose = true;
        copy_files(verbose);

        if sys::process::user().is_none() {
            println!();
            println!("{}Creating user...{}", csi_color, csi_reset);
            
            print!("Enter username: ");
            let username = io::stdin().read_line();

            let res = api::user::create(&username);

            if res == Err(ExitCode::Failure) {
                return res.unwrap_err();
            }
        }

        println!();
        println!("{}Installation successful!{}", csi_color, csi_reset);
        println!();
        super::main();
    }

    ExitCode::Success
}

fn create_dir(pathname: &str, verbose: bool) {
    if syscall::info(pathname).is_none() {
        if let Some(handle) = api::io::fs::create_dir(pathname) {
            syscall::close(handle);
            if verbose {
                println!("Created '{}'", pathname);
            }
        }
    }
}

fn create_dev(pathname: &str, dev: DeviceType, verbose: bool) {
    if syscall::info(pathname).is_none() {
        let mut buf = dev.buf();
        // NOTE: The first byte of `buf` contains the device type
        match pathname {
            "/dev/ata/0/0" => { buf[1] = 0; buf[2] = 0 },
            "/dev/ata/0/1" => { buf[1] = 0; buf[2] = 1 },
            "/dev/ata/1/0" => { buf[1] = 1; buf[2] = 0 },
            "/dev/ata/1/1" => { buf[1] = 1; buf[2] = 1 },
            _ => {},
        }
        if let Some(handle) = api::io::fs::create_device(pathname, &buf) {
            syscall::close(handle);
            if verbose {
                println!("Created '{}'", pathname);
            }
        }
    }
}

fn copy_file(pathname: &str, buf: &[u8], verbose: bool) {
    if api::io::fs::exists(pathname) {
        return;
    }
    if pathname.ends_with(".txt") {
        if let Ok(text) = String::from_utf8(buf.to_vec()) {
            let text = text.replace("{x.x.x}", env!("CARGO_PKG_VERSION"));
            api::io::fs::write(pathname, text.as_bytes()).ok();
        } else {
            api::io::fs::write(pathname, buf).ok();
        }
    } else {
        api::io::fs::write(pathname, buf).ok();
    }
    // TODO: add File::write_all to split buf if needed
    if verbose {
        println!("Copied '{}'", pathname);
    }
}