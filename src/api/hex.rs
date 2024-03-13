use alloc::{format, string::String};

pub fn print_hex(buf: &[u8]) {
    for b in buf {
        print!("{:02x}", b);
    }
    println!();
}

pub fn to_string(buf: &[u8]) -> String {
    let mut s = String::new();
    for b in buf {
        s.push_str(&format!("{:02x}", b));
    }
    s
}