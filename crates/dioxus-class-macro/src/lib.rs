#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

#[cfg(feature = "build-classes")]
mod build;

mod class;

/// The arguments are list of expressions, separated by space, all of them need to be compatible with
/// String::from()
///
/// The idea is to write in a very similar way of CSS, but can be compile time checked, also can provide
/// code completion nicely.
///
/// # Example
/// ```rust
/// class: class!(card card_compact w_64 h_64 bg_base_300 shadow_xl text_center hover(bg_base_200) hover(scale_105)),
/// ```
#[proc_macro]
pub fn class(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as class::Dsl)
        .into_token_stream()
        .into()
}
