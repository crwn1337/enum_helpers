use convert_case::{Case, Casing};
use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{Fields, Ident, ItemEnum};

pub(crate) fn impl_as(ast: &ItemEnum) -> TokenStream {
    let enum_name = &ast.ident;
    let generics = &ast.generics;
    let variants = &ast.variants;

    let enum_methods = variants.iter().map(|v| {
        let variant_ident = &v.ident;

        let field_types = v
            .fields
            .iter()
            .enumerate()
            .map(|(_, f)| &f.ty)
            .collect::<Vec<_>>();

        let function_name = Ident::new(
            &format!("as_{}", variant_ident).to_case(Case::Snake),
            Span::call_site().into(),
        );

        match &v.fields {
            Fields::Unit => None, // not possible
            Fields::Unnamed(_) => {
                // unnamed fields dont have names, so make them manually
                let fields = v
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(i, _)| Ident::new(&format!("f{}", i), Span::call_site().into()))
                    .collect::<Vec<_>>();

                Some(quote! {
                    #[allow(dead_code)]
                    #[must_use]
                    pub fn #function_name(&self) -> Option<(#( &#field_types ),*)> {
                        match self {
                            Self::#variant_ident(#( ref #fields ),*) => Some((#( #fields ),*)),
                            _ => None
                        }
                    }
                })
            }
            Fields::Named(_) => {
                // named fields have names
                let fields = v.fields.iter().map(|f| &f.ident).collect::<Vec<_>>();

                Some(quote! {
                    #[allow(dead_code)]
                    #[must_use]
                    pub fn #function_name(&self) -> Option<(#( &#field_types ),*)> {
                        match self {
                            Self::#variant_ident{#( ref #fields ),*} => Some((#( #fields ),*)),
                            _ => None
                        }
                    }
                })
            }
        }
    });

    quote! {
        impl #generics #enum_name #generics {
            #(#enum_methods)*
        }
    }
    .into()
}

pub(crate) fn impl_as_mut(ast: &ItemEnum) -> TokenStream {
    let enum_name = &ast.ident;
    let generics = &ast.generics;
    let variants = &ast.variants;

    let enum_methods = variants.iter().map(|v| {
        let variant_ident = &v.ident;

        let field_types = v
            .fields
            .iter()
            .enumerate()
            .map(|(_, f)| &f.ty)
            .collect::<Vec<_>>();

        let function_name = Ident::new(
            &format!("as_mut_{}", variant_ident).to_case(Case::Snake),
            Span::call_site().into(),
        );

        match &v.fields {
            Fields::Unit => None, // not possible
            Fields::Unnamed(_) => {
                // unnamed fields dont have names, so make them manually
                let fields = v
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(i, _)| Ident::new(&format!("f{}", i), Span::call_site().into()))
                    .collect::<Vec<_>>();

                Some(quote! {
                    #[allow(dead_code)]
                    #[must_use]
                    pub fn #function_name(&mut self) -> Option<(#( &mut #field_types ),*)> {
                        match self {
                            Self::#variant_ident(#( ref mut #fields ),*) => Some((#( #fields ),*)),
                            _ => None
                        }
                    }
                })
            }
            Fields::Named(_) => {
                // named fields have names
                let fields = v.fields.iter().map(|f| &f.ident).collect::<Vec<_>>();

                Some(quote! {
                    #[allow(dead_code)]
                    #[must_use]
                    pub fn #function_name(&mut self) -> Option<(#( &mut #field_types ),*)> {
                        match self {
                            Self::#variant_ident{#( ref mut #fields ),*} => Some((#( #fields ),*)),
                            _ => None
                        }
                    }
                })
            }
        }
    });

    quote! {
        impl #generics #enum_name #generics {
            #(#enum_methods)*
        }
    }
    .into()
}
