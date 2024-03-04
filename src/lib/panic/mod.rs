use core::panic::PanicInfo;
use crate::lib::device::io::vga::*;
use crate::lib::device::io::serial::*;

#[cfg(test)]
use crate::test::infra::*;

use crate::{
    serial_println,
    serial_print,
    println,
    print
};

/// our panic handler in non-test mode
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}