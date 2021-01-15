#![warn(missing_docs)]

//! This crate provides an auto-increment generator that is guaranteed to
//! produce unique values.
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
//! # use std::{sync::{Arc, Mutex}, thread};
//! #
//! fn main() {
//!     let users_mutex = Arc::new(Mutex::new(Vec::new()));
//!     let users_clone = Arc::clone(&users_mutex);
//!
//!     let handle = thread::spawn(move || {
//!         let alice = User::new("alice@domain.xyz");
//!         let mary = User::new("mary@domain.xyz");
//!         let mut users = users_clone.lock().unwrap();
//!
//!         users.push(alice);
//!         users.push(mary);
//!     });
//!
//!     handle.join().unwrap();
//!
//!     let bob = User::new("bob@domain.xyz");
//!     let fred = User::new("fred@domain.xyz");
//!     let mut users = users_mutex.lock().unwrap();
//!
//!     users.push(bob);
//!     users.push(fred);
//!
//!     assert_eq!(4, users.len());
//! }
//!
//! lazy_static! {
//!     static ref user_id_gen: Mutex<SerialGenerator>
//!         = Mutex::new(SerialGenerator::new());
//! }
//!
//! struct User {
//!     id: u32,
//!     email: String,
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
//!
//! ```
//! # use serial_int::SerialGenerator;
//! let generator_size = std::mem::size_of::<SerialGenerator<u8>>();
//!
//! assert_eq!(1, generator_size);
//! ```

mod serial;
mod serial_generator;
mod tests;

pub use serial::Serial;
pub use serial_generator::SerialGenerator;
