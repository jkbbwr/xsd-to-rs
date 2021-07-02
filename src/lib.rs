mod xsd;

use proc_macro::TokenStream;
use structmeta::StructMeta;
use syn::{parse_macro_input, DeriveInput, LitStr};

#[derive(StructMeta, Debug)]
struct XsdToRs {
    path: LitStr,
}

#[proc_macro_derive(Schema, attributes(xsd_to_rs))]
pub fn derive_xsd(input: TokenStream) -> TokenStream {
    let attributes = parse_macro_input!(input as DeriveInput)
        .attrs
        .first()
        .expect("Expected xsd_to_rs attribute")
        .parse_args::<XsdToRs>()
        .expect("Failed to parse attribute")
        .path
        .value();

    let schema = xsd::parse_xsd(&attributes);
    let simple_types_token_streak = schema.simple_types.into_iter().map(TokenStream::from);
    let complex_types_token_stream = schema.complex_types.into_iter().map(TokenStream::from);

    simple_types_token_streak
        .chain(complex_types_token_stream)
        .collect()
}
