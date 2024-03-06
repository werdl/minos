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
use x86_64::structures::paging::Page;
use x86_64::structures::paging::Translate;
use x86_64::VirtAddr;

entry_point!(kernel_main);

/// start the kernel
#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    lib::init();
    println!("Hello World{}", "!");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { init(phys_mem_offset) };
    let mut frame_allocator = EmptyFrameAllocator;

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};
    #[cfg(test)]
    test_main();

    lib::hlt_loop();
}
