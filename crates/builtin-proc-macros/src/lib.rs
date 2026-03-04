// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

extern crate proc_macro;

mod derive;

use derive::derive_workbench_definition;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::{Parse, ParseStream}, *};

/// Get all doc comments as concetenated string.
fn get_doc_comment(attrs: &[Attribute]) -> String {
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

/// Attribute macro used to declare a µcad builtin module.
///
/// It automatically generate its symbol registration function of the same name.
/// This macro is designed to reduce boilerplate when defining built-in modules in µcad.
///
/// # Example
///
/// Let's a look a built-in module `math`:
///
/// ```rust,ignore
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
/// ```rust,ignore
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
                // Match `pub use foo::bar` statements.
                Item::Use(u) if matches!(u.vis, Visibility::Public(_)) => {
                    let tree = &u.tree;
                    Some(quote! { .symbol(#tree()) })
                }

                // Skip everything else (Private items or different Item types)
                _ => None,
            }
        })
        .collect::<Vec<_>>();

    // Generate the builder function and keep the original module items
    TokenStream::from(quote! {
        #item_mod

        #[allow(missing_docs)]
        pub fn #mod_name() -> microcad_lang::resolve::Symbol {
            crate::ModuleBuilder::new(#mod_name_str)
                #(#registrations)*
                .build()
        }
    })
}

// Represents: x: Scalar = 3.0
struct BuiltinParam {
    name: Ident,
    ty: Option<Type>,
    default_value: Option<Expr>,
}

impl Parse for BuiltinParam {
    fn parse(input: ParseStream) -> Result<Self> {
        // 1. Parse the mandatory name (e.g., 'x')
        let name: Ident = input.parse()?;

        // 2. Look for an optional ':' followed by a Type
        let ty = if input.peek(Token![:]) {
            let _colon: Token![:] = input.parse()?;
            Some(input.parse::<Type>()?)
        } else {
            None
        };

        // 3. Look for an optional '=' followed by an Expression
        let default_value = if input.peek(Token![=]) {
            let _eq: Token![=] = input.parse()?;
            Some(input.parse::<Expr>()?)
        } else {
            None
        };

        Ok(BuiltinParam { name, ty, default_value })
    }
}

#[proc_macro_attribute]
pub fn builtin_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    use syn::*;

    let parser = punctuated::Punctuated::<BuiltinParam, Token![,]>::parse_terminated;
    let attrs = parse_macro_input!(attr with parser);

    // Parse the function
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_attrs = &input_fn.attrs;
    let fn_docs = get_doc_comment(fn_attrs);
    let fn_body = &input_fn.block; // This is the closure returned by the user

    // Generate the parameter! calls
    let params = attrs.iter().map(|p| {
        let name_ident = &p.name;
        
        // Handle optional types/defaults in your parameter! macro
        // Assuming your parameter! macro supports these fields:
        match (&p.ty, &p.default_value) {
            (Some(t), Some(d)) => quote! { parameter!(#name_ident: #t = #d) },
            (Some(t), None)    => quote! { parameter!(#name_ident: #t) },
            (None, Some(d))    => quote! { parameter!(#name_ident = #d) },
            (None, None)       => quote! { parameter!(#name_ident) },
        }
    });

    TokenStream::from(quote! {
        #(#fn_attrs)*
        #fn_vis fn #fn_name() -> crate::Symbol {
            crate::Symbol::new_builtin_fn(
                stringify!(#fn_name),
                vec![#(#params),*].into_iter(),
                &#fn_body,
                Some(#fn_docs),
            )
        }
    })
}
