use crate::compiler::properties::{Property};
use proc_macro2::{TokenStream};
use quote::quote;
use syn::{Expr, Field, ItemStruct, Type};

pub(crate) fn impl_block(path_fields: &Vec<(Field, Type)>, properties: &[Property], inherit_type: &Type, item: &ItemStruct) -> TokenStream {
    let struct_name = &item.ident;

    let property_inits = property_inits(properties);
    let grab_nodes_by_path = grab_nodes_by_path(path_fields);

    return quote! {
        impl #struct_name {
            #[allow(clippy::default_trait_access)]
            fn new(_owner: &#inherit_type) -> Self {
                Self {
                    #(#property_inits,)*
                }
            }

            fn grab_nodes_by_path(&mut self, owner: &#inherit_type) {
                #(#grab_nodes_by_path)*
            }
        }
    };
}

fn grab_nodes_by_path(path_fields: &Vec<(Field, Type)>) -> Vec<TokenStream> {
    return path_fields.iter()
            .map(|(field, source_type)| {
                let path_field_name = field.ident.as_ref().unwrap().to_string();
                let source_name = path_field_name.replace("path_", "");
                return quote! {
                    self.#source_name = Some(unsafe { owner.get_node_as::<gdnative::prelude::#source_type>(self.#path_field_name.new_ref()).unwrap().assume_shared() });
                }
            }).collect();
}

fn property_inits(properties: &[Property]) -> Vec<TokenStream> {
    return properties.iter()
            .map(|property| {
                let ident = &property.name;
                let default = get_default(property.default.as_ref());
                quote! { #ident: #default }
            }).collect();
}

fn get_default(default: Option<&Expr>) -> TokenStream {
    return if let Some(default) = default {
        quote! { #default }
    } else {
        quote! {
            Default::default()
        }
    };
}