mod impl_block;
mod properties;

use crate::compiler::properties::{ExportType, extract_properties};
use crate::Extends;
use proc_macro2::TokenStream;
use syn::{parse_quote, ItemStruct, Field, Visibility, Type, Fields};

pub(crate) fn compile(item: &mut ItemStruct, extends: &Extends) -> TokenStream {
    let mut properties = extract_properties(item);
    let inherit_type = &extends.inherit_type;

    let Fields::Named(fields_named) = &mut item.fields else { panic!("Expected named fields") };
    let path_fields : Vec<(Field, Type, ExportType)> = properties.iter_mut()
            .filter(|property| property.export_type != ExportType::DoNotExport)
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

                let source_type = property.var_type.clone();
                let extracted_type : Type = match property.export_type {
                    ExportType::ExportBuiltIn => {
                        let type_string = (quote::quote! { #source_type }).to_string();
                        assert!(type_string.starts_with("Option<Ref<") && type_string.ends_with(">>"));
                        let type_string = type_string.replace("Option<Ref<", "").replace(">>", "");
                        parse_quote! { #type_string }
                    },
                    ExportType::ExportUserScript => {
                        let type_string = (quote::quote! { #source_type }).to_string();
                        assert!(type_string.starts_with("Option<Instance<") && type_string.ends_with(">>"));
                        let type_string = type_string.replace("Option<Instance<", "").replace(">>", "");
                        parse_quote! { #type_string }
                    }
                    ExportType::DoNotExport => unreachable!(),
                };

                return (field, extracted_type, property.export_type);
            }).collect();

    item.attrs.push(parse_quote! { #[derive(gdnative::NativeClass, Default)] });
    item.attrs.push(parse_quote! { #[inherit(#inherit_type)]});

    let impl_block = impl_block::impl_block(&path_fields, &properties, &inherit_type, item);
    quote::quote! {
        #item

        #impl_block
    }
}
