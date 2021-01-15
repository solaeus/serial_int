# serial_int

Safe, easy-to-use auto-increment integers

Auto-increment integers make great unique identifers because they do not need to
be large (i.e. using more memory) to prevent collisions. They are always unique
until they reach their max value, mimicking the behavior of PostgreSQL's
`SERIAL` data type.

## Features

- [_] Usability
  - [X] Straightforward, documented API
  - [?] `no_std` support
  - [_] Serde support via feature flag
- [X] Safety
  - [X] Panic-free
  - [X] No dependencies
  - [X] Full test coverage
- [X] Extensibility
  - [X] Support custom serial types with single trait
  - [X] Tests use trait generics, making it test new implementations
  - [X] Included implementations for all unsigned integers in std::core

## Usage

Use a generator to create unique identifiers.

```rust
use serial_int::SerialGenerator;
let mut gen = SerialGenerator::<u32>::new();

assert_eq!(0, gen.generate());
assert_eq!(1, gen.generate());
```

Using a wrapper to support concurrency or `static ref` for generators that don't
have an owner is simple. See the [docs](https://docs.rs/serial_int) for a more
complex example.

## Contributing

Submit a patch. If you add a new implementation of Serial, add a submodule to
`tests` using the provided functions.
