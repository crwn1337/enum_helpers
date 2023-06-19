mod impl_as;
mod impl_into;
mod impl_is;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemEnum};

/// Adds `as_` methods to enums.
/// These functions take self by reference and return an `Option<&T>` containing a reference to the value.
/// These functions aren't generated for unit types (empty types).
///
/// # Example
/// ```rust
/// use enum_helpers::EnumAs;
///
/// #[derive(EnumAs)]
/// pub enum Example {
///     A(u32),
/// }
///
/// fn example() {
///     assert_eq!(Example::A(1).as_a(), Some(&1));
/// }
#[proc_macro_derive(EnumAs)]
pub fn enum_as(input: TokenStream) -> TokenStream {
    impl_as::impl_as(&parse_macro_input!(input as ItemEnum))
}

/// Adds `as_mut_` methods to enums.
/// These functions take self by mutable reference and return an `Option<&mut T>` containing a reference to the value.
/// These functions aren't generated for unit types (empty types).
///
/// # Example
/// ```rust
/// use enum_helpers::EnumAsMut;
///
/// #[derive(EnumAsMut)]
/// pub enum Example {
///     A(u32),
/// }
///
/// fn example() {
///     assert_eq!(Example::A(1).as_mut_a(), Some(&mut 1));
/// }
#[proc_macro_derive(EnumAsMut)]
pub fn enum_as_mut(input: TokenStream) -> TokenStream {
    impl_as::impl_as_mut(&parse_macro_input!(input as ItemEnum))
}

/// Adds `into_` methods to enums,
/// These functions consume self and return an `Option<T>` containing the value.
/// These functions aren't generated for unit types (empty types).
///
/// # Example
/// ```rust
/// use enum_helpers::EnumInto;
///
/// #[derive(EnumInto)]
/// pub enum Example {
///     A(u32),
/// }
///
/// fn example() {
///     assert_eq!(Example::A(1).into_a(), Some(1));
/// }
#[proc_macro_derive(EnumInto)]
pub fn enum_into(input: TokenStream) -> TokenStream {
    impl_into::impl_into(&parse_macro_input!(input as ItemEnum))
}

/// Adds `is_` methods to enums.
/// These functions take self by reference and return a bool value whether the enum contains the value.
///
/// # Example
/// ```rust
/// use enum_helpers::EnumIs;
///
/// #[derive(EnumIs)]
/// pub enum Example {
///     A,
/// }
///
/// fn example() {
///     assert_eq!(Example::A.is_a(), true);
/// }
#[proc_macro_derive(EnumIs)]
pub fn enum_is(input: TokenStream) -> TokenStream {
    impl_is::impl_is(&parse_macro_input!(input as ItemEnum))
}
