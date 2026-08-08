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

// Parses the syntax inside #[builtin_fn(#mod_name::#name(arg1: Type1, ...) -> #return_type)]
struct BuiltinFnSig {
    /// E.g. `core`
    mod_name: Ident,
    /// E:g. `add`
    name: Ident,
    params: Vec<BuiltinParam>,
    return_type: Ident,
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
        while !content.is_empty() {
            params.push(content.parse::<BuiltinParam>()?);
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            } else {
                break;
            }
        }

        // 3. Parse `-> return_type`
        input.parse::<Token![->]>()?;
        let return_type: Ident = input.parse()?;

        Ok(Self {
            mod_name,
            name,
            params,
            return_type,
        })
    }
}

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

#[proc_macro_attribute]
pub fn builtin_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the macro attribute syntax
    let BuiltinFnSig {
        name,
        mod_name,
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
            #mod_name::#name(#(#formatted_params),*) -> Type::#return_ty_ident
        );
    }
    .into()
}

#[proc_macro_attribute]
pub fn builtin_constant(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the macro attribute syntax
    let BuiltinConstantSig { name, mod_name } = parse_macro_input!(attr as BuiltinConstantSig);

    // Parse the annotated static item (e.g., `pub static PI: Builtin = std::f64::consts::PI;`)
    let input_static = parse_macro_input!(item as ItemStatic);

    // Extract doc comment string
    let mut doc_comment = String::new();
    for attr in &input_static.attrs {
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
        #vis static #static_name: Builtin = builtin_constant_helper!(
            #doc_comment
            #mod_name::#name = #expr
        );
    }
    .into()
}

#[proc_macro_attribute]
pub fn builtin_mod(_attr: TokenStream, item: TokenStream) -> TokenStream {
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
            pub static ALL_BUILTINS: &[&Builtin] = &[
                #(&#collected_builtins),*
            ];
        });
    }

    quote! {
        #item_mod
    }
    .into()
}
