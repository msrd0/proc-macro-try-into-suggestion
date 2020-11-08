use proc_macro2::TokenStream;
use syn::{spanned::Spanned, DeriveInput, Error, Expr, Lit, Meta, Path, Result};

pub(crate) trait PathEndsWith {
	fn ends_with(&self, s: &str) -> bool;
}

impl PathEndsWith for Path {
	fn ends_with(&self, s: &str) -> bool {
		self.segments.last().map(|segment| segment.ident.to_string()).as_deref() == Some(s)
	}
}

pub(super) fn expand(input: DeriveInput) -> Result<TokenStream> {
	let name = &input.ident;
	let (impl_gen, ty_gen, were) = input.generics.split_for_impl();

	let mut expr: Option<TokenStream> = None;
	for attr in input.attrs {
		let meta = attr.parse_meta()?;
		if !meta.path().ends_with("custom") {
			continue;
		}
		let kv = match meta {
			Meta::NameValue(kv) => kv,
			_ => return Err(Error::new(meta.span(), "invalid syntax"))
		};

		let expr_span = kv.lit.span();
		let expr_str = match kv.lit {
			Lit::Str(str) => str,
			_ => return Err(Error::new(expr_span, "invalid syntax"))
		};
		let expr_parsed: Expr = syn::parse_str(&expr_str.value())?;
		expr = Some(quote_spanned!(expr_span => #expr_parsed));
	}
	let expr = expr.unwrap();

	Ok(quote! {
		impl #impl_gen #name #ty_gen #were {
			fn thisdoesnothing() {
				let value: usize = #expr;
			}
		}
	})
}
