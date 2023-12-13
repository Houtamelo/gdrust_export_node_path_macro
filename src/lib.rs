mod compiler;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{parse_quote, ItemStruct, Result, Type, Path};

pub(crate) enum Extends {
    Type(Type),
    Path(Path),
}

impl Parse for Extends {
    fn parse(input: ParseStream) -> Result<Self> {
        if let Ok(var_type) = input.parse() {
            return Ok(Self::Type(var_type));
        }
        
        if let Ok(var_path) = input.parse() {
            return Ok(Self::Path(var_path));
        }
        
        return Err(input.error(format!("Expected type or path, got: {input}")));
    }
}

impl ToTokens for Extends {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Type(ty) => ty.to_tokens(tokens),
            Self::Path(path) => path.to_tokens(tokens),
        }
    }
}

#[proc_macro_attribute]
pub fn extends(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut parsed = syn::parse_macro_input!(item as ItemStruct);
    let extends = syn::parse_macro_input::parse::<Extends>(attr).unwrap_or(Extends::Type(parse_quote! { gdnative::api::Object }));
    let compiled = compiler::compile(&mut parsed, &extends);
    return compiled.into();
}
