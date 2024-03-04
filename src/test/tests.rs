#[macro_use]
use crate::{
    serial_println,
    serial_print,
    test_name
};

#[test_case]
fn test_suite() {
    test_name!("test_suite");

    assert!(true);
}

pub mod vga;
pub mod idt;