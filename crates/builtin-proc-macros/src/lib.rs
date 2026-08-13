// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

extern crate proc_macro;

mod __mu;
mod builtin_constant;
mod builtin_fn;
mod builtin_mod;
mod test_builtin_fn;

pub(crate) mod prelude {
    pub use proc_macro::TokenStream;

    pub use quote::{format_ident, quote};
    pub use syn::parse::{Parse, ParseStream};
    pub use syn::spanned::Spanned;
    pub use syn::{
        Expr, Ident, ItemFn, ItemMod, ItemStatic, Path, Result, Token, parse_macro_input,
        parse_quote,
    };
}

use prelude::*;

#[proc_macro_attribute]
pub fn builtin_constant(attr: TokenStream, item: TokenStream) -> TokenStream {
    builtin_constant::builtin_constant_impl(attr, item)
}

#[proc_macro_attribute]
pub fn builtin_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    builtin_fn::builtin_fn_impl(attr, item)
}

#[proc_macro_attribute]
pub fn builtin_mod(_attr: TokenStream, item: TokenStream) -> TokenStream {
    builtin_mod::builtin_mod_impl(item)
}

/// Create a test case for a built-in function.
///
/// Expands `core::add(3, 4) == 7` to
/// ```rs
/// fn add() {
///     let mut ctx = BuiltinEvalContext::new();
///     assert_that!(core::add(arguments!(3, 4), &mut ctx), eq(8))
/// }
/// ```
#[proc_macro_attribute]
pub fn test_builtin_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    test_builtin_fn::test_builtin_fn_impl(attr, item)
}

/// Macro to get the hash from built-in function.
#[proc_macro]
pub fn __mu(input: TokenStream) -> TokenStream {
    __mu::__mu_impl(input)
}
