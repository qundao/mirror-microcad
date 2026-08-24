// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! `include_inner_docs` proc macro implementation.

use crate::prelude::*;

fn expand_include_inner_docs(lit_str: &syn::LitStr) -> syn::Result<String> {
    let relative_path = lit_str.value();

    // Resolve path relative to the manifest directory of the calling crate
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let file_path = std::path::Path::new(&manifest_dir).join(&relative_path);

    // 1. Read file
    let content = std::fs::read_to_string(&file_path).map_err(|err| {
        syn::Error::new(
            lit_str.span(),
            format!("Failed to read '{}': {err}", file_path.display()),
        )
    })?;

    // 2. Extract inner docs
    let inner_docs = syn::parse_file(&content)?
        .attrs
        .into_iter()
        .filter_map(|attr| match (attr.style, attr.meta) {
            (
                syn::AttrStyle::Inner(_),
                syn::Meta::NameValue(syn::MetaNameValue {
                    value:
                        syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(s),
                            ..
                        }),
                    path,
                    ..
                }),
            ) if path.is_ident("doc") => {
                let val = s.value();
                Some(val.strip_prefix(' ').unwrap_or(&val).to_string())
            }
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");

    Ok(inner_docs)
}

/// Extract the inner doc comments `//!` in a source file.
pub fn include_inner_docs_impl(input: TokenStream) -> TokenStream {
    let lit_str = parse_macro_input!(input as syn::LitStr);

    match expand_include_inner_docs(&lit_str) {
        Ok(docs) => quote!(#docs).into(),
        Err(err) => err.to_compile_error().into(),
    }
}
