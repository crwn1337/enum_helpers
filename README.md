# enum_helpers
A simple rust library that adds functions such as `is_`, `as_`, `as_mut_` and `into_` to enums.

## Example
```rust
use enum_helpers::EnumIs;

#[derive(EnumIs)]
pub enum Test {
    Example(u32),
}

pub fn main() {
    let test = Test::Example(123);
    assert!(test.is_example()); // true
}
```

[More examples](tests/).

[Changelog](CHANGELOG.md).