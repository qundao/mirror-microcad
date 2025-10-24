extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{DeriveInput, Field, parse_macro_input, punctuated::Punctuated, token::Comma};

/// Build parameter list entries: for each field, generate `parameter!(<field_name>: <Type>)`
fn generate_parameters(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        quote! {
            microcad_lang::builtin::parameter!( #ident : #ty )
        }
    })
}

/// Build argument list entries: for each field, generate `<field_name>: args.get("<field_name>")`
fn generate_arguments(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        quote! {
            #ident: args.get(stringify!(#ident))
        }
    })
}

fn derive_workbench_definition(
    input: TokenStream,
    kind: &'static str,
    output_type: &'static str,
) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Operations are lower case.
    let id = if kind == "Operation" {
        syn::Ident::new(
            name.to_string().to_lowercase().as_str(),
            proc_macro2::Span::call_site(),
        )
    } else {
        name.clone()
    };

    let kind = syn::Ident::new(kind, proc_macro2::Span::call_site());
    let output_type = syn::Ident::new(output_type, proc_macro2::Span::call_site());

    // parse fields, validate etc
    // Only support structs with named fields
    let expanded = match &input.data {
        syn::Data::Struct(ds) => match &ds.fields {
            // Generate BuiltinWorkbenchDefinition for a struct with fields `struct Foo { bar: Integer, baz: Scalar };`.
            syn::Fields::Named(named) => {
                let fields = &named.named;
                let (parameters, arguments) =
                    (generate_parameters(fields), generate_arguments(fields));

                quote! {
                    impl microcad_lang::builtin::BuiltinWorkbenchDefinition for #name {
                        fn id() -> &'static str {
                            stringify!(#id)
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
            syn::Fields::Unit => {
                quote! {
                        impl microcad_lang::builtin::BuiltinWorkbenchDefinition for #name {
                            fn id() -> &'static str {
                                stringify!(#id)
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
                return syn::Error::new_spanned(
                    &name,
                    format!("{kind} macro can only be derived for structs with named fields or unit structs"),
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(
                &name,
                format!("{kind} macro can only be derived for structs"),
            )
            .to_compile_error()
            .into();
        }
    };

    TokenStream::from(expanded)
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
