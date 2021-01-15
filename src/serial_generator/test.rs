#![cfg(test)]

use crate::{serial, Serial, SerialGenerator};

#[test]
fn creates_unique_values_until_end() {
    let mut gen = SerialGenerator::with_init_value(u32::MAX - 10);
    let mut used = Vec::new();

    while gen.remaining_increments() > 0 {
        let serial = gen.generate();

        assert!(!used.contains(&serial));

        used.push(serial);
    }
}

#[test]
fn recreates_end_value() {
    let mut gen = SerialGenerator::with_init_value(u8::MAX - 1);

    assert_eq!(254, gen.generate());
    assert_eq!(255, gen.generate());
    assert_eq!(255, gen.generate());
    assert_eq!(255, gen.generate());
}
