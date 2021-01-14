mod test;

use crate::Serial;

use std::{
    cell::Cell,
    fmt,
    fmt::{Display, Formatter},
};

#[derive(Clone, Debug)]
pub struct SerialGenerator<T: Serial = u32> {
    value: Cell<T>,
}

impl<T: Serial> SerialGenerator<T> {
    pub fn new() -> Self {
        Self::default()
    }

    fn with_value(value: T) -> Self {
        SerialGenerator {
            value: Cell::new(value),
        }
    }

    fn previous(&self) -> Option<T> {
        let value = self.value.get();

        if value == T::START {
            None
        } else if value == T::END {
            Some(value)
        } else {
            Some(self.value.get().previous())
        }
    }

    fn next(&self) -> Option<T> {
        let value = self.value.get();

        if value == T::END {
            None
        } else {
            Some(self.value.get().previous())
        }
    }

    fn generate(&self) -> T {
        let next = self.value.get();

        self.value.set(next.increment());

        next
    }

    fn remaining_increments(&self) -> T {
        self.value.get().remaining_increments()
    }
}

impl<T: Serial> Default for SerialGenerator<T> {
    fn default() -> Self {
        SerialGenerator {
            value: Cell::new(T::START)
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
