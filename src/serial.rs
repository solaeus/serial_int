pub trait Serial
where
    Self: Clone + Ord,
{
    const START: Self;

    fn next_increment(&self) -> Self;

    fn prev_increment(&self) -> Self;

    fn remaining_increments(&self) -> Self;

    fn has_remaining_increments(&self) -> bool;
}

impl Serial for u8 {
    const START: Self = Self::MIN;

    fn next_increment(&self) -> Self {
        self.saturating_add(1)
    }

    fn prev_increment(&self) -> Self {
        self.saturating_sub(1)
    }

    fn remaining_increments(&self) -> Self {
        Self::MAX.saturating_sub(*self)
    }

    fn has_remaining_increments(&self) -> bool {
        self.remaining_increments() > 0
    }
}

impl Serial for u16 {
    const START: Self = Self::MIN;

    fn next_increment(&self) -> Self {
        self.saturating_add(1)
    }

    fn prev_increment(&self) -> Self {
        self.saturating_sub(1)
    }

    fn remaining_increments(&self) -> Self {
        Self::MAX.saturating_sub(*self)
    }

    fn has_remaining_increments(&self) -> bool {
        self.remaining_increments() > 0
    }
}

impl Serial for u32 {
    const START: Self = Self::MIN;

    fn next_increment(&self) -> Self {
        self.saturating_add(1)
    }

    fn prev_increment(&self) -> Self {
        self.saturating_sub(1)
    }

    fn remaining_increments(&self) -> Self {
        Self::MAX.saturating_sub(*self)
    }

    fn has_remaining_increments(&self) -> bool {
        self.remaining_increments() > 0
    }
}

impl Serial for u64 {
    const START: Self = Self::MIN;

    fn next_increment(&self) -> Self {
        self.saturating_add(1)
    }

    fn prev_increment(&self) -> Self {
        self.saturating_sub(1)
    }

    fn remaining_increments(&self) -> Self {
        Self::MAX.saturating_sub(*self)
    }

    fn has_remaining_increments(&self) -> bool {
        self.remaining_increments() > 0
    }
}

impl Serial for u128 {
    const START: Self = Self::MIN;

    fn next_increment(&self) -> Self {
        self.saturating_add(1)
    }

    fn prev_increment(&self) -> Self {
        self.saturating_sub(1)
    }

    fn remaining_increments(&self) -> Self {
        Self::MAX.saturating_sub(*self)
    }

    fn has_remaining_increments(&self) -> bool {
        self.remaining_increments() > 0
    }
}

impl Serial for usize {
    const START: Self = Self::MIN;

    fn next_increment(&self) -> Self {
        self.saturating_add(1)
    }

    fn prev_increment(&self) -> Self {
        self.saturating_sub(1)
    }

    fn remaining_increments(&self) -> Self {
        Self::MAX.saturating_sub(*self)
    }

    fn has_remaining_increments(&self) -> bool {
        self.remaining_increments() > 0
    }
}
