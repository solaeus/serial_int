use crate::Serial;

use std::{
    fmt,
    fmt::{Display, Formatter},
    mem::replace,
};

#[cfg(feature = "serde_impl")]
use serde::{Deserialize, Serialize};

/**
A generator that creates instances of a [Serial] type.

See the [crate] documentation for more information.
*/
#[cfg_attr(feature = "serde_impl", derive(Deserialize, Serialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SerialGenerator<T: Serial = u32> {
    value: T,
}

impl<T: Serial> SerialGenerator<T> {
    /**
    Creates a new generator.

    ```rust
    # use serial_int::SerialGenerator;
    let mut gen = SerialGenerator::new();

    assert_eq!(0_u32, gen.generate());
    ```
    */
    pub fn new() -> Self {
        Self::default()
    }

    /**
    Creates a new generator with the given value.

    [Serial::START] is always considered the first value. If this method is
    used with a greater value, [previous](Self::previous) may give an
    unexpected answer because the "previous" value is calculated, not
    recorded.

    ```rust
    # use serial_int::SerialGenerator;
    let mut gen = SerialGenerator::with_init_value(u8::MAX - 1);

    assert_eq!(254, gen.generate());
    ```
    */
    pub fn with_init_value(value: T) -> Self {
        SerialGenerator { value }
    }

    /**
    Generates a new instance of the generator's [Serial] type.

    ```rust
    # use serial_int::SerialGenerator;
    let mut gen = SerialGenerator::<u32>::new();

    assert_eq!(0, gen.generate());
    assert_eq!(1, gen.generate());
    ```
    */
    pub fn generate(&mut self) -> T {
        let next = self.value.next_increment();
        replace(&mut self.value, next)
    }

    /**
    Returns the previously generated value.

    This method will return None if the current value is [Serial::START].

    The return value is calculated, not recorded. If the highest possible
    value has been reached, this method will still return one less than that
    value. To check that unique values can still be generated, use
    [is_at_max](Self::is_at_max).

    ```rust
    # use serial_int::SerialGenerator;
    let mut gen = SerialGenerator::<u32>::new();

    assert_eq!(None, gen.previous());

    assert_eq!(0, gen.generate());
    assert_eq!(Some(0), gen.previous());

    assert_eq!(1, gen.generate());
    assert_eq!(Some(1), gen.previous());
    ```
    */
    pub fn previous(&self) -> Option<T> {
        if self.value == T::START {
            None
        } else {
            Some(self.value.prev_increment())
        }
    }

    /**
    Returns a boolean representing whether unique values can still be
    generated.
    ```rust
    # use serial_int::SerialGenerator;
    let mut gen = SerialGenerator::with_init_value(u8::MAX - 2);

    // Not at max, returns a value unique to this generator
    assert!(!gen.is_at_max());
    assert_eq!(253, gen.generate());

    // Not at max, returns a value unique to this generator
    assert!(!gen.is_at_max());
    assert_eq!(254, gen.generate());

    // At max, returns 255 forever
    assert!(gen.is_at_max());
    assert_eq!(255, gen.generate());
    ```
    */
    pub fn is_at_max(&self) -> bool {
        self.value.is_max_value()
    }
}

impl<T: Serial, U: Serial + From<T>> From<T> for SerialGenerator<U> {
    fn from(other: T) -> Self {
        SerialGenerator::with_init_value(other.into())
    }
}

impl<T: Serial> Default for SerialGenerator<T> {
    fn default() -> Self {
        SerialGenerator { value: T::START }
    }
}

impl<T: fmt::Debug + Display + Serial> Display for SerialGenerator<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<T: Serial> Iterator for SerialGenerator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_at_max() {
            None
        } else {
            let next_value = self.generate();

            Some(next_value)
        }
    }
}
