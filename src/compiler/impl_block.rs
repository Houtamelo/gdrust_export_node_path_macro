
use crate::compiler::properties::{ExportType};
use proc_macro2::{TokenStream};
use quote::{quote};
use syn::{Field, ItemStruct, parse_quote, parse_str, Type};

pub(crate) fn impl_block(path_fields: &Vec<(Field, Type, ExportType)>, inherit_type: &Type, item: &ItemStruct) -> TokenStream {
    let struct_name = &item.ident;

    let grab_nodes_by_path = grab_nodes_by_path(path_fields);

    return quote! {
        impl #struct_name {
            fn new(_owner: &#inherit_type) -> Self {
                Self::default()
            }

            fn grab_nodes_by_path(&mut self, owner: &#inherit_type) {
                #(#grab_nodes_by_path)*
            }
        }
    };
}

fn grab_nodes_by_path(path_fields: &Vec<(Field, Type, ExportType)>) -> Vec<TokenStream> {
    return path_fields.iter()
            .map(|(field, source_type, export_type)| {
                let path_field_name = field.ident.as_ref().expect(std::panic::Location::caller().to_string().as_str()).to_string();
                let source_name = path_field_name.replace("path_", "");
                let source_type_ident : syn::Ident = parse_quote!(#source_type);

                return match export_type {
                    ExportType::DoNotExport => unreachable!(),
                    ExportType::ExportBuiltIn    => {
                        let result = format!("self.{source_name} = Some(unsafe {{ owner.get_node_as::<gdnative::prelude::{source_type_ident}>(self.{path_field_name}.new_ref()).unwrap().assume_shared() }});");
                        parse_str(result.as_str()).expect("Failed to parse result: {result}")
                    },
                    ExportType::ExportUserScript => {
                        let result = format!("self.{source_name} = Some(unsafe {{ owner.get_node_as_instance::<{source_type_ident}>(self.{path_field_name}.new_ref()).unwrap().claim() }});");
                        parse_str(result.as_str()).expect("Failed to parse result: {result}")
                    },
                };
            }).collect();
}
