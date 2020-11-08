#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, DeriveInput, Result};

mod derive_macro;

fn expand_derive<F>(input: TokenStream, expand: F) -> TokenStream
where
	F: FnOnce(DeriveInput) -> Result<TokenStream2>
{
	expand(parse_macro_input!(input))
		.unwrap_or_else(|err| err.to_compile_error())
		.into()
}

#[proc_macro_derive(DeriveMacro, attributes(custom))]
pub fn derive_macro(input: TokenStream) -> TokenStream {
	expand_derive(input, derive_macro::expand)
}
