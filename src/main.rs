#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::infra::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod test;

use core::panic;

use test::{
    infra::*,
    tests::*
};

mod lib;

use lib::device::io::vga::*;
use lib::device::io::serial::*;
use lib::panic::*;

/// start the kernel
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}