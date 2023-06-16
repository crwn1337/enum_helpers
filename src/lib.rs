mod as_enum;
mod as_mut_enum;
mod is_enum;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemEnum};

#[proc_macro_derive(EnumIs)]
pub fn enum_is(input: TokenStream) -> TokenStream {
    is_enum::impl_enum_is(&parse_macro_input!(input as ItemEnum))
}

#[proc_macro_derive(EnumAs)]
pub fn enum_as(input: TokenStream) -> TokenStream {
    as_enum::impl_enum_as(&parse_macro_input!(input as ItemEnum))
}

#[proc_macro_derive(EnumAsMut)]
pub fn enum_as_mut(input: TokenStream) -> TokenStream {
    as_mut_enum::impl_enum_mut_as(&parse_macro_input!(input as ItemEnum))
}
