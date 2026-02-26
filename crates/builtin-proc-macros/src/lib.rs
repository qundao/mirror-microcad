// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

extern crate proc_macro;

mod derive;

use derive::derive_workbench_definition;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

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

#[proc_macro_attribute]
pub fn builtin_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // 1. Parse the input function
    let input = parse_macro_input!(item as ItemFn);

    let name = &input.sig.ident;
    let name_str = name.to_string();
    let body = &input.block;
    let vis = &input.vis;
    let attrs = &input.attrs;

    // 1. Extract arguments from the function signature
    let mut arg_names = Vec::new();
    let mut arg_types = Vec::new();

    for arg in &input.sig.inputs {
        if let syn::FnArg::Typed(pat_type) = arg {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                arg_names.push(&pat_ident.ident);
                // We assume the type name matches the Value enum variant (e.g., String, Integer)
                if let syn::Type::Path(type_path) = &*pat_type.ty {
                    if let Some(segment) = type_path.path.segments.last() {
                        arg_types.push(&segment.ident);
                    }
                }
            }
        }
    }

    // 2. Rebuild the function with the boilerplate included
    let expanded = quote::quote! {
        #(#attrs)*
        #vis fn #name() -> Symbol {
            Symbol::new_builtin_fn(
                #name_str,
                [].into_iter(),
                &|_params, args, ctx| {
                    let arg = args.get_single()?;
                    #body
                },
                None,
            )
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn builtin_mod(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let mod_name = &input.sig.ident;
    let mod_name_str = mod_name.to_string();
    let vis = &input.vis;

    // 1. Parse the function body to find the [fn1, fn2] list
    // We expect the body to be exactly one expression: an array.
    let symbols = if let Some(syn::Stmt::Expr(syn::Expr::Array(syn::ExprArray { elems, .. }), _)) =
        input.block.stmts.first()
    {
        elems.iter().map(|expr| {
            quote! { .symbol(#expr()) }
        })
    } else {
        panic!("Expected a list of functions in square brackets, e.g., [count, len]");
    };

    // 2. Generate the ModuleBuilder boilerplate
    let expanded = quote! {
        #vis fn #mod_name() -> Symbol {
            crate::ModuleBuilder::new(#mod_name_str)
                #(#symbols)*
                .build()
        }
    };

    TokenStream::from(expanded)
}
