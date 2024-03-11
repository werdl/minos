#[macro_export]
macro_rules! printk {
    ($($arg:tt)*) => ({
        $crate::sys::device::io::console::print_fmt(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ({
        let csi_color = $crate::api::console::Style::color("LightBlue");
        let csi_reset = $crate::api::console::Style::reset();
        $crate::sys::device::io::console::print_fmt(format_args!(
            "{}DEBUG: {}{}\n", csi_color, format_args!($($arg)*), csi_reset
        ));
    });
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => ({
        if !cfg!(test) {
            let uptime = $crate::sys::time::clock::uptime();
            let csi_color = $crate::api::console::Style::color("LightGreen");
            let csi_reset = $crate::api::console::Style::reset();
            $crate::sys::device::io::console::print_fmt(format_args!(
                "{}[{:.6}]{} {}\n",
                csi_color, uptime, csi_reset, format_args!($($arg)*)
            ));

            let realtime = $crate::sys::time::clock::realtime();
            $crate::sys::device::io::log::write_fmt(format_args!(
                "[{:.6}] {}\n",
                realtime, format_args!($($arg)*)
            ));
        }
    });
}

pub mod acpi;
pub mod allocator;
pub mod device;


pub mod fs;
pub mod gdt;
pub mod idt;
pub mod mem;
pub mod net;
pub mod pci;
pub mod pic;
pub mod process;
pub mod random;
pub mod syscall;
pub mod time;
