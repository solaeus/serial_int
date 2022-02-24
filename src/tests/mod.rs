#![cfg(test)]

mod core_types;

use crate::{Serial, SerialGenerator};

pub fn creates_unique_values_until_end<T: Serial + Ord>(start: T) {
    let mut gen = SerialGenerator::with_init_value(start);
    let mut used = Vec::new();

    while !gen.is_at_max() {
        let serial = gen.generate();

        assert!(!used.contains(&serial));

        used.push(serial);
    }

    assert!(!used.is_empty(), "Test did not check for anything...")
}

pub fn recreates_end_value<T: Serial + std::fmt::Debug>(init: T, end: T) {
    let mut gen = SerialGenerator::with_init_value(init.clone());

    assert_eq!(init, gen.generate());
    assert_eq!(end, gen.generate());
    assert_eq!(end, gen.generate());
    assert_eq!(end, gen.generate());
}
