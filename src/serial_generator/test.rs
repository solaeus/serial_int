#![cfg(test)]

use crate::{Serial, SerialGenerator};

#[test]
/// NOTE: This is a slow test.
fn creates_unique_values_before_end() {
    let gen = SerialGenerator::<u8>::new();
    let mut used = Vec::new();

    while gen.next().is_some() {
        let new_serial = gen.generate();

        assert!(!used.contains(&new_serial));

        used.push(new_serial);
    }
}

#[test]
/// NOTE: This is a slow test.
fn recreates_end_value() {
    let gen = SerialGenerator::<u8>::new();

    while gen.next().is_some() {
        gen.generate();
    }

    assert_eq!(u8::END, gen.previous().unwrap());
    assert_eq!(gen.previous().unwrap(), gen.generate());
}
