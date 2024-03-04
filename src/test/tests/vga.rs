#[macro_use]
use crate::{
    serial_println,
    serial_print,
    test_name,
    println
};

use crate::lib::device::io::vga::WRITER;
use crate::lib::device::io::vga::BUFFER_HEIGHT;

#[test_case]
fn test_println() {
    test_name!("vga::println");

    println!("test_println output");
}

#[test_case]
fn test_println_many() {
    test_name!("vga::println_many");

    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    test_name!("vga::println_output");

    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}
