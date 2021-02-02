# serial_int

Safe, easy-to-use auto-increment integers

- [Docs](https://docs.rs/serial_int)
- [Repository](https://sr.ht/~jeffa/serial_int)
- [Issues](https://todo.sr.ht/~jeffa/serial_int)
- [GitHub mirror](https://github.com/JKAnderson409/serial_int)

Serial (or auto-increment) integers make great unique identifiers because they do
not need to be large (i.e. using more memory) to prevent collisions.  They are
always unique until they reach their max value, mimicking the behavior of
PostgreSQL's `SERIAL` data type. Creating serial values has minimal performance
impact because it relies on simple adding rather than hashing or randomizing.

This crate provides a generator (that is also an iterator) that outputs serial
values. By default, any unsigned integer from the standard library can be
generated. This is essentially a counter, a simple iterator for integers. This
crate is appropriately tiny.

For safety and stability, the generator "saturates" the values instead of
overflowing. This guarantees that the output values are unique to that generator
(except for the greatest possible value, e.g. u8::MAX or u32::MAX).

## Features

- Usability
  - [X] Straightforward, documented API
  - [X] Includes support for all unsigned integers in the standard library
  - [X] Iterator implementation
  - [X] Serde support via feature flag
  - [X] `no_std`
- Safety
  - [X] Panic-free
  - [X] No dependencies
  - [X] Full test coverage
- Extensibility
  - [X] Support custom serial types with single trait
  - [X] Tests use trait generics, making it easy to test new implementations

## Usage

Use a generator to create unique identifiers.

```rust
let mut gen = SerialGenerator::<u32>::new();

assert_eq!(0, gen.generate());
assert_eq!(1, gen.generate());
```

More examples are available in the [docs](https://docs.rs/serial_int/#examples).

## Contributing

### How to contribute

Submit a patch. If you add a new implementation of Serial, add a submodule to
`tests` using the provided functions.

This project is maintained on Sourcehut. Contributions through this platform are preferred. You may submit a [ticket](https://todo.sr.ht/~jeffa/serial_int) if you encounter a problem or want to request a feature. You may use the [mailing list](https://lists.sr.ht/~jeffa/serial_int) to contribute patches or public discussion.

Contributions are also welcome on the Github mirror.

### 🦀🔧 Contributors 🔧🦀

- Jeff Anderson 🐙 (creator/maintainer) - [Sourcehut](https://sr.ht/~jeffa/) | [Github](https://github.com/JKAnderson409) | https://jeffa.io
- Yash Karandikar 🧞 (community contributor) - [Sourcehut](https://lists.sr.ht/~karx) | [Github](https://github.com/karx1) | https://karx.xyz/
