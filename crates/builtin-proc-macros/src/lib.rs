// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

extern crate proc_macro;
use proc_macro::TokenStream;

use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, *};

/// Build parameter list entries: for each field, generate `parameter!(<field_name>: <Type>)`
fn generate_parameters(
    fields: &punctuated::Punctuated<Field, token::Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        quote_spanned!(field.span() =>
            microcad_lang::builtin::parameter!( #ident : #ty )
        )
    })
}

/// Build argument list entries: for each field, generate `<field_name>: args.get("<field_name>")`
fn generate_arguments(
    fields: &punctuated::Punctuated<Field, token::Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        quote_spanned!(field.span() =>
            #ident: args.get(stringify!(#ident))
        )
    })
}

fn get_doc_comment(attrs: &[Attribute]) -> String {
    attrs.iter().filter_map(|attr| 
        // Parse the meta of the attribute
        if attr.path().is_ident("doc") 
            && let Meta::NameValue(nv) = &attr.meta
            && let Expr::Lit(ExprLit{ lit: Lit::Str(lit_str), ..}) = &nv.value {
            // Return the string value, e.g., "Doc test"
            Some(String::from(lit_str.value().trim()))
        } else {
            None
        }
    ).collect::<Vec<_>>().join("\n")
}

fn generate_help(input: &DeriveInput) -> String {
    fn type_to_string(ty: &Type) -> String {
        quote!(#ty).to_string()
    }

    let mut help = get_doc_comment(&input.attrs);
    let args = match &input.data {
        Data::Struct(ds) => match &ds.fields {
            Fields::Named(fields_named) => {
                help += "\n\n# Arguments\n";
                fields_named.named.iter().map(|field| 
                    format!("- `{name}: {ty}`: {doc}", 
                        name = field.ident.as_ref().expect("Ident"),
                        ty = type_to_string(&field.ty),
                        doc = get_doc_comment(&field.attrs)
                    )
                ).collect()
            },
            _ => vec![]
        }
        _ => vec![]
    }.join("\n");
    help += args.as_str();
    
    help
}

fn derive_workbench_definition(
    input: TokenStream,
    kind: &'static str,
    output_type: &'static str,
) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();

    use convert_case::{Casing, Case};

    // Operations are lower case.
    let id = if kind == "Operation" {
        syn::Ident::new(
            name.to_string().to_case(Case::Snake).as_str(),
            proc_macro2::Span::call_site(),
        )
    } else {
        name.clone()
    };

    let kind = Ident::new(kind, proc_macro2::Span::call_site());
    let output_type = Ident::new(output_type, proc_macro2::Span::call_site());
    let help = generate_help(&input);
    let help = help.as_str();

    // parse fields, validate etc
    // Only support structs with named fields
    match &input.data {
        Data::Struct(ds) => match &ds.fields {
            // Generate BuiltinWorkbenchDefinition for a struct with fields `struct Foo { bar: Integer, baz: Scalar };`.
            Fields::Named(named) => {
                let fields = &named.named;
                let (parameters, arguments) =
                    (generate_parameters(fields), generate_arguments(fields));

                quote! {
                    impl microcad_lang::builtin::BuiltinWorkbenchDefinition for #name {
                        fn id() -> &'static str {
                            stringify!(#id)
                        }

                        fn help() -> Option<&'static str> {
                            Some(#help)
                        }

                        fn output_type() -> microcad_lang::model::OutputType {
                            microcad_lang::model::OutputType::#output_type
                        }

                        fn kind() -> microcad_lang::builtin::BuiltinWorkbenchKind {
                            microcad_lang::builtin::BuiltinWorkbenchKind::#kind
                        }

                        fn workpiece_function() -> &'static microcad_lang::builtin::BuiltinWorkpieceFn {
                            &|args| {
                                Ok(microcad_lang::builtin::BuiltinWorkpieceOutput::#kind(Box::new(#name {
                                    #(#arguments),*
                                })))
                            }
                        }
                        fn parameters() -> microcad_lang::value::ParameterValueList {
                            [
                                #(#parameters),*
                            ]
                            .into_iter()
                            .collect()
                        }
                    }
                }
            }
            // Generate BuiltinWorkbenchDefinition for a unit struct `struct Foo;`.
            Fields::Unit => {
                quote! {
                    impl microcad_lang::builtin::BuiltinWorkbenchDefinition for #name {
                        fn id() -> &'static str {
                            stringify!(#id)
                        }

                        fn help() -> Option<&'static str> {
                            Some(#help)
                        }


                        fn output_type() -> microcad_lang::model::OutputType {
                            microcad_lang::model::OutputType::#output_type
                        }

                        fn kind() -> microcad_lang::builtin::BuiltinWorkbenchKind {
                            microcad_lang::builtin::BuiltinWorkbenchKind::#kind
                        }

                        fn workpiece_function() -> &'static microcad_lang::builtin::BuiltinWorkpieceFn {
                            &|_| {
                                Ok(microcad_lang::builtin::BuiltinWorkpieceOutput::#kind(Box::new(#name)))
                            }
                        }
                    }
                }
            }
            // Enum structs are not supported.
            _ => {
                return Error::new_spanned(
                    &name,
                    format!("{kind} macro can only be derived for structs with named fields or unit structs"),
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return Error::new_spanned(
                &name,
                format!("{kind} macro can only be derived for structs"),
            )
            .to_compile_error()
            .into();
        }
    }.into()
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
