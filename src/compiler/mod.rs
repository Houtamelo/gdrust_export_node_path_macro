mod impl_block;
mod properties;

use crate::compiler::properties::extract_properties;
use crate::Extends;
use proc_macro2::TokenStream;
use syn::{parse_quote, ItemStruct, Field, Visibility, Type, Fields};

pub(crate) fn compile(item: &mut ItemStruct, extends: &Extends) -> TokenStream {
    let mut properties = extract_properties(item);
    let inherit_type = &extends.inherit_type;

    let Fields::Named(fields_named) = &mut item.fields else { panic!("Expected named fields") };
    let path_fields : Vec<(Field, Type)> = properties.iter_mut().filter(|property| property.should_export_path)
            .map(|property| {
                let field_name = format!("path_{}", property.name);
                let field = Field {
                    attrs: vec![parse_quote! { #[export] }],
                    vis: Visibility::Inherited,
                    ident: Some(parse_quote! { #field_name }),
                    colon_token: Some(syn::token::Colon::default()),
                    ty: Type::Path(parse_quote! { gdnative::api::NodePath }),
                };

                fields_named.named.insert(0, field.clone());

                return (field, property.var_type.clone());
            }).collect();

    item.attrs.push(parse_quote! { #[derive(gdnative::NativeClass, Default)] });
    item.attrs.push(parse_quote! { #[inherit(#inherit_type)]});

    let impl_block = impl_block::impl_block(&path_fields, &properties, &inherit_type, item);
    quote::quote! {
        #item

        #impl_block
    }
}
