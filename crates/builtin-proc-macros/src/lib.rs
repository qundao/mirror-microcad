// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

extern crate proc_macro;

mod builtin_constant;
mod builtin_fn;
mod builtin_mod;
mod test_builtin_fn;

pub(crate) mod prelude {
    pub use proc_macro::TokenStream;

    pub use quote::{format_ident, quote};
    pub use syn::parse::{Parse, ParseStream};
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

#[proc_macro_attribute]
pub fn test_builtin_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    test_builtin_fn::test_builtin_fn_impl(attr, item)
}
