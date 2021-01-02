use crate::Serial;

use std::{
    fmt,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub struct SerialGenerator<T = u32> {
    value: T,
}

impl<T: Default> SerialGenerator<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Serial> SerialGenerator<T> {
    fn with_value(value: T) -> Self {
        SerialGenerator { value }
    }

    fn prev(&self) -> T {
        self.value
    }

    fn generate(&mut self) -> T {
        self.value += T::INTERVAL;

        self.value
    }
}

impl<T: Default> Default for SerialGenerator<T> {
    fn default() -> Self {
        SerialGenerator { value: T::default() }
    }
}

impl<T: fmt::Debug + Display> Display for SerialGenerator<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<T: Serial> Iterator for SerialGenerator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.generate())
    }
}
