#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test::infra::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod test;

use core::panic;

use bootloader::entry_point;
use bootloader::BootInfo;
use test::{infra::*, tests::*};

mod lib;

use lib::device::io::serial::*;
use lib::device::io::vga::*;
use lib::interrupts::*;
use lib::panic::*;
use lib::memory::*;
use x86_64::VirtAddr;

entry_point!(kernel_main);

/// start the kernel
#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    lib::init();
    println!("Hello World{}", "!");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }

    #[cfg(test)]
    test_main();

    lib::hlt_loop();
}
