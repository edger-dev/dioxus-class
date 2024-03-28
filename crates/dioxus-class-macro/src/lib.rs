use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

#[cfg(feature = "build-classes")]
mod build;

mod class;

#[proc_macro]
pub fn class(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as class::Dsl)
        .into_token_stream()
        .into()
}