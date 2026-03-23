// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::*;

/// Derives the `SrcReferrer` trait for structs.
///
/// This macro supports two types of data structures:
/// 1. **Named Structs**: Automatically implements `src_ref()` by cloning a field
///    named `src_ref`.
/// 2. **Unnamed (Tuple) Structs**: Automatically implements `src_ref()` by
///    delegating to the first element (`self.0`). The first element must
///    implement the `SrcReferrer` trait.
///
/// # Panics
/// Will fail to compile if applied to Enums, Unions, or Unit structs.
#[proc_macro_derive(SrcReferrer)]
pub fn derive_src_referrer(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Only support structs with named and unnamed fields.
    match &input.data {
        Data::Struct(ds) => match &ds.fields {
            // Generate SrcReferrer for a struct with `src_ref` field:
            // `struct Foo { bar: Integer, src_ref: SrcRef };`.
            Fields::Named(_) => {
                quote! {
                    impl microcad_lang_base::SrcReferrer for #name {
                        fn src_ref(&self) -> microcad_lang_base::SrcRef  {
                            self.src_ref.clone()
                        }
                    }
                }
            }
            // Generate SrcReferrer for a tuple `struct Bar(Refer<Identifier>);`.
            Fields::Unnamed(_) => {
                quote! {
                    impl microcad_lang_base::SrcReferrer for #name {
                        fn src_ref(&self) -> microcad_lang_base::SrcRef  {
                            self.0.src_ref()
                        }
                    }
                }
            }
            // Unit structs are not supported.
            Fields::Unit => {
                Error::new_spanned(name, "Unit structs are not supported").to_compile_error()
            }
        },
        _ => Error::new_spanned(name, "Unions and enums are not supported").to_compile_error(),
    }
    .into()
}

/// Derives the `Identifiable` trait for named structs.
///
/// This macro implements `id_ref()` by returning a reference to an `id` field.
/// The `id` field must be of type `crate::Identifier`.
///
/// # Constraints
/// - Only works on **Named Structs**.
/// - Does **not** support Tuple structs, Unit structs, Enums, or Unions.
#[proc_macro_derive(Identifiable)]
pub fn derive_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();

    match &input.data {
        Data::Struct(ds) => match &ds.fields {
            // Generate `Identifiable` for a struct with `id` field `struct Foo { bar: Integer, id: Identifier };`.
            Fields::Named(_) => {
                quote! {
                    impl crate::Identifiable for #name {
                        fn id_ref(&self) -> &crate::Identifier  {
                            &self.id
                        }
                    }
                }
            }
            Fields::Unnamed(_) => {
                Error::new_spanned(name, "Unnamed structs are not supported").to_compile_error()
            }
            // Unit structs not supported.
            Fields::Unit => {
                Error::new_spanned(name, "Unit structs are not supported").to_compile_error()
            }
        },
        _ => Error::new_spanned(name, "Unions and enums are not supported").to_compile_error(),
    }
    .into()
}
