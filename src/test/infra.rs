#[macro_use]
use crate::{
    serial_println,
    serial_print,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

/// exit qemu with the given exit code. only to be used in test mode
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub trait Testable {
    fn run(self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(self) {
        self();
        serial_println!("[ok]");
    }
}

#[macro_export]
macro_rules! test_name {
    ($name:expr) => {
        serial_print!("{}...\t", $name);
    };

}


/// runs the given tests and exits qemu with the appropriate exit code
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}