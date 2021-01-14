use crate::serial_generator;

use std::{fmt::Debug, ops::{Add, AddAssign, Sub}};

pub trait Serial
where
    Self: Add + AddAssign + Copy + Debug + Eq + Sub,
{
    const START: Self;

    const END: Self;

    const INTERVAL: Self;

    fn increment(&self) -> Self;

    fn previous(&self) -> Self;

    fn remaining_increments(&self) -> Self;
}

impl Serial for u8 {
    const START: Self = Self::MIN;

    const END: Self = Self::MAX;

    const INTERVAL: Self = 1;

    fn increment(&self) -> Self {
        self.saturating_add(Self::INTERVAL)
    }

    fn previous(&self) -> Self {
        self.saturating_sub(Self::INTERVAL)
    }

    fn remaining_increments(&self) -> Self {
        Self::MAX.saturating_sub(*self)
    }
}

impl Serial for u16 {
    const START: Self = Self::MIN;

    const END: Self = Self::MAX;

    const INTERVAL: Self = 1;

    fn increment(&self) -> Self {
        self.saturating_add(Self::INTERVAL)
    }

    fn previous(&self) -> Self {
        self.saturating_sub(Self::INTERVAL)
    }

    fn remaining_increments(&self) -> Self {
        Self::MAX.saturating_sub(*self)
    }
}

impl Serial for u32 {
    const START: Self = Self::MIN;

    const END: Self = Self::MAX;

    const INTERVAL: Self = 1;

    fn increment(&self) -> Self {
        self.saturating_add(Self::INTERVAL)
    }

    fn previous(&self) -> Self {
        self.saturating_sub(Self::INTERVAL)
    }

    fn remaining_increments(&self) -> Self {
        Self::MAX.saturating_sub(*self)
    }
}

impl Serial for u64 {
    const START: Self = Self::MIN;

    const END: Self = Self::MAX;

    const INTERVAL: Self = 1;

    fn increment(&self) -> Self {
        self.saturating_add(Self::INTERVAL)
    }

    fn previous(&self) -> Self {
        self.saturating_sub(Self::INTERVAL)
    }

    fn remaining_increments(&self) -> Self {
        Self::MAX.saturating_sub(*self)
    }
}

impl Serial for u128 {
    const START: Self = Self::MIN;

    const END: Self = Self::MAX;

    const INTERVAL: Self = 1;

    fn increment(&self) -> Self {
        self.saturating_add(Self::INTERVAL)
    }

    fn previous(&self) -> Self {
        self.saturating_sub(Self::INTERVAL)
    }

    fn remaining_increments(&self) -> Self {
        Self::MAX.saturating_sub(*self)
    }
}

impl Serial for usize {
    const START: Self = Self::MIN;

    const END: Self = Self::MAX;

    const INTERVAL: Self = 1;

    fn increment(&self) -> Self {
        self.saturating_add(Self::INTERVAL)
    }

    fn previous(&self) -> Self {
        self.saturating_sub(Self::INTERVAL)
    }

    fn remaining_increments(&self) -> Self {
        Self::MAX.saturating_sub(*self)
    }
}
