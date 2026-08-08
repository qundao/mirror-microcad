// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Implementation for the `#[builtin_mod]` attribute macro.

use crate::prelude::*;

pub fn builtin_mod_impl(item: TokenStream) -> TokenStream {
    let mut item_mod = parse_macro_input!(item as ItemMod);

    let mut collected_builtins = Vec::new();

    if let Some((_, items)) = &item_mod.content {
        for item in items {
            match item {
                // Scan functions with #[builtin_fn(...)]
                syn::Item::Fn(ItemFn { attrs, sig, .. }) => {
                    let has_builtin_fn =
                        attrs.iter().any(|attr| attr.path().is_ident("builtin_fn"));
                    if has_builtin_fn {
                        // Converts Rust fn name `greater_than` -> UPPERCASE static name `GREATER_THAN`
                        let static_ident =
                            format_ident!("{}", sig.ident.to_string().to_uppercase());
                        collected_builtins.push(static_ident);
                    }
                }
                // Scan statics with #[builtin_constant(...)]
                syn::Item::Static(ItemStatic { attrs, ident, .. }) => {
                    let has_builtin_constant = attrs
                        .iter()
                        .any(|attr| attr.path().is_ident("builtin_constant"));
                    if has_builtin_constant {
                        collected_builtins.push(ident.clone());
                    }
                }
                _ => {}
            }
        }
    }

    // Append `pub static ALL_BUILTINS` to the end of the module's item vector
    if let Some((_, items)) = &mut item_mod.content {
        items.push(parse_quote! {
            pub static ALL_BUILTINS: &[&'static Builtin] = &[
                #(&#collected_builtins),*
            ];
        });
    }

    quote! {
        #item_mod
    }
    .into()
}
