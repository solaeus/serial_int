# serial_int

> !!! This crate is in early development and should not be used. Check back soon. !!!

Safe, immutable auto-increment integers

## Features

- [ ] Flexibility
  - [ ] Can use any integer type
  - [ ] Converts to any numeric type
- [ ] Safety
  - [ ] Uses immutable types by default
  - [ ] Panic-free in release mode
  - [ ] Uses only the standard library by default
- [ ] Usability
  - [ ] Uses sane defaults that mimic PostgreSQL's `SERIAL` data type
  - [ ] Feature flags
    - [ ] Serde implementations
    - [ ] Thread safety
    - [ ] A mutable generator for `no_std`
