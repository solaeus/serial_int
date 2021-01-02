pub mod serial_generator;

use std::ops::{Add, AddAssign};

pub trait Serial
where
    Self: Add + AddAssign + Copy,
{
    const INIT: Self;

    const INTERVAL: Self;
}

impl Serial for u8 {
    const INIT: Self = Self::MIN;

    const INTERVAL: Self = 1;
}

impl Serial for u16 {
    const INIT: Self = Self::MIN;

    const INTERVAL: Self = 1;
}

impl Serial for u32 {
    const INIT: Self = Self::MIN;

    const INTERVAL: Self = 1;
}

impl Serial for u64 {
    const INIT: Self = Self::MIN;

    const INTERVAL: Self = 1;
}

impl Serial for u128 {
    const INIT: Self = Self::MIN;

    const INTERVAL: Self = 1;
}

impl Serial for usize {
    const INIT: Self = Self::MIN;

    const INTERVAL: Self = 1;
}
