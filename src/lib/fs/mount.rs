use crate::{
    print, println
};

use alloc::string::String;

use serde_json::de::from_str;

#[derive(serde::Deserialize)]
struct File {
    name: String,
    contents: String
}

use alloc::vec::Vec;
pub fn include_bytes_test() {
    let x = include_bytes!("../../../disk.img");

    // parse the json into Value
    let string = core::str::from_utf8(x).unwrap();

    let json: Vec<File> = from_str(string).unwrap();
} 