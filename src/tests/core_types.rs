use crate::tests::{creates_unique_values_until_end, recreates_end_value};

/* u8 */

#[test]
fn u8_creates_unique_values_until_end() {
    creates_unique_values_until_end(u8::MAX - 5);
}

#[test]
fn u8_recreates_end_value() {
    recreates_end_value(u8::MAX - 1, u8::MAX);
}

/* u16 */

#[test]
fn u16_creates_unique_values_until_end() {
    creates_unique_values_until_end(u16::MAX - 5);
}

#[test]
fn u16_recreates_end_value() {
    recreates_end_value(u16::MAX - 1, u16::MAX);
}

/* u32 */

#[test]
fn u32_creates_unique_values_until_end() {
    creates_unique_values_until_end(u32::MAX - 5);
}

#[test]
fn u32_recreates_end_value() {
    recreates_end_value(u32::MAX - 1, u32::MAX);
}

/* u64 */

#[test]
fn u64_creates_unique_values_until_end() {
    creates_unique_values_until_end(u64::MAX - 5);
}

#[test]
fn u64_recreates_end_value() {
    recreates_end_value(u64::MAX - 1, u64::MAX);
}

/* u128 */

#[test]
fn u128_creates_unique_values_until_end() {
    creates_unique_values_until_end(u128::MAX - 5);
}

#[test]
fn u128_recreates_end_value() {
    recreates_end_value(u128::MAX - 1, u128::MAX);
}

/* usize */

#[test]
fn usize_creates_unique_values_until_end() {
    creates_unique_values_until_end(usize::MAX - 5);
}

#[test]
fn usize_recreates_end_value() {
    recreates_end_value(usize::MAX - 1, usize::MAX);
}
