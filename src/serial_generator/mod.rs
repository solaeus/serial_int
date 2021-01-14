mod test;

use crate::Serial;

use std::{
    cell::Cell,
    fmt,
    fmt::{Display, Formatter},
};

#[derive(Clone, Debug)]
pub struct SerialGenerator<T: Serial = u32> {
    value: T,
}

impl<T: Serial> SerialGenerator<T> {
    pub fn new() -> Self {
        Self::default()
    }

    fn with_init_value(value: T) -> Self {
        SerialGenerator {
            value,
        }
    }

    fn previous(&self) -> Option<T> {
        if self.value == T::START {
            None
        } else if self.value == T::END {
            Some(self.value)
        } else {
            Some(self.value.prev_increment())
        }
    }

    fn generate(&mut self) -> T {
        let next = self.value.next_increment();
        self.value = next;

        next
    }

    fn remaining_increments(&self) -> T {
        self.value.remaining_increments()
    }
}

impl<T: Serial> Default for SerialGenerator<T> {
    fn default() -> Self {
        SerialGenerator {
            value: T::START
        }
    }
}

impl<T: fmt::Debug + Display + Serial> Display for SerialGenerator<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl<T: Serial> Iterator for SerialGenerator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let next_value = self.generate();

        Some(next_value)
    }
}
