# enum_helpers
a rust library that adds functions such as `is_`, `as_` and `as_mut_` to enums

simple example usage:
```rust
#[derive(EnumIs)]
pub enum Test {
    Example(u32),
}

pub fn main() {
    let test = Test::Example(123);
    assert!(test.is_example()); // true
}
```

more examples at [tests](tests/tests.rs)