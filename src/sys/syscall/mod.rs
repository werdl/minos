pub mod number;
pub mod service;

use crate::api::process::ExitCode;
use crate::sys;
use crate::sys::fs::FileInfo;

use core::arch::asm;
use smoltcp::wire::IpAddress;
use smoltcp::wire::Ipv4Address;

fn utf8_from_raw_parts(ptr: *mut u8, len: usize) -> &'static str {
    unsafe {
        let slice = core::slice::from_raw_parts(ptr, len);
        core::str::from_utf8_unchecked(slice)
    }
}

pub fn dispatcher(
    n: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize
) -> usize {
    match n {
        number::EXIT => service::exit(ExitCode::from(arg1)) as usize,
        number::SLEEP => {
            service::sleep(f64::from_bits(arg1 as u64));
            0
        }
        number::DELETE => {
            let ptr = sys::process::ptr_from_addr(arg1 as u64);
            let len = arg2;
            let path = utf8_from_raw_parts(ptr, len);
            service::delete(path) as usize
        }
        number::INFO => {
            let ptr = sys::process::ptr_from_addr(arg1 as u64);
            let len = arg2;
            let path = utf8_from_raw_parts(ptr, len);
            let info = unsafe { &mut *(arg3 as *mut FileInfo) };
            service::info(path, info) as usize
        }
        number::OPEN => {
            let ptr = sys::process::ptr_from_addr(arg1 as u64);
            let len = arg2;
            let path = utf8_from_raw_parts(ptr, len);
            let flags = arg3;
            service::open(path, flags) as usize
        }
        number::READ => {
            let handle = arg1;
            let ptr = sys::process::ptr_from_addr(arg2 as u64);
            let len = arg3;
            let buf = unsafe {
                core::slice::from_raw_parts_mut(ptr, len)
            };
            service::read(handle, buf) as usize
        }
        number::WRITE => {
            let handle = arg1;
            let ptr = sys::process::ptr_from_addr(arg2 as u64);
            let len = arg3;
            let buf = unsafe {
                core::slice::from_raw_parts_mut(ptr, len) // TODO: Remove mut
            };
            service::write(handle, buf) as usize
        }
        number::CLOSE => {
            let handle = arg1;
            service::close(handle);
            0
        }
        number::DUP => {
            let old_handle = arg1;
            let new_handle = arg2;
            service::dup(old_handle, new_handle) as usize
        }
        number::SPAWN => {
            let path_ptr = sys::process::ptr_from_addr(arg1 as u64);
            let path_len = arg2;
            let path = utf8_from_raw_parts(path_ptr, path_len);
            let args_ptr = arg3;
            let args_len = arg4;
            service::spawn(path, args_ptr, args_len) as usize
        }
        number::STOP => {
            let code = arg1;
            service::stop(code)
        }
        number::POLL => {
            let ptr = sys::process::ptr_from_addr(arg1 as u64) as *const _;
            let len = arg2;
            let list = unsafe { core::slice::from_raw_parts(ptr, len) };
            service::poll(list) as usize
        }
        number::CONNECT => {
            let handle = arg1;
            let ptr = sys::process::ptr_from_addr(arg2 as u64);
            let len = arg3;
            let buf = unsafe { core::slice::from_raw_parts(ptr, len) };
            let addr = IpAddress::from(Ipv4Address::from_bytes(buf));
            let port = arg4 as u16;
            service::connect(handle, addr, port) as usize
        }
        number::LISTEN => {
            let handle = arg1;
            let port = arg2 as u16;
            service::listen(handle, port) as usize
        }
        number::ACCEPT => {
            let handle = arg1;
            let ptr = sys::process::ptr_from_addr(arg2 as u64);
            let len = arg3;
            let buf = unsafe { core::slice::from_raw_parts_mut(ptr, len) };
            if let Ok(addr) = service::accept(handle) {
                buf[0..len].clone_from_slice(addr.as_bytes());
                0
            } else {
                -1 as isize as usize
            }
        }
        number::ALLOC => {
            let size = arg1;
            let align = arg2;
            service::alloc(size, align) as usize
        }
        number::FREE => {
            let ptr = arg1 as *mut u8;
            let size = arg2;
            let align = arg3;
            service::free(ptr, size, align);
            0
        }
        _ => {
            unimplemented!();
        }
    }
}

#[doc(hidden)]
pub unsafe fn syscall0(n: usize) -> usize {
    let res: usize;
    asm!(
        "int 0x80", in("rax") n,
        lateout("rax") res
    );
    res
}

#[doc(hidden)]
pub unsafe fn syscall1(n: usize, arg1: usize) -> usize {
    let res: usize;
    asm!(
        "int 0x80", in("rax") n,
        in("rdi") arg1,
        lateout("rax") res
    );
    res
}

#[doc(hidden)]
pub unsafe fn syscall2(n: usize, arg1: usize, arg2: usize) -> usize {
    let res: usize;
    asm!(
        "int 0x80", in("rax") n,
        in("rdi") arg1, in("rsi") arg2,
        lateout("rax") res
    );
    res
}

#[doc(hidden)]
pub unsafe fn syscall3(
    n: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize
) -> usize {
    let res: usize;
    asm!(
        "int 0x80", in("rax") n,
        in("rdi") arg1, in("rsi") arg2, in("rdx") arg3,
        lateout("rax") res
    );
    res
}

#[doc(hidden)]
pub unsafe fn syscall4(
    n: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize
) -> usize {
    let res: usize;
    asm!(
        "int 0x80", in("rax") n,
        in("rdi") arg1, in("rsi") arg2, in("rdx") arg3, in("r8") arg4,
        lateout("rax") res
    );
    res
}

#[macro_export]
macro_rules! syscall {
    ($n:expr) => {
        $crate::sys::syscall::syscall0($n as usize)
    };
    ($n:expr, $a1:expr) => {
        $crate::sys::syscall::syscall1($n as usize, $a1 as usize)
    };
    ($n:expr, $a1:expr, $a2:expr) => {
        $crate::sys::syscall::syscall2($n as usize, $a1 as usize, $a2 as usize)
    };
    ($n:expr, $a1:expr, $a2:expr, $a3:expr) => {
        $crate::sys::syscall::syscall3(
            $n as usize,
            $a1 as usize,
            $a2 as usize,
            $a3 as usize,
        )
    };
    ($n:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
        $crate::sys::syscall::syscall4(
            $n as usize,
            $a1 as usize,
            $a2 as usize,
            $a3 as usize,
            $a4 as usize,
        )
    };
}
