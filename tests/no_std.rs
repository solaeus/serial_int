/*
Without feature flags, this crate only uses the standard library. The only use
of `std` is to implement common traits such as Debug and Display. Therefore,
a no_std flag is not used in the library in the interest of simplicity. Instead
this test provides a quick way to be sure that no_std works after any changes
are made.
*/

#![no_std]

use serial_int::SerialGenerator;

#[test]
fn no_std() {
    let mut gen = SerialGenerator::<u8>::new();

    gen.generate();
}
