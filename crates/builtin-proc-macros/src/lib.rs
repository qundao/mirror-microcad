// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

extern crate proc_macro;

mod derive;

use derive::derive_workbench_definition;
use proc_macro::TokenStream;
use syn::*;

/// Get all doc comments as concetenated string.
fn get_doc_block(attrs: &[Attribute]) -> String {
    attrs.iter().filter_map(|attr|
        // Parse the meta of the attribute
        if attr.path().is_ident("doc")
            && let syn::Meta::NameValue(nv) = &attr.meta
            && let syn::Expr::Lit(ExprLit{ lit: Lit::Str(lit_str), ..}) = &nv.value {
            // Return the string value, e.g., "Doc test"
            Some(String::from(lit_str.value().trim()))
        } else {
            None
        }
    ).collect::<Vec<_>>().join("\n")
}

#[proc_macro_derive(BuiltinPrimitive2D)]
pub fn derive_primitive2d(input: TokenStream) -> TokenStream {
    derive_workbench_definition(input, "Primitive2D", "Geometry2D")
}

#[proc_macro_derive(BuiltinPrimitive3D)]
pub fn derive_primitive3d(input: TokenStream) -> TokenStream {
    derive_workbench_definition(input, "Primitive3D", "Geometry3D")
}

#[proc_macro_derive(BuiltinOperation)]
pub fn derive_operation(input: TokenStream) -> TokenStream {
    derive_workbench_definition(input, "Operation", "NotDetermined")
}

#[proc_macro_derive(BuiltinOperation2D)]
pub fn derive_operation2d(input: TokenStream) -> TokenStream {
    derive_workbench_definition(input, "Operation", "Geometry2D")
}

#[proc_macro_derive(BuiltinOperation3D)]
pub fn derive_operation3d(input: TokenStream) -> TokenStream {
    derive_workbench_definition(input, "Operation", "Geometry3D")
}

use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{FnArg, Ident, ItemFn, Result, Token, Type, parse_macro_input};

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

// Parses the syntax inside #[builtin_fn((arg1: Type1, ...) -> ReturnType)]
struct BuiltinFnSig {
    params: Vec<BuiltinParam>,
    return_type: Ident,
}

impl Parse for BuiltinFnSig {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        syn::parenthesized!(content in input);

        let mut params = Vec::new();
        while !content.is_empty() {
            params.push(content.parse::<BuiltinParam>()?);
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            } else {
                break;
            }
        }

        input.parse::<Token![->]>()?;
        let return_type: Ident = input.parse()?;

        Ok(BuiltinFnSig {
            params,
            return_type,
        })
    }
}

#[proc_macro_attribute]
pub fn builtin_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the macro attribute syntax
    let BuiltinFnSig {
        params,
        return_type,
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
    // Generate uppercase static name (e.g., `add` -> `ADD`)
    let static_name = format_ident!("{}", fn_name.to_string().to_uppercase());

    // Reconstruct param list format: `lhs: Type::Any, rhs: Type::Any`
    let formatted_params = params.iter().map(|p| {
        let p_name = &p.name;
        let p_ty = format_ident!("{}", p.ty);
        quote! { #p_name: Type::#p_ty }
    });

    let return_ty_ident = format_ident!("{}", return_type);

    quote! {
        #input_fn

        pub static #static_name: Builtin = builtin_function_helper!(
            #doc_comment
            core::#fn_name(#(#formatted_params),*) -> Type::#return_ty_ident
        );
    }
    .into()
}
