mod impl_block;
mod properties;

use quote::quote;

use crate::compiler::properties::{ExportType, extract_properties};
use crate::Extends;
use proc_macro2::TokenStream;
use syn::{parse_quote, ItemStruct, Field, Visibility, Type, Fields, parse_str};

pub(crate) fn compile(item: &mut ItemStruct, extends: &Extends) -> TokenStream {
    let mut properties = extract_properties(item);

    let Fields::Named(fields_named) = &mut item.fields else { panic!("Expected named fields") };
    let path_fields : Vec<(Field, Type, ExportType)> = properties.iter_mut()
            .filter(|property| property.export_type != ExportType::DoNotExport)
            .map(|property| {
                let source_type = property.var_type.clone();
                let Some((extracted_type, _)) = properties::get_field_type(&source_type)
                        else {
                            let source_type_string = quote!(#source_type).to_string();
                            panic!("Failed to extract type from {source_type_string}");
                        };

                let path_type = match property.export_type {
                    ExportType::DoNotExport       => unreachable!(),
                    ExportType::ExportNode        => parse_str::<Type>("gdnative::prelude::NodePath"),
                    ExportType::ExportInstance    => parse_str::<Type>("gdnative::prelude::NodePath"),
                    ExportType::ExportNodeVec     => parse_str::<Type>("Vec<gdnative::prelude::NodePath>"),
                    ExportType::ExportInstanceVec => parse_str::<Type>("Vec<gdnative::prelude::NodePath>"),
                }.expect("Failed to parse path type");

                let field_name = format!("path_{}", property.name);
                let field = Field {
                    attrs: vec![parse_quote! { #[property] }],
                    vis: Visibility::Inherited,
                    ident: Some(proc_macro2::Ident::new(&field_name, proc_macro2::Span::call_site())),
                    colon_token: Some(syn::token::Colon::default()),
                    ty: path_type,
                };

                fields_named.named.insert(0, field.clone());

                return (field, extracted_type, property.export_type);
            }).collect();

    item.attrs.insert(0,parse_quote! { #[inherit(#extends)]});
    item.attrs.insert(0, parse_quote! { #[derive(gdnative::prelude::NativeClass, Default)] });

    let impl_block = impl_block::impl_block(&path_fields, extends, item);

    return  quote! {
        #item

        #impl_block
    };
}
