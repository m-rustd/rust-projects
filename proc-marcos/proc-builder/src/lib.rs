mod builder;
mod builder_with_attr;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(myderive))]
struct MyDeriveInput {
    name: Option<String>,
    age: Option<u32>,
}

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

#[proc_macro_derive(DarlingDerive, attributes(myderive))]
pub fn darling_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let res = MyDeriveInput::from_derive_input(&input).unwrap();
    println!("res======={:#?}", res);
    TokenStream::default()
}