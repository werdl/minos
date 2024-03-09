use alloc::boxed::Box;

use alloc::vec::Vec;

use super::test_name;
use crate::serial_print;

#[test_case]
fn simple_allocation() {
    test_name!("alloc::simple_allocation");
    let heap_value_1 = Box::new(41);
    let heap_value_2 = Box::new(13);
    assert_eq!(*heap_value_1, 41);
    assert_eq!(*heap_value_2, 13);
}

#[test_case]
fn large_vec() {
    test_name!("alloc::large_vec");
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

use crate::lib::memory::allocate::HEAP_SIZE;

#[test_case]
fn many_boxes() {
    test_name!("alloc::many_boxes");

    for i in 0..1000 {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}