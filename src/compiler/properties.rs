use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream, Result};
use syn::parse_macro_input::parse;
use syn::{parenthesized, Expr, Field, ItemStruct, Type, parse_str};
use quote::quote;

mod kw {
    syn::custom_keyword!(export_path);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ExportType {
    DoNotExport,
    ExportNode,
    ExportInstance,
    ExportNodeVec,
    ExportInstanceVec,
}


pub struct Property {
    pub name: Ident,
    pub var_type: Type,
    pub export_type: ExportType,
    pub default: Option<Expr>,
}

impl Property {
    fn new(name: Ident, var_type: Type) -> Self {
        Self {
            name,
            var_type,
            export_type: ExportType::DoNotExport,
            default: None,
        }
    }
}

struct DefaultProperty {
    pub expr: Expr,
}

impl Parse for DefaultProperty {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _ = parenthesized!(content in input);
        let expr = content.parse()?;
        Ok(Self { expr })
    }
}

#[allow(clippy::module_name_repetitions)]
pub fn extract_properties(item: &mut ItemStruct) -> Vec<Property> {
    return item.fields.iter_mut().map(|x| get_property(x)).collect();
}

pub fn get_property(field: &mut Field) -> Property {
    let mut property = Property::new(field.ident.as_ref().expect("Properties must be on named field").clone(), field.ty.clone());

    field.attrs = field.attrs.iter()
            .filter(|attr| {
                let Some(ident) = attr.path.get_ident()
                        else { return true };

                let ident = ident.to_string();
                let tokens = attr.tokens.clone().into();

                match ident.as_str() {
                    "default" => {
                        let default = parse::<DefaultProperty>(tokens).expect("Invalid params for default").expr;
                        property.default = Some(default);
                        return false;
                    }
                    "export_path" => {
                        let (_, export_type) = get_field_type(&field.ty)
                                .expect("Invalid type for export_path: {field_type}, supported types are: \n
                        Option<Ref<...>>, \n
                        Option<Instance<...>>, \n
                        Vec<Ref<...>>, \n
                        Vec<Instance<...>>");

                        property.export_type = export_type;
                        return false;
                    },
                    _ => return true,
                }
            }).cloned().collect();

    return property;
}

pub fn get_field_type(source_type: &Type) -> Option<(Type, ExportType)> {
    let type_string = (quote! { #source_type }).to_string();

    if let Some((start, end)) = matches_prefix_suffix(&type_string, "Option<Ref<", ">>") {
        let extracted_type_string = &type_string[start..(type_string.len() - end)];
        let extracted_type = parse_str::<Type>(extracted_type_string).expect(format!("Failed to parse extracted type: {extracted_type_string}").as_str());
        return Some((extracted_type, ExportType::ExportNode));
    }

    if let Some((start, end)) = matches_prefix_suffix(&type_string, "Option<Instance<", ">>") {
        let extracted_type_string = &type_string[start..(type_string.len() - end)];
        let extracted_type = parse_str::<Type>(extracted_type_string).expect(format!("Failed to parse extracted type: {extracted_type_string}").as_str());
        return Some((extracted_type, ExportType::ExportInstance));
    }

    if let Some((start, end)) = matches_prefix_suffix(&type_string, "Vec<Ref<", ">>") {
        let extracted_type_string = &type_string[start..(type_string.len() - end)];
        let extracted_type = parse_str::<Type>(extracted_type_string).expect(format!("Failed to parse extracted type: {extracted_type_string}").as_str());
        return Some((extracted_type, ExportType::ExportNodeVec));
    }

    if let Some((start, end)) = matches_prefix_suffix(&type_string, "Vec<Instance<", ">>") {
        let extracted_type_string = &type_string[start..(type_string.len() - end)];
        let extracted_type = parse_str::<Type>(extracted_type_string).expect(format!("Failed to parse extracted type: {extracted_type_string}").as_str());
        return Some((extracted_type, ExportType::ExportInstanceVec));
    }

    return None;
}

/// Returns the start and end index of the prefix and suffix if they match, otherwise returns None
fn matches_prefix_suffix(input: &String, prefix: &str, suffix: &str) -> Option<(usize, usize)> {
    let mut extracted_index_start = 0;
    let mut extracted_index_end = 0;

    let mut expected_index = 0;
    for char in input.chars() {
        extracted_index_start += 1;
        if char.is_whitespace() {
            continue;
        }

        if char != prefix.chars().nth(expected_index)? {
            return None;
        }

        expected_index += 1;

        if expected_index >= prefix.len() {
            break;
        }
    }

    let mut expected_index = 0;
    for char in input.chars().rev() {
        extracted_index_end += 1;

        if char.is_whitespace() {
            continue;
        }

        if char != suffix.chars().nth_back(expected_index)? {
            return None;
        }

        expected_index += 1;

        if expected_index >= suffix.len() {
            break;
        }
    }

    return Some((extracted_index_start, extracted_index_end));
}
