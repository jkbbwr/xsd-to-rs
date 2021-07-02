use quote::quote;
use proc_macro::TokenStream;
use structmeta::StructMeta;
use syn::{parse_macro_input, DeriveInput, LitStr};
use xsd::parse_xsd;
use std::iter::FromIterator;
use proc_macro2::TokenStream as TokenStream2;

mod xsd;

#[derive(StructMeta, Debug)]
struct XsdToRs {
    path: LitStr,
}

#[proc_macro_derive(Schema, attributes(xsd_to_rs))]
pub fn derive_xsd(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let attributes = input
        .attrs
        .first()
        .expect("Expected xsd_to_rs attribute")
        .parse_args::<XsdToRs>()
        .expect("Failed to parse attribute")
        .path
        .value();

    let schema = parse_xsd(&attributes);

    let simple_types_token_streak = schema.simple_types.into_iter().map(|t| TokenStream::from(t));
    let complex_types_token_stream = schema.complex_types.into_iter().map(|t| TokenStream::from(t));
    let token_stream_chain = simple_types_token_streak.chain(complex_types_token_stream);

    TokenStream::from_iter(
        token_stream_chain
    )
}
