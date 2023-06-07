use std::{io::Bytes, str::FromStr};

use rust_prog::{refferal, add_refferal};
// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    let mut chars = [0i8; 20];

    let my_string = b"Anand Gedam";

    // Make sure we're not truncating.
    // A more sophisticated check could truncate on the last valid code point that will fit.
    assert!(my_string.len() < 20);

    // You could use ptr::copy_nonoverlapping() as well, but this should
    // optimize down to something like it anyway.
    // Slicing at 10 makes sure we're not overwriting the NUL terminator.
    for (left, right) in chars[..19].iter_mut().zip(my_string) {
        *left = *right as i8;
    }

    // Do FFI stuff here.
    let anand = refferal { name: chars, age: 30 , pincode: 411001, salary: 40000.0 };
    unsafe { add_refferal(anand) };
    println!("Hello, world!");
}
