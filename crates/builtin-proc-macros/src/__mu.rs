// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

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

pub(crate) fn __mu_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MuInput);
    let mut path = input.path;

    // Extract the last path segment (e.g., `greater_than` from `core::greater_than`)
    let last_segment = match path.segments.pop() {
        Some(pair) => pair.into_value(),
        None => {
            return syn::Error::new(path.span(), "Expected a valid path segment")
                .to_compile_error()
                .into();
        }
    };

    let fn_ident = last_segment.ident;

    // Convert to SCREAMING_SNAKE_CASE using case conversion library
    let screaming_name = fn_ident.to_string().to_uppercase();
    let screaming_ident = Ident::new(&screaming_name, fn_ident.span());

    // Reconstruct module path leading to the static item
    let module_path = &path;

    // Full path to static constant: microcad_builtin::mu::core::GREATER_THAN
    let static_const_path = quote! {
        microcad_builtin::mu:: #module_path #screaming_ident
    };

    // Full path to target function for static checking: core::greater_than
    let fn_path = if module_path.segments.is_empty() {
        quote! { #fn_ident }
    } else {
        quote! { #module_path #fn_ident }
    };

    quote! {
        {
            // Verifies function exists at compile time
            let _check_fn_exists = microcad_builtin::mu:: #fn_path;

            // Resolves static item ID
            #static_const_path.id().into()
        }
    }
    .into()
}
