// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Implementation for the `#[builtin_fn(...)` attribute macro.

use quote::ToTokens;

use crate::prelude::*;

/// A built-in type.
enum BuiltinType {
    /// Any identifier
    Single(Ident),
    /// An identifier in `[]`
    Array(Ident),
}

impl BuiltinType {
    fn map_ty_ident(ty: &Ident) -> TokenStream2 {
        match ty.to_string().as_str() {
            "Scalar" => quote! { microcad_lang_types::Type::scalar() },
            "Angle" => quote! { microcad_lang_types::Type::angle() },
            "Length" => quote! { microcad_lang_types::Type::length() },
            "Mat3" => quote! { microcad_lang_types::Type::matrix(3,3) },
            "Model" => {
                quote! { microcad_lang_types::Type::Model(microcad_lang_type::model::ModelOutputType::Any) }
            }
            _ => quote! { microcad_lang_types::Type::#ty },
        }
    }
}

impl quote::ToTokens for BuiltinType {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let expanded = match self {
            BuiltinType::Single(ident) => Self::map_ty_ident(ident),
            BuiltinType::Array(ident) => {
                let ty = Self::map_ty_ident(ident);
                quote! { microcad_lang_types::Type::Array(Box::new(#ty)) }
            }
        };

        // Append the expanded tokens into the mutable buffer
        expanded.to_tokens(tokens);
    }
}
// Represents a parameter in the macro attribute, e.g.: `lhs: Any`
struct BuiltinParam {
    name: Ident,
    ty: BuiltinType,
}

impl quote::ToTokens for BuiltinParam {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.name;
        let ty = &self.ty;

        quote! {
            #name: #ty
        }
        .to_tokens(tokens);
    }
}

impl Parse for BuiltinType {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(syn::token::Bracket) {
            let content;
            syn::bracketed!(content in input);
            Ok(Self::Array(content.parse()?))
        } else {
            Ok(Self::Single(input.parse()?))
        }
    }
}

impl Parse for BuiltinParam {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: BuiltinType = input.parse()?;

        Ok(Self { name, ty })
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
    return_type: Option<BuiltinType>,
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

    let static_name = helpers::ident_upper(&fn_name);

    // Generate list of parameters from input tokens.
    let formatted_params = if is_variadic {
        quote! { (*) }
    } else {
        let formatted_params = params.into_iter().map(BuiltinParam::into_token_stream);
        quote! { (#(#formatted_params),*) }
    };

    let return_type = match return_type {
        Some(return_type) => {
            let return_type = return_type.into_token_stream();
            quote! { -> #return_type  }
        }
        None => {
            quote! {}
        }
    };

    let id_doc = format!(
        "**Static ID:** `{}`",
        microcad_hash::HashId::compile_time_hash(format!("__mu::{fn_name}::{mod_name}").as_str())
    );
    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;
    let fn_block = &input_fn.block;
    let fn_attrs = &input_fn.attrs;

    let fn_with_doc = quote! {
        #(#fn_attrs)*
        #[doc = ""]
        #[doc = #id_doc]
        #fn_vis #fn_sig {
            #fn_block
        }
    };

    let doc_comment = helpers::attr_fetch_doc(&input_fn.attrs);

    quote! {
        #fn_with_doc

        pub static #static_name: Builtin = builtin!(
            Function
            #doc_comment
            #mod_name::#name( microcad_lang_types::function_type!(#formatted_params #return_type))
        );
    }
    .into()
}
