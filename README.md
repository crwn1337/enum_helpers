# enum_helpers
[![github](https://img.shields.io/badge/enum__helpers-github?style=flat-square&logo=github&label=github&labelColor=232323&color=333333)](https://github.com/crwn1337/enum_helpers)
[![crates.io](https://img.shields.io/crates/v/enum_helpers?style=flat-square&logo=rust&labelColor=232323&color=333333)](https://crates.io/crates/enum_helpers)
[![docs.rs](https://img.shields.io/badge/enum__helpers-docs?style=flat-square&logo=docs.rs&label=docs.rs&labelColor=232323&color=333333)](https://docs.rs/enum_helpers/)


A Rust library to add helper functions to enums, with methods such as `is_`, `as_`, `as_mut_`, and `into_`, allowing you to simply your codebase and improve code readability.

## Example Usage
The following example demonstrates the usage of the `EnumIs` derive macro to utilize a simpler type checking for enums.

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

## Additional Resources
For more examples, refer to the [tests](tests/) directory.

To keep up with the latest features, improvements, and bug fixes, refer to the [Changelog](CHANGELOG.md) file.