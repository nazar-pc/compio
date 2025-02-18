use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse::Parse, AttrStyle, Attribute, Signature, Visibility};

use crate::{
    item_fn::{RawAttr, RawBodyItemFn},
    retrieve_runtime_mod,
};

pub(crate) struct CompioMain(pub RawBodyItemFn);

impl CompioMain {
    pub fn with_args(mut self, args: RawAttr) -> Self {
        self.0.set_args(args.inner_attrs);
        self
    }
}

impl Parse for CompioMain {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        let mut sig: Signature = input.parse()?;
        let body: TokenStream = input.parse()?;

        if sig.asyncness.is_none() {
            return Err(syn::Error::new_spanned(
                sig.ident,
                "the `async` keyword is missing from the function declaration",
            ));
        };
        if sig.ident != Ident::new("main", Span::call_site()) {
            return Err(syn::Error::new_spanned(
                sig.ident,
                "`compio::main` can only be used for main function.",
            ));
        }

        sig.asyncness.take();
        Ok(Self(RawBodyItemFn::new(attrs, vis, sig, body)))
    }
}

impl ToTokens for CompioMain {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(
            self.0
                .attrs
                .iter()
                .filter(|a| matches!(a.style, AttrStyle::Outer)),
        );
        self.0.vis.to_tokens(tokens);
        self.0.sig.to_tokens(tokens);
        let block = &self.0.body;
        let runtime_mod = self.0.crate_name().unwrap_or_else(retrieve_runtime_mod);
        tokens.append_all(quote!({
            #runtime_mod::block_on(async move #block)
        }));
    }
}
