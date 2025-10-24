extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Primitive2D)]
pub fn derive_primitive2d(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // parse fields, validate etc

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
                        radius: args.get("radius"),
                        start_angle: args.get("start_angle"),
                        end_angle: args.get("end_angle"),
                    })))
                }
            }
            fn parameters() -> ParameterValueList {
                [
                    microcad_lang::builtin::parameter!(radius: Scalar),
                    microcad_lang::builtin::parameter!(start_angle: Angle),
                    microcad_lang::builtin::parameter!(end_angle: Angle),
                ]
                .into_iter()
                .collect()
            }
        }
    };
    TokenStream::from(expanded)
}
