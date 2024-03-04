use crate::{
    serial_print,
    test_name
};

#[test_case]
fn test_breakpoint_exception() {
    test_name!("idt::breakpoint_exception");

    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}
