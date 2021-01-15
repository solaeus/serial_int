# serial_int

> !!! This crate is in early development and should not be used. Check back soon. !!!

Safe, easy-to-use auto-increment integers

## Features

- [ ] Flexibility
  - [X] Extensible, trait-based implementation
  - [X] Included implementations for all unsigned integers
  - [ ] Included implementations for all atomic unsigned integers
- [ ] Safety
  - [X] Panic-free
  - [X] Uses only the standard library by default
  - [ ] Full test coverage
- [ ] Usability
  - [X] Uses sane defaults that mimic PostgreSQL's `SERIAL` data type
  - [X] `no_std`
  - [ ] Serde support via feature flage
