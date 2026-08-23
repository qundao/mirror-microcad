// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Proc macro helpers. These functions can be used in proc macros.

/// Extract doc comment string from attributes.
pub fn attr_fetch_doc(attrs: &[syn::Attribute]) -> String {
    attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc")
                && let Ok(nv) = attr.meta.require_name_value()
                && let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit_str),
                    ..
                }) = &nv.value
            {
                Some(lit_str.value().trim().to_string())
            } else {
                None
            }
        })
        .reduce(|acc, line| format!("{acc}\n{line}"))
        .unwrap_or_default()
}

/// Check if an attribute with a name exists with an attribute list.
pub fn attr_exists(attrs: &[syn::Attribute], name: &str) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident(name))
}

/// Generate uppercase name (e.g., `add` -> `ADD`)
pub fn ident_upper(ident: &syn::Ident) -> syn::Ident {
    quote::format_ident!("{}", ident.to_string().to_uppercase())
}
