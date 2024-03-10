use x86_64::{
    structures::paging::Page,
    VirtAddr,
};

use crate::BootInfoFrameAllocator;

pub mod device;
pub mod panic;
pub mod interrupts;
pub mod gdt;
pub mod memory;
pub mod fs;


pub fn init(boot_info: &'static bootloader::BootInfo) {
    interrupts::idt::init_idt();
    gdt::init();
    unsafe { interrupts::pic::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // new
    memory::allocate::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}