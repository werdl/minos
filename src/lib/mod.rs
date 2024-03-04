pub mod device;
pub mod panic;
pub mod interrupts;
pub mod gdt;

pub fn init() {
    interrupts::idt::init_idt();
    gdt::init();
    unsafe { interrupts::pic::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}
