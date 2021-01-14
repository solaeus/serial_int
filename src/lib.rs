/// This crate provides auto-increment integers that are guaranteed to produce
/// unique values.
///
/// This is a simple implementation of a simple concept. This crate is
/// appropriately tiny. Any extraneous features are behind a feature flag.

mod serial;
mod serial_generator;

pub use serial::Serial;
pub use serial_generator::SerialGenerator;
