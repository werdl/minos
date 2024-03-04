#[macro_use]
use crate::{
    serial_println,
    serial_print,
    test_name,
    println
};

#[test_case]
fn test_println() {
    test_name!("vga::println     ");

    println!("test_println output");
}

#[test_case]
fn test_println_many() {
    test_name!("vga::println_many");

    for _ in 0..200 {
        println!("test_println_many output");
    }
}