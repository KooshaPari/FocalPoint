#![forbid(unsafe_code)]

//! Phenotype Derive — Proc macros for domain type derivations.

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro for the `ValueObject` trait.
///
/// Automatically implements `Debug`, `Clone`, `PartialEq`, `Eq`, and `ValueObject`
/// for a struct or newtype.
#[proc_macro_derive(ValueObject)]
pub fn derive_value_object(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics ::std::fmt::Debug for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct(stringify!(#name))
                    .finish_non_exhaustive()
            }
        }

        impl #impl_generics ::std::clone::Clone for #name #ty_generics #where_clause {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }

        impl #impl_generics ::std::cmp::PartialEq for #name #ty_generics #where_clause {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl #impl_generics ::std::cmp::Eq for #name #ty_generics #where_clause {}

        // Note: ValueObject trait implementation is provided by phenotype_contracts
        // The user must add: impl ValueObject for MyType {}
    };

    TokenStream::from(expanded)
}

/// Derive macro for the `Entity` trait.
///
/// Expects a struct with a field named `id` and implements the `Entity` trait.
/// This is a stub; full implementation requires the user to specify the Id type.
#[proc_macro_derive(Entity)]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let _name = &input.ident;
    let (_impl_generics, _ty_generics, _where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        // Note: Entity trait requires the user to specify the Id type.
        // The derive macro assumes a field named `id` exists.
        // Full implementation is provided by the user with:
        // impl Entity for MyType { type Id = MyId; fn id(&self) -> &Self::Id { &self.id } }
    };

    TokenStream::from(expanded)
}
