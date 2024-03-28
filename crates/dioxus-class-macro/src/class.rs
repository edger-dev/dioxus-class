use fehler::{throw, throws};
use quote::{quote, ToTokens};
use proc_macro2::TokenStream;
use syn::parse::{Error, Parse, ParseStream};
use syn::{Expr, Ident, Token};
use proc_macro2::Span;

use dioxus_class_internal::Class;

pub struct Dsl {
    pub span: Span,
    pub text: String,
    pub values: Vec<Expr>,
}

impl Parse for Dsl {
    #[throws(Error)]
    fn parse(input: ParseStream) -> Self {
        let span = Span::call_site();
        let text = match span.source_text() {
            Some(text) => text,
            None => input.to_string(),
        };
        let mut values = vec![];
        loop {
            if input.is_empty() {
                break;
            }
            let val = input.parse::<Expr>()?;
            values.push(val);
        }
        Dsl {
            span,
            text,
            values,
        }
    }
}

impl ToTokens for Dsl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let values_quote: Vec<_> = self.values.iter().map(
            |x| quote! { String::from(#x) }
        ).collect();
        let code = quote! {
            Class::from(vec![
                #(#values_quote),*
            ])
        };
        #[cfg(feature = "build-classes")]
        {
            // Span.source_file() is not stable yet
            let lines = format!("/* {:?}\n{}\n */\n{},\n\n",
                self.span, self.text, code);
            let _ = crate::build::write_text(&lines);
        }
        tokens.extend(code);
    }
}