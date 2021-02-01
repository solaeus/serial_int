//! Serial (or auto-increment) integers make great unique identifers. They do
//! not need to be large (i.e. using more memory) to prevent collisions and they
//! are always unique until they reach their max value, mimicking the behavior
//! of PostgreSQL's `SERIAL` data type. Creating serial values has minimal
//! performance impact because it relies on simple adding rather than hashing or
//! randomizing.
//!
//! This crate provides a generator (that is also an iterator) that outputs
//! serial values. By default, any unsigned integer from the standard library
//! can be generated. This is essentially a counter, a simple iterator for
//! integers. This crate is appropriately tiny.
//!
//! For safety and stability, the generator "saturates" the values instead of
//! overflowing. This guarantees that the output values are unique to that
//! generator (except for the greatest possible value, e.g. u8::MAX or
//! u32::MAX).
//!
//! # Panics
//!
//! None
//!
//! # Examples
//!
//! A simple example.
//!
//! ```rust
//! # use serial_int::SerialGenerator;
//! #
//! let mut gen = SerialGenerator::<u32>::new();
//!
//! assert_eq!(0, gen.generate());
//! assert_eq!(1, gen.generate());
//! ```
//!
//! ## Using the "serde_impl" feature
//!
//! Serialize a generator with Serde.
//!
//! ```rust
//! # use serial_int::SerialGenerator;
//! # use toml;
//! #
//! let mut gen = SerialGenerator::<u8>::new();
//!
//! gen.generate();
//! gen.generate();
//!
//! let gen_string = toml::to_string(&gen).unwrap();
//!
//! assert_eq!("value = 2\n", gen_string);
//! ```
//!
//! Deserialize a generator with Serde.
//!
//! ```rust
//! # use serial_int::SerialGenerator;
//! # use toml;
//! #
//! let gen = SerialGenerator::<u8>::new();
//! let gen_from_toml =
//!     toml::from_str::<SerialGenerator<u8>>("value = 0\n").unwrap();
//!
//! assert_eq!(gen_from_toml, gen);
//! ```
//!
//! ## Using `no_std`
//!
//! No feature flags are needed.
//!
//! ```rust
//! #![no_std]
//! # use serial_int::SerialGenerator;
//!
//! fn main() {
//!     let mut gen = SerialGenerator::<u8>::new();
//!     let serial_ids = [gen.generate(), gen.generate()];
//!
//!     assert_eq!([0, 1], serial_ids);
//! }
//! ```
//!
//! ## Other
//!
//! A complex example showing the use of `static` and concurrency
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
//!     assert_eq!(0, users[0].id);
//!     assert_eq!(1, users[1].id);
//!     assert_eq!(2, users[2].id);
//!     assert_eq!(3, users[3].id);
//! }
//!
//! lazy_static! {
//!     static ref USER_ID_GEN: Mutex<SerialGenerator>
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
//!             id: USER_ID_GEN.lock().unwrap().generate(),
//!             email: email.to_string(),
//!         }
//!     }
//! }
//! ```

#![warn(missing_docs)]

mod serial;
mod serial_generator;
mod tests;

pub use serial::Serial;
pub use serial_generator::SerialGenerator;
