mod impl_as;
mod impl_into;
mod impl_is;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemEnum};

#[proc_macro_derive(EnumAs)]
pub fn enum_as(input: TokenStream) -> TokenStream {
    impl_as::impl_as(&parse_macro_input!(input as ItemEnum))
}

#[proc_macro_derive(EnumAsMut)]
pub fn enum_as_mut(input: TokenStream) -> TokenStream {
    impl_as::impl_as_mut(&parse_macro_input!(input as ItemEnum))
}

#[proc_macro_derive(EnumInto)]
pub fn enum_into(input: TokenStream) -> TokenStream {
    impl_into::impl_into(&parse_macro_input!(input as ItemEnum))
}

#[proc_macro_derive(EnumIs)]
pub fn enum_is(input: TokenStream) -> TokenStream {
    impl_is::impl_is(&parse_macro_input!(input as ItemEnum))
}
