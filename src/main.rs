#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![test_runner(crate::test::infra::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod test;

use core::panic;

use alloc::{
    vec, vec::Vec
};

use alloc::boxed::Box;
use alloc::rc::Rc;
use bootloader::entry_point;
use bootloader::BootInfo;
use test::{infra::*, tests::*};

mod lib;

use lib::device::io::serial::*;
use lib::device::io::vga::*;
use lib::interrupts::*;
use lib::panic::*;
use lib::memory::*;

extern crate alloc;

use x86_64::structures::paging::Page;
use x86_64::structures::paging::Translate;
use x86_64::VirtAddr;

entry_point!(kernel_main);

/// start the kernel
#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    lib::init(boot_info);
    println!("Hello World{}", "!"); 
    
    #[cfg(test)]
    test_main();

<<<<<<< HEAD
    lib::device::disk::ata::ata_test();
=======
    println!("It did not crash!");
>>>>>>> fad8a70e07bce29369e86ebbdc00f14f72e075e8

    lib::hlt_loop();
}
