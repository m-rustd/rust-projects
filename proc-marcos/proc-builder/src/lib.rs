mod builder;
mod builder_with_attr;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    builder::BuilderContext::from(input)
        .render()
        .into()
}

#[proc_macro_derive(BuilderWithAttr, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    builder_with_attr::BuilderContext::from(input)
        .render()
        .into()
}