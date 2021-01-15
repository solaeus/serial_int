/// This crate provides auto-increment integers that are guaranteed to produce
/// unique values.
///
/// This is a simple implementation of a simple concept. This crate is
/// appropriately tiny.
mod serial;
mod serial_generator;
mod tests;

pub use serial::Serial;
pub use serial_generator::SerialGenerator;
