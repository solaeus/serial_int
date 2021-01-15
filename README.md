# serial_int

Safe, easy-to-use auto-increment integers

[Docs](https://docs.rs/serial_int)

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
let mut gen = SerialGenerator::<u32>::new();

assert_eq!(0, gen.generate());
assert_eq!(1, gen.generate());
```

The generator is a simple type with minimal overhead.

```rust
let generator_size = std::mem::size_of::<SerialGenerator<u8>>();

assert_eq!(1, generator_size);
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

    assert_eq!(4, users.len());
}

lazy_static! {
    static ref user_id_gen: Arc<Mutex<SerialGenerator>>
        = Arc::new(Mutex::new(SerialGenerator::new()));
}

struct User {
    id: u32,
    email: String,
}

impl User {
    pub fn new(email: &str) -> Self {
        User {
            id: user_id_gen.lock().unwrap().generate(),
            email: email.to_string(),
        }
    }
}
```


## Contributing

Submit a patch. If you add a new implementation of Serial, add a submodule to
`tests` using the provided functions.
