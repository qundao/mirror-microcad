extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Primitive2D)]
pub fn derive_primitive2d(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // parse fields, validate etc
    // Only support structs with named fields
    let fields = match &input.data {
        syn::Data::Struct(ds) => match &ds.fields {
            syn::Fields::Named(named) => &named.named,
            _ => {
                return syn::Error::new_spanned(
                    &input.ident,
                    "Primitive2D can only be derived for structs with named fields",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(
                &input.ident,
                "Primitive2D can only be derived for structs",
            )
            .to_compile_error()
            .into();
        }
    };

    // Build parameter list entries: for each field, generate `parameter!(<field_name>: <Type>)`
    let parameters = fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        quote! {
            parameter!( #ident : #ty )
        }
    });

    // Build argument list entries: for each field, generate `<field_name>: args.get("<field_name>")`
    let arguments = fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        quote! {
            #ident: args.get(stringify!(#ident))
        }
    });

    let expanded = quote! {
        impl microcad_lang::builtin::BuiltinWorkbenchDefinition for #name {
            fn id() -> &'static str {
                stringify!(#name)
            }
            fn kind() -> microcad_lang::builtin::BuiltinWorkbenchKind {
                microcad_lang::builtin::BuiltinWorkbenchKind::Primitive2D
            }
            fn workpiece_function() -> &'static BuiltinWorkpieceFn {
                &|args| {
                    Ok(microcad_lang::builtin::BuiltinWorkpieceOutput::Primitive2D(Box::new(#name {
                        #(#arguments),*
                    })))
                }
            }
            fn parameters() -> ParameterValueList {
                [
                    #(#parameters),*
                ]
                .into_iter()
                .collect()
            }
        }
    };
    TokenStream::from(expanded)
}
