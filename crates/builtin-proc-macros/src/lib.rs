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
    let input = parse_macro_input!(item as ItemFn);

    let fn_name = &input.sig.ident;
    let fn_name_str = fn_name.to_string();
    let body = &input.block;
    let vis = &input.vis;

    let mut param_definitions = Vec::new();
    let mut arg_extractions = Vec::new();
    use syn::*;

    for arg in &input.sig.inputs {
        if let FnArg::Typed(pat_type) = arg {
            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                let name = &pat_ident.ident;
                let name_str = name.to_string();

                // Extract the type (e.g., Angle, String)
                if let Type::Path(type_path) = &*pat_type.ty {
                    let ty = &type_path.path.segments.last().unwrap().ident;

                    // 1. Create the parameter! macro calls
                    param_definitions.push(quote! {
                        parameter!(#name: #ty)
                    });

                    // 2. Create the extraction logic inside the match
                    arg_extractions.push(quote! {
                        let #name: #ty = matched_args.get(#name_str);
                    });
                }
            }
        }
    }

    let expanded = quote! {
        #vis fn #fn_name() -> Symbol {
            use microcad_lang::parameter;
            Symbol::new_builtin_fn(
                #fn_name_str,
                [
                    #(#param_definitions),*
                ].into_iter(),
                &|params, args, ctx| {
                    match microcad_lang::eval::ArgumentMatch::find_match(args, params) {
                        Ok(matched_args) => {
                            use microcad_lang::builtin::ValueAccess;
                            #(#arg_extractions)*
                            Ok(#body)
                        }
                        Err(err) => {
                            ctx.error(args, err)?;
                            Ok(Value::None)
                        }
                    }
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
