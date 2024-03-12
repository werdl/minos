use alloc::vec::Vec;

pub fn encode(s: &[u8]) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    buf.resize(s.len() * 4 / 3 + 4, 0); // Resize to base64 + padding
    let bytes_written = base64::encode_config_slice(
        s, base64::STANDARD_NO_PAD, &mut buf
    );
    buf.resize(bytes_written, 0); // Resize back to actual size
    buf
}

pub fn decode(s: &[u8]) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    buf.resize(s.len(), 0);
    let bytes_written = base64::decode_config_slice(
        s, base64::STANDARD_NO_PAD, &mut buf
    ).unwrap();
    buf.resize(bytes_written, 0);
    buf
}