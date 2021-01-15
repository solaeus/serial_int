#![warn(missing_docs)]

//! This crate provides auto-increment integers that are guaranteed to produce
//! unique values.
//!
//! This is a simple implementation of a simple concept. This crate is
//! appropriately tiny.
//!
//! # Panics
//!
//! There are no possible scenarios for panicking in this type or in any of the
//! [Serial] implementations provide by this library.
//!
//! # Examples
//!
//! ```rust
//! # use serial_int::SerialGenerator;
//! let mut gen = SerialGenerator::<u32>::new();
//!
//! assert_eq!(0, gen.generate());
//! assert_eq!(1, gen.generate());
//! ```
//!
//! ```rust
//! # use serial_int::SerialGenerator;
//! # use lazy_static::lazy_static;
//! # use std::sync::Mutex;
//!
//! fn main() {
//!     let bob = User::new("bob@domain.xyz");
//!     let fred = User::new("fred@domain.xyz");
//!
//!     assert_eq!(0, bob.id);
//!     assert_eq!(1, fred.id);
//! }
//!
//! lazy_static! {
//!     static ref user_id_gen: Mutex<SerialGenerator>
//!         = Mutex::new(SerialGenerator::new());
//! }
//!
//! struct User {
//!     id: u32,
//!     email: String
//! }
//!
//! impl User {
//!     pub fn new(email: &str) -> Self {
//!         User {
//!             id: user_id_gen.lock().unwrap().generate(),
//!             email: email.to_string(),
//!         }
//!     }
//! }
//! ```

mod serial;
mod serial_generator;
mod tests;

pub use serial::Serial;
pub use serial_generator::SerialGenerator;
