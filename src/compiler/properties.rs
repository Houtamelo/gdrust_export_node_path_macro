use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream, Result};
use syn::parse_macro_input::parse;
use syn::{parenthesized, token, Expr, Field, ItemStruct, Type};

mod kw {
    syn::custom_keyword!(export_node_path);
}

pub struct Property {
    pub name: Ident,
    pub var_type: Type,
    pub should_export_path: bool,
    pub default: Option<Expr>,
}

impl Property {
    fn new(name: Ident, var_type: Type) -> Self {
        Self {
            name,
            var_type,
            should_export_path: false,
            default: None,
        }
    }
}

struct DefaultProperty {
    pub paren_token: token::Paren,
    pub expr: Expr,
}

impl Parse for DefaultProperty {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let paren_token = parenthesized!(content in input);
        let expr = content.parse()?;
        Ok(Self { paren_token, expr })
    }
}

#[allow(clippy::module_name_repetitions)]
pub fn extract_properties(item: &mut ItemStruct) -> Vec<Property> {
    return item.fields.iter_mut().map(|x| get_property(x)).collect();
}

pub fn get_property(item: &mut Field) -> Property {
    let mut property = Property::new(item.ident.as_ref().expect("Properties must be on named field").clone(), item.ty.clone());

    item.attrs = item.attrs.iter()
            .filter(|x| {
                let ident = x.path.get_ident().expect("Expected valid attr on property").to_string();
                let tokens = x.tokens.clone().into();

                match ident.as_str() {
                    "default" => {
                        let default = parse::<DefaultProperty>(tokens).expect("Invalid params for default").expr;
                        property.default = Some(default);
                        return false;
                    }
                    "export_node_path" => {
                        property.should_export_path = true;
                        return false;
                    },
                    _ => return true,
                }
            }).cloned().collect();

    return property;
}