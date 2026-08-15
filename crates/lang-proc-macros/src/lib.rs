// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

extern crate proc_macro;

pub(crate) mod prelude {
    pub use syn::*;
}

use prelude::*;

/// Derives the `SrcReferrer` trait for structs.
///
/// This macro supports two types of data structures:
/// 1. **Named Structs**: Automatically implements `src_ref()` by cloning a field
///    named `src_ref`.
/// 2. **Unnamed (Tuple) Structs**: Automatically implements `src_ref()` by
///    delegating to the first element (`self.0`). The first element must
///    implement the `SrcReferrer` trait.
///
/// # Panics
/// Will fail to compile if applied to Enums, Unions, or Unit structs.
#[proc_macro_derive(SrcReferrer)]
pub fn derive_src_referrer(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Only support structs with named and unnamed fields.
    match &input.data {
        Data::Struct(ds) => match &ds.fields {
            // Generate SrcReferrer for a struct with `src_ref` field:
            // `struct Foo { bar: Integer, src_ref: SrcRef };`.
            Fields::Named(_) => {
                quote! {
                    impl microcad_lang_base::SrcReferrer for #name {
                        fn src_ref(&self) -> microcad_lang_base::SrcRef  {
                            self.src_ref
                        }
                    }
                }
            }
            // Generate SrcReferrer for a tuple `struct Bar(Refer<Identifier>);`.
            Fields::Unnamed(_) => {
                quote! {
                    impl microcad_lang_base::SrcReferrer for #name {
                        fn src_ref(&self) -> microcad_lang_base::SrcRef  {
                            self.0.src_ref()
                        }
                    }
                }
            }
            // Unit structs are not supported.
            Fields::Unit => {
                Error::new_spanned(name, "Unit structs are not supported").to_compile_error()
            }
        },
        _ => Error::new_spanned(name, "Unions and enums are not supported").to_compile_error(),
    }
    .into()
}

/// Derives the `Identifiable` trait for named structs.
///
/// This macro implements `id_ref()` by returning a reference to an `id` field.
/// The `id` field must be of type `crate::Identifier`.
///
/// # Constraints
/// - Only works on **Named Structs**.
/// - Does **not** support Tuple structs, Unit structs, Enums, or Unions.
#[proc_macro_derive(Identifiable)]
pub fn derive_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();

    match &input.data {
        Data::Struct(ds) => match &ds.fields {
            // Generate `Identifiable` for a struct with `id` field `struct Foo { bar: Integer, id: Identifier };`.
            Fields::Named(_) => {
                quote! {
                    impl microcad_lang_base::Identifiable for #name {
                        fn id_ref(&self) -> &microcad_lang_base::Identifier  {
                            &self.id
                        }
                    }
                }
            }
            Fields::Unnamed(_) => {
                Error::new_spanned(name, "Unnamed structs are not supported").to_compile_error()
            }
            // Unit structs not supported.
            Fields::Unit => {
                Error::new_spanned(name, "Unit structs are not supported").to_compile_error()
            }
        },
        _ => Error::new_spanned(name, "Unions and enums are not supported").to_compile_error(),
    }
    .into()
}

