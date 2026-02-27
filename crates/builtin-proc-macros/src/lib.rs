// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

extern crate proc_macro;

mod derive;

use derive::derive_workbench_definition;
use proc_macro::TokenStream;
use quote::quote;

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

/// Attribute macro used to declare a µcad builtin module.
///
/// It automatically generate its symbol registration function of the same name.
/// This macro is designed to reduce boilerplate when defining built-in modules in µcad.
///
/// # Example
///
/// Let's a look a built-in module `math`:
///
/// ```rust
/// #[builtin_mod]
/// pub mod math {
///     pub const PI: Scalar = 3.14;
///
///     pub fn int() -> Symbol {
///         // ...
///     }
/// }
/// ```
///
/// The `#[builtin_mod]` will auto-generate a `math` registration function:
///
/// ```rust
/// pub fn math() -> microcad_lang::resolve::Symbol {
///     crate::ModuleBuilder::new("math")
///         .pub_const("PI", math::PI)
///         .symbol(math::add())
///         .build()
/// }
/// ```
///
///
/// # Conditions
///
/// - Private items are ignored.
/// - Non-const / non-function items are ignored.
/// - The module must be **inline** (`mod name { ... }`).
#[proc_macro_attribute]
pub fn builtin_mod(_attr: TokenStream, item: TokenStream) -> TokenStream {
    use syn::*;
    let item_mod = parse_macro_input!(item as ItemMod);
    let mod_name = &item_mod.ident;
    let mod_name_str = mod_name.to_string();

    let registrations = item_mod
        .content
        .as_ref()
        .map(|(_, items)| items)
        .expect("Some inline module")
        .iter()
        .filter_map(|item| {
            match item {
                // Match only public constants
                Item::Const(c) if matches!(c.vis, Visibility::Public(_)) => {
                    let name = &c.ident;
                    let name_str = name.to_string();
                    Some(quote! { .pub_const(#name_str, #mod_name::#name) })
                }

                // Match only public functions
                Item::Fn(f) if matches!(f.vis, Visibility::Public(_)) => {
                    let name = &f.sig.ident;
                    Some(quote! { .symbol(#mod_name::#name()) })
                }

                // Skip everything else (Private items or different Item types)
                _ => None,
            }
        })
        .collect::<Vec<_>>();

    // Generate the builder function and keep the original module items
    TokenStream::from(quote! {
        #item_mod

        pub fn #mod_name() -> microcad_lang::resolve::Symbol {
            crate::ModuleBuilder::new(#mod_name_str)
                #(#registrations)*
                .build()
        }
    })
}
