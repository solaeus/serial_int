# serial_int

> !!! This crate is in early development and should not be used. Check back soon. !!!

Safe, easy-to-use auto-increment integers

## Features

- [ ] Flexibility
  - [X] Extensible, trait-based interface
  - [X] Included implementations for all unsigned integers
- [ ] Safety
  - [X] Immutability
  - [X] Panic-free
  - [X] Uses only the standard library by default
  - [ ] Full test coverage
- [ ] Usability
  - [X] Uses sane defaults that mimic PostgreSQL's `SERIAL` data type
  - [ ] Feature flags
    - [ ] Serde implementations
    - [ ] Thread safety
    - [ ] A mutable generator for `no_std`
