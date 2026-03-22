// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::*;

#[proc_macro_derive(SrcReferrer)]
pub fn derive_src_referrer(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();

    // parse fields, validate etc
    // Only support structs with named fields
    match &input.data {
        Data::Struct(ds) => match &ds.fields {
            // Generate SrcReferrer for a struct with fields `struct Foo { bar: Integer, src_ref: SrcRef };`.
            Fields::Named(_) => {
                quote! {
                    impl microcad_lang_base::SrcReferrer for #name {
                        fn src_ref(&self) -> microcad_lang_base::SrcRef  {
                            self.src_ref.clone()
                        }
                    }
                }
            }
            Fields::Unnamed(_) => {
                quote! {
                    impl microcad_lang_base::SrcReferrer for #name {
                        fn src_ref(&self) -> microcad_lang_base::SrcRef  {
                            self.0.src_ref()
                        }
                    }
                }
            }
            // Unit structs not supported.
            Fields::Unit => {
                return Error::new_spanned(&name, format!("Unit structs are not supported"))
                    .to_compile_error()
                    .into();
            }
        },
        _ => {
            return Error::new_spanned(&name, format!("Unions and enums are not supported"))
                .to_compile_error()
                .into();
        }
    }
    .into()
}

/*
#[proc_macro_derive(TreeDisplay)]
pub fn derive_tree_display(input: TokenStream) -> TokenStream {}

#[proc_macro_derive(Identifiable)]
pub fn derive_id(input: TokenStream) -> TokenStream {}
*/
