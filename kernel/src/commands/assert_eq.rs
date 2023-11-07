use crate::println;
use alloc::vec::Vec;

pub fn execute(args: &str) {
    let values: Vec<&str> = args.split_whitespace().collect();
    let length = values.len();
    if length != 2 {
        println!("assert_eq takes 2 arguments but {length} arguments was supplied");
        return;
    }

    let val1 = values[0];
    let val2 = values[1];

    let val1: i32 = val1.parse().expect("failed to parse to i32");
    let val2: i32 = val2.parse().expect("failed to parse to i32");

    assert_eq(val1, val2);    
}


fn assert_eq(left: i32, right: i32) -> bool {
    if left == right {
        return true
    } 
    panic!("assertion left == right failed: \nleft: {left} \nright: {right}");
}