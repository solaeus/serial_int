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
    /// Create a new generator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new generator with the given value.
    ///
    /// [Serial::START] is always considered the first value. If this method is
    /// used with a greater value, [SerialGenerator::previous] may give an
    /// unexpected answer because the "previous" value is calculated, not
    /// recorded.
    fn with_init_value(value: T) -> Self {
        SerialGenerator { value }
    }

    /// Return the previously generated value.
    ///
    /// This method will return None if the current value is [Serial::START].
    fn previous(&self) -> Option<T> {
        if self.value == T::START {
            None
        } else {
            Some(self.value.prev_increment())
        }
    }

    fn generate(&mut self) -> T {
        let current = self.value;
        let next = current.next_increment();
        self.value = next;

        current
    }

    fn remaining_increments(&self) -> T {
        self.value.remaining_increments()
    }
}

impl<T: Serial> Default for SerialGenerator<T> {
    fn default() -> Self {
        SerialGenerator { value: T::START }
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
