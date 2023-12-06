extern crate core;

mod json_schema;

use proc_macro::TokenStream;

use json_schema::{get_string_literal, StructsTemplate};

#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    let filename = get_string_literal(input).unwrap();
    let result = StructsTemplate::render(&filename).unwrap();
    println!("{}", result);
    result.parse().unwrap()
}