#[proc_macro_derive(Visit, attributes(visit))]
pub fn derive_visit(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    fn has_visit_default(attrs: &[syn::Attribute]) -> bool {
        attrs.iter().any(|attr| {
            attr.path().is_ident("visit")
                && attr
                    .parse_nested_meta(|meta| {
                        if meta.path.is_ident("default") {
                            Ok(())
                        } else {
                            Err(meta.error("unsupported container visit attribute"))
                        }
                    })
                    .is_ok()
        })
    }

    // Check if the container itself has the #[visit(default)] attribute
    if has_visit_default(&input.attrs) {
        return quote! {
            impl crate::ast::visitor::Visit for #name {
                fn visit<'ast, V>(&'ast self, _visitor: &mut V) -> std::ops::ControlFlow<V::BreakTy>
                    where V: crate::ast::visitor::Visitor<'ast> + ?Sized
                {
                    std::ops::ControlFlow::Continue(())
                }
            }
        }
        .into();
    }

    fn has_visit_skip(attrs: &[syn::Attribute]) -> bool {
        attrs.iter().any(|attr| {
            // Check if the attribute path is exactly "visit"
            attr.path().is_ident("visit") &&
                // Parse inside the parentheses: #[visit(...)]
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("skip") {
                        Ok(())
                    } else {
                        Err(meta.error("unsupported visit attribute"))
                    }
                }).is_ok()
        })
    }

    match input.data {
        Data::Struct(s) => match s.fields {
            // Named struct
            Fields::Named(f) => {
                // Generate code for each field
                let recurse = f.named.iter().map(|f| {
                    if !has_visit_skip(&f.attrs) {
                        let name = &f.ident;
                        // This assumes every field in your AST implements Visit
                        quote! {
                            self.#name.visit(visitor)?;
                        }
                    } else {
                        proc_macro2::TokenStream::new()
                    }
                });

                quote! {
                    impl crate::ast::visitor::Visit for #name {
                        fn visit<'ast, V>(&'ast self, visitor: &mut V) -> std::ops::ControlFlow<V::BreakTy>
                            where V: crate::ast::visitor::Visitor<'ast> + ?Sized
                        {
                            #(#recurse)*
                            std::ops::ControlFlow::Continue(())
                        }
                    }
                }
                .into()
            }
            _ => panic!("Only named fields supported"),
        },
        Data::Enum(e) => {
            let variants = e.variants.iter().map(|variant| {
                let variant_name = &variant.ident;

                // Scenario A: The entire variant is marked #[skip_visit]
                if has_visit_skip(&variant.attrs) {
                    return match &variant.fields {
                        Fields::Named(_) => quote! { #name::#variant_name { .. } => {} },
                        Fields::Unnamed(_) => quote! { #name::#variant_name(..) => {} },
                        Fields::Unit => quote! { #name::#variant_name => {} },
                    };
                }

                // Scenario B: Individual fields inside the variant might be marked #[skip_visit]
                match &variant.fields {
                    Fields::Named(fields) => {
                        // Create pattern bindings for ALL fields, but only generate visit calls for non-skipped ones
                        let idents = fields.named.iter().map(|f| &f.ident);
                        let visit_calls = fields
                            .named
                            .iter()
                            .filter(|f| !has_visit_skip(&f.attrs))
                            .map(|f| {
                                let name = &f.ident;
                                quote! { #name.visit(visitor)?; }
                            });

                        quote! {
                            #name::#variant_name { #(#idents),* } => {
                                #(#visit_calls)*
                            }
                        }
                    }
                    Fields::Unnamed(fields) => {
                        // Dynamically create temporary names for destructuring
                        let idents = fields
                            .unnamed
                            .iter()
                            .enumerate()
                            .map(|(i, _)| quote::format_ident!("field{}", i))
                            .collect::<Vec<_>>();

                        let visit_calls = fields
                            .unnamed
                            .iter()
                            .enumerate()
                            .filter(|(_, f)| !has_visit_skip(&f.attrs))
                            .map(|(i, _)| {
                                let field_ident = quote::format_ident!("field{}", i);
                                quote! { #field_ident.visit(visitor)?; }
                            });

                        quote! {
                            #name::#variant_name(#(#idents),*) => {
                                #(#visit_calls)*
                            }
                        }
                    }
                    Fields::Unit => {
                        quote! {
                            #name::#variant_name => {}
                        }
                    }
                }
            });

            quote! {
                impl crate::ast::visitor::Visit for #name {
                    fn visit<'ast, V>(&'ast self, visitor: &mut V) -> std::ops::ControlFlow<V::BreakTy>
                        where V: crate::ast::visitor::Visitor<'ast> + ?Sized
                    {
                        match self {
                            #(#variants)*
                        }
                        std::ops::ControlFlow::Continue(())
                    }
                }
            }
            .into()
        }
        _ => panic!("Only structs supported"),
    }
}

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(Scaffold)]
pub fn derive_scaffold(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Ensure we are deriving for a struct with named fields
    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("#[derive(Scaffold)] only supports structs with named fields"),
        },
        _ => panic!("#[derive(Scaffold)] only supports structs"),
    };

    let field_idents: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();

    quote! {
        impl crate::Scaffold for #name {
            fn scaffold(self, context: &mut crate::LowerContext) -> ir::IrNodeId {
                #(
                    self.#field_idents.scaffold(context);
                )*
                *context.top_node()
            }
        }
    }
    .into()
}

#[proc_macro_derive(Artifact)]
pub fn derive_artifact(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    quote! {
        impl microcad_lang_base::Artifact for #name {
            fn kind() -> microcad_lang_base::ArtifactKind {
                microcad_lang_base::ArtifactKind::#name
            }
        }
    }
    .into()
}
