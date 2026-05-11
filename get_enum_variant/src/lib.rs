extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(GetEnumVariant)]
pub fn derive_get_enum_variant(input: TokenStream) -> TokenStream {
    let syn_input: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &syn_input.ident;
    let variants = match syn_input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("#[derive(GetEnumVariant)] can only be used with enums"),
    };
    let len = variants.len();
    let mut match_expr = format!("match rng.random_range(0..{} ) {}", len, "{");
    for (i, variant) in variants.iter().enumerate() {
        let arm = format!("{} => {}::{},", i, name, variant.ident);
        match_expr.push_str(arm.as_str());
    }
    let default = format!("_ => {}::{}, {}", name, variants[0].ident, "}");
    match_expr.push_str(default.as_str());

    let output = format!("impl get_enum_variant_trait::GetEnumVariant for {} {}
        fn get_enum_variant(mut rng: rand::prelude::ThreadRng) -> {} {}
            {} {}",
                         name, "{", name, " {", match_expr, "}}");

    output.parse::<TokenStream>().unwrap()
}