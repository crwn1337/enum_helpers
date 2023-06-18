use convert_case::{Case, Casing};
use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{Fields, Ident, ItemEnum};

pub(crate) fn impl_enum_is(ast: &ItemEnum) -> TokenStream {
    let enum_name = &ast.ident;
    let generics = &ast.generics;
    let variants = &ast.variants;

    let enum_methods = variants.iter().map(|v| {
        let variant_ident = &v.ident;

        let function_name = Ident::new(
            &format!("is_{}", variant_ident).to_case(Case::Snake),
            Span::call_site().into(),
        );

        match &v.fields {
            Fields::Unit => Some(quote! {
                #[allow(dead_code)]
                #[must_use]
                pub fn #function_name(&self) -> bool {
                    matches!(self, Self::#variant_ident)
                }
            }),
            Fields::Unnamed(_) => Some(quote! {
                #[allow(dead_code)]
                #[must_use]
                pub fn #function_name(&self) -> bool {
                    matches!(self, Self::#variant_ident(..))
                }
            }),
            Fields::Named(_) => Some(quote! {
                #[allow(dead_code)]
                #[must_use]
                pub fn #function_name(&self) -> bool {
                    matches!(self, Self::#variant_ident{..})
                }
            }),
        }
    });

    quote! {
        impl #generics #enum_name #generics {
            #(#enum_methods)*
        }
    }
    .into()
}
