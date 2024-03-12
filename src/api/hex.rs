pub fn print_hex(buf: &[u8]) {
    for b in buf {
        print!("{:02x}", b);
    }
    println!();
}