// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

extern crate proc_macro;

mod derive;

use derive::derive_workbench_definition;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::{Parse, ParseStream}, *};

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



#[proc_macro_attribute]
pub fn builtin_mod(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut module = parse_macro_input!(item as ItemMod);

    let mod_name = module.ident.to_string();
    let mut keys = Vec::new();
    let mut fn_paths = Vec::new();

    // Inspect the items inside the module block
    if let Some((_, items)) = &mut module.content {
        for item in items.iter() {
            if let Item::Fn(func) = item {
                let fn_name = func.sig.ident.to_string();
                
                // Accumulate namespace: "core" + "::" + "add" => "core::add"
                let full_path = format!("__mu::{}::{}", mod_name, fn_name);
                let hash = microcad_hash::fnv1a_hash(&full_path);

                let fn_ident = &func.sig.ident;
                let mod_ident = &module.ident;

                keys.push(hash);
                // Qualified path to function inside module: core::add
                fn_paths.push(quote! { #mod_ident::#fn_ident });
            }
        }
    }

    // Generate the original module + a static phf registry table for this module
    let expanded = quote! {
        #module

        pub static BUILTIN_REGISTRY: phf::Map<u64, BuiltinFn> = phf::phf_map! {
            #( #keys => #fn_paths, )*
        };
    };

    TokenStream::from(expanded)
}