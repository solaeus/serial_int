# serial_int

Safe, easy-to-use auto-increment integers

- [Docs](https://docs.rs/serial_int)
- [Repository](https://sr.ht/~jeffa/serial_int)
- [Issues](https://todo.sr.ht/~jeffa/serial_int)

Serial (or auto-increment) integers make great unique identifers because they do
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
  - [X] `no_std`
  - [_] Serde support via feature flag
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

To support concurrency, simply use a wrapper. You can also use `static ref` for
generators that don't have an owner.

```rust
fn main() {
    let users_mutex = Arc::new(Mutex::new(Vec::new()));
    let users_clone = Arc::clone(&users_mutex);

    let handle = thread::spawn(move || {
        let alice = User::new("alice@domain.xyz");
        let mary = User::new("mary@domain.xyz");
        let mut users = users_clone.lock().unwrap();

        users.push(alice);
        users.push(mary);
    });

    handle.join().unwrap();

    let bob = User::new("bob@domain.xyz");
    let fred = User::new("fred@domain.xyz");
    let mut users = users_mutex.lock().unwrap();

    users.push(bob);
    users.push(fred);

    assert_eq!(0, users[0].id);
    assert_eq!(1, users[1].id);
    assert_eq!(2, users[2].id);
    assert_eq!(3, users[3].id);
}

lazy_static! {
    static ref USER_ID_GEN: Mutex<SerialGenerator>
        = Mutex::new(SerialGenerator::new());
}

struct User {
    id: u32,
    email: String,
}

impl User {
    pub fn new(email: &str) -> Self {
        User {
            id: USER_ID_GEN.lock().unwrap().generate(),
            email: email.to_string(),
        }
    }
}
```


## Contributing

Submit a patch. If you add a new implementation of Serial, add a submodule to
`tests` using the provided functions.
