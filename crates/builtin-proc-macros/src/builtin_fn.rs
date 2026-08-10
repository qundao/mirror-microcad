// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Implementation for the `#[builtin_fn(...)` attribute macro.

use crate::prelude::*;

// Represents a parameter in the macro attribute: `lhs: Any`
struct BuiltinParam {
    name: Ident,
    ty: Ident,
}

impl Parse for BuiltinParam {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Ident = input.parse()?;
        Ok(BuiltinParam { name, ty })
    }
}

// Parses the syntax inside #[builtin_fn(#mod_name::#name(arg1: Type1, ...) -> #return_type)]
struct BuiltinFnSig {
    /// E.g. `core`
    mod_name: Ident,
    /// E:g. `add`
    name: Ident,
    is_variadic: bool,
    params: Vec<BuiltinParam>,
    return_type: Option<Ident>,
}

impl Parse for BuiltinFnSig {
    fn parse(input: ParseStream) -> Result<Self> {
        // 1. Parse `mod_name::name`
        let mod_name: Ident = input.parse()?;
        input.parse::<Token![::]>()?;
        let name: Ident = input.parse()?;

        // 2. Parse parenthesized parameter list `(arg1: Type1, ...)`
        let content;
        syn::parenthesized!(content in input);

        let mut params = Vec::new();
        let mut is_variadic = false;

        while !content.is_empty() {
            if content.peek(syn::token::Star) {
                is_variadic = true;
                content.parse::<Token![*]>()?;
                break;
            }
            params.push(content.parse::<BuiltinParam>()?);

            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            } else {
                break;
            }
        }

        // 3. Optionally Parse `-> return_type`
        let return_type = if input.peek(Token![->]) {
            input.parse::<Token![->]>()?;
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Self {
            mod_name,
            name,
            params,
            return_type,
            is_variadic,
        })
    }
}

pub(crate) fn builtin_fn_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the macro attribute syntax
    let BuiltinFnSig {
        name,
        mod_name,
        params,
        return_type,
        is_variadic,
    } = parse_macro_input!(attr as BuiltinFnSig);

    // Parse the annotated function
    let input_fn = parse_macro_input!(item as ItemFn);

    // Get the doc string comment if present
    let mut doc_comment = String::new();
    for attr in &input_fn.attrs {
        if attr.path().is_ident("doc") {
            if let Ok(syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(lit_str),
                ..
            })) = &attr.meta.require_name_value().map(|nv| &nv.value)
            {
                doc_comment.push_str(lit_str.value().trim());
            }
        }
    }

    let fn_name = &input_fn.sig.ident;

    if name != *fn_name {
        return syn::Error::new_spanned(
            &name,
            format!(
                "builtin function name `{}` does not match Rust function name `{}`",
                name, fn_name
            ),
        )
        .to_compile_error()
        .into();
    }

    // Generate uppercase static name (e.g., `add` -> `ADD`)
    let static_name = format_ident!("{}", fn_name.to_string().to_uppercase());

    let formatted_params = if is_variadic {
        quote! { (*) }
    } else {
        // Reconstruct param list format: `lhs: Type::Any, rhs: Type::Any`
        let formatted_params = params.iter().map(|p| {
            let p_name = &p.name;
            let p_ty = format_ident!("{}", p.ty);
            quote! { #p_name: microcad_lang_types::Type::#p_ty }
        });

        quote! { (#(#formatted_params),*) }
    };

    let return_type = match return_type {
        Some(return_type) => {
            let return_type = format_ident!("{}", return_type);
            quote! { -> microcad_lang_types::Type::#return_type  }
        }
        None => {
            quote! {}
        }
    };

    quote! {
        #input_fn

        pub static #static_name: Builtin = builtin_function_helper!(
            #doc_comment
            #mod_name::#name( microcad_lang_types::function_type!(#formatted_params #return_type))
        );
    }
    .into()
}
