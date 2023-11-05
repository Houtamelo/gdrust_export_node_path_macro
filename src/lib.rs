use proc_macro::TokenStream;
mod compiler;

use syn::parse::{Parse, ParseStream};
use syn::{parse_quote, ItemStruct, Result, Token, Type};

mod kw {
    syn::custom_keyword!(extends);
}

pub(crate) struct Extends {
    inherit_type: Type,
}

impl Parse for Extends {
    fn parse(input: ParseStream) -> Result<Self> {
        let _extends = input.parse::<kw::extends>()?;
        let _eq = input.parse::<Token![=]>()?;
        let var_type = input.parse()?;
        Ok(Self { inherit_type: var_type })
    }
}

#[proc_macro_attribute]
pub fn gdrust(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut parsed = syn::parse_macro_input!(item as ItemStruct);
    let extends = syn::parse_macro_input::parse::<Extends>(attr).unwrap_or(Extends { inherit_type: parse_quote! { gdnative::api::Object }, });
    let compiled = compiler::compile(&mut parsed, &extends);
    return compiled.into();
}
