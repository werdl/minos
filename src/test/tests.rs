#[macro_use]
use crate::{
    serial_println,
    serial_print,
    test_name
};

#[test_case]
fn trivial_assertion() {
    test_name!("trivial_assertion");

    assert_eq!(1, 1);
}

pub mod vga;