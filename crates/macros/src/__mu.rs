// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use proc_macro2::Span;

use crate::prelude::*;

struct MuInput {
    path: Path,
}

impl Parse for MuInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(MuInput {
            path: input.parse()?,
        })
    }
}

fn get_mod_mu() -> proc_macro2::TokenStream {
    match proc_macro_crate::crate_name("microcad-builtin") {
        Ok(proc_macro_crate::FoundCrate::Itself) => quote! { crate::mu },
        Ok(proc_macro_crate::FoundCrate::Name(name)) => {
            let ident = Ident::new(&name, Span::call_site());
            quote! { ::#ident::mu }
        }
        Err(_) => panic!("Crate not found"), // quote! { ::microcad_builtin::mu }, // Fallback
    }
}

pub(crate) fn __mu_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MuInput);
    let path = &input.path;

    if path.segments.len() != 2 {
        return syn::Error::new_spanned(
            path,
            "Please provide a path with exactly two segments (e.g. `core::add`).",
        )
        .to_compile_error()
        .into();
    }

    let __mu = get_mod_mu(); // `__mu`
    let module = &path.segments[0].ident; // `core`
    let fn_ident = &path.segments[1].ident; // `add`
    let static_ident = Ident::new(&fn_ident.to_string().to_uppercase(), fn_ident.span()); // `ADD`

    quote! {
        {
            // 1. Compile-time existence check for function item
            use #__mu::#module::#fn_ident;

            // 2. Resolve static BuiltinId item
            #__mu::#module::#static_ident.id().into()
        }
    }
    .into()
}
