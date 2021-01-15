# serial_int

Safe, easy-to-use auto-increment integers

Auto-increment integers make great unique identifers because they do not need to
be large (i.e. using more memory) to prevent collisions. They are always unique
until they reach their max value, mimicking the behavior of PostgreSQL's
`SERIAL` data type.

## Features

- [ ] Usability
  - [X] Straightforward, documented API
  - [?] `no_std`
  - [ ] Serde support via feature flag
- [X] Safety
  - [X] Panic-free
  - [X] Uses only the standard library by default
  - [X] Full test coverage
- [X] Extensibility
  - [X] Trait-based
  - [X] Tests use trait generics, making it test new implementations
  - [X] Included implementations for all unsigned integers in std::core

## Usage

Use a generator to create unique identifiers.

```Rust
fn main()
```

## Contributing

Submit a patch. If you add a new implementation of Serial, add a submodule to
`tests` using the provided functions.
