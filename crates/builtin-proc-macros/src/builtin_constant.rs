// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Implementation for the `#[builtin_constant(...)` attribute macro.

use crate::prelude::*;

// Parses the syntax inside #[builtin_fn(#mod_name::#name)]
struct BuiltinConstantSig {
    /// E.g. `math`
    mod_name: Ident,
    /// E:g. `PI`
    name: Ident,
}

impl Parse for BuiltinConstantSig {
    fn parse(input: ParseStream) -> Result<Self> {
        // 1. Parse `mod_name::name`
        let mod_name: Ident = input.parse()?;
        input.parse::<Token![::]>()?;
        let name: Ident = input.parse()?;

        Ok(Self { mod_name, name })
    }
}

pub(crate) fn builtin_constant_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the macro attribute syntax
    let BuiltinConstantSig { name, mod_name } = parse_macro_input!(attr as BuiltinConstantSig);

    // Parse the annotated static item (e.g., `pub static PI: Builtin = std::f64::consts::PI;`)
    let input_static = parse_macro_input!(item as ItemStatic);

    // Extract doc comment string
    let mut doc_comment = String::new();
    for attr in &input_static.attrs {
        if attr.path().is_ident("doc")
            && let Ok(syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(lit_str),
                ..
            })) = &attr.meta.require_name_value().map(|nv| &nv.value)
        {
            doc_comment.push_str(lit_str.value().trim());
        }
    }

    let static_name = &input_static.ident;

    if name != *static_name {
        return syn::Error::new_spanned(
            &name,
            format!(
                "builtin constant name name `{}` does not match Rust static name `{}`",
                name, static_name
            ),
        )
        .to_compile_error()
        .into();
    }

    let expr = &input_static.expr;
    let vis = &input_static.vis;

    // Generate output expansion wrapping inside `builtin_constant_helper!`
    quote! {
        #vis static #static_name: Builtin = builtin!(
            Constant
            #doc_comment
            #mod_name::#name = #expr
        );
    }
    .into()
}
