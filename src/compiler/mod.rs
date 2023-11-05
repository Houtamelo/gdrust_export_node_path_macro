mod impl_block;
mod properties;

use crate::compiler::properties::{ExportType, extract_properties};
use crate::Extends;
use proc_macro2::TokenStream;
use syn::{parse_quote, ItemStruct, Field, Visibility, Type, Fields, parse_str};

pub(crate) fn compile(item: &mut ItemStruct, extends: &Extends) -> TokenStream {
    let mut properties = extract_properties(item);
    let inherit_type = &extends.inherit_type;

    let Fields::Named(fields_named) = &mut item.fields else { panic!("Expected named fields") };
    let path_fields : Vec<(Field, Type, ExportType)> = properties.iter_mut()
            .filter(|property| property.export_type != ExportType::DoNotExport)
            .map(|property| {
                let field_name = format!("path_{}", property.name);
                let field = Field {
                    attrs: vec![parse_quote! { #[property] }],
                    vis: Visibility::Inherited,
                    ident: Some(proc_macro2::Ident::new(&field_name, proc_macro2::Span::call_site())),
                    colon_token: Some(syn::token::Colon::default()),
                    ty: Type::Path(parse_quote! { gdnative::prelude::NodePath }),
                };

                fields_named.named.insert(0, field.clone());

                let source_type = property.var_type.clone();
                let extracted_type : Type = match property.export_type {
                    ExportType::ExportBuiltIn => {
                        // we expect the type to be Option<Ref<...>> but there could be whitespace between the tokens, but just arbitrarily removing whitespace could lead to issues
                        let type_string = (quote::quote! { #source_type }).to_string();

                        let mut extracted_index_start = 0;
                        let mut extracted_index_end = 0;

                        let expected_prefix = "Option<Ref<";
                        let mut expected_index = 0;
                        for char in type_string.chars() {
                            extracted_index_start += 1;
                            if char.is_whitespace() {
                                continue;
                            }

                            assert_eq!(char, expected_prefix.chars().nth(expected_index).expect("Expected prefix to be at least as long as the expected index"));
                            expected_index += 1;

                            if expected_index >= expected_prefix.len() {
                                break;
                            }
                        }

                        let expected_suffix = ">>";
                        let mut expected_index = 0;

                        for char in type_string.chars().rev() {
                            extracted_index_end += 1;

                            if char.is_whitespace() {
                                continue;
                            }

                            assert_eq!(char, expected_suffix.chars().nth_back(expected_index).expect("Expected suffix to be at least as long as the expected index"));
                            expected_index += 1;

                            if expected_index >= expected_suffix.len() {
                                break;
                            }
                        }

                        let extracted_type_string = &type_string[extracted_index_start..(type_string.len() - extracted_index_end)];
                        parse_str::<Type>(extracted_type_string).unwrap()
                    },
                    ExportType::ExportUserScript => {
                        // we expect the type to be Option<Instance<...>> but there could be whitespace between the tokens, but just arbitrarily removing whitespace could lead to issues
                        let type_string = (quote::quote! { #source_type }).to_string();

                        let mut extracted_index_start = 0;
                        let mut extracted_index_end = 0;

                        let expected_prefix = "Option<Instance<";
                        let mut expected_index = 0;
                        for char in type_string.chars() {
                            extracted_index_start += 1;
                            if char.is_whitespace() {
                                continue;
                            }

                            assert_eq!(char, expected_prefix.chars().nth(expected_index).expect("Expected prefix to be at least as long as the expected index"));
                            expected_index += 1;

                            if expected_index >= expected_prefix.len() {
                                break;
                            }
                        }

                        let expected_suffix = ">>";
                        let mut expected_index = 0;

                        for char in type_string.chars().rev() {
                            extracted_index_end += 1;

                            if char.is_whitespace() {
                                continue;
                            }

                            assert_eq!(char, expected_suffix.chars().nth_back(expected_index).expect("Expected suffix to be at least as long as the expected index"));
                            expected_index += 1;

                            if expected_index >= expected_suffix.len() {
                                break;
                            }
                        }

                        let extracted_type_string = &type_string[extracted_index_start..(type_string.len() - extracted_index_end)];
                        parse_str::<Type>(extracted_type_string).unwrap()
                    }
                    ExportType::DoNotExport => unreachable!(),
                };

                return (field, extracted_type, property.export_type);
            }).collect();

    item.attrs.push(parse_quote! { #[derive(gdnative::prelude::NativeClass, Default)] });
    item.attrs.push(parse_quote! { #[inherit(#inherit_type)]});

    let impl_block = impl_block::impl_block(&path_fields, &inherit_type, item);

    return quote::quote! {
        #item

        #impl_block
    };
}
