
use log;

use syn::visit::Visit;

use crate::function_visitor::ItemNameVisitor;
use crate::types::{
    RustEnum, RustFunction, RustStruct,
};


pub fn fn_dependency_on_target(func: &RustFunction, target_name: &str) -> bool {
    // get clean syntax tree via syn library by feeding it formatted code
    let func_block = func.function_block_str();
    let syntax_tree =
        syn::parse_file(&func_block).expect("Unable to parse code");

    let mut item_names = Vec::new();
    for item in syntax_tree.items {
        if let syn::Item::Fn(item_fn) = item {
            let mut visitor = ItemNameVisitor::new();
            visitor.visit_item_fn(&item_fn);
            item_names.extend(visitor.item_names);
        }
    }
    // Check if any of the extracted names match the target name.
    item_names.iter().any(|name| name == target_name)
}


pub fn enum_dependency_on_target(enu: &RustEnum, target_name: &str) -> bool {
    // get clean syntax tree via syn library by feeding it formatted code
    let enum_variants_block = enu.enum_base_block_str();
    let syntax_tree =
        syn::parse_file(&enum_variants_block).expect("Unable to parse code");

    let mut type_names = Vec::new();
    for item in syntax_tree.items {
        if let syn::Item::Enum(enum_item) = item {
            for variant in &enum_item.variants {
                for field in &variant.fields {
                    extract_type_names(&field.ty, &mut type_names);
                }
            }
        } else {
            panic!("Unexpected item in syntax tree");
        }
    }
    // Check if any of the extracted type names match the target name.
    type_names.iter().any(|name| name == target_name)
}


pub fn struct_dependency_on_target(strct: &RustStruct, target_name: &str) -> bool {
    // get clean syntax tree via syn library by feeding it formatted code
    let struct_fields_block = strct.struct_base_block_str();
    let syntax_tree =
        syn::parse_file(&struct_fields_block).expect("Unable to parse code");

    let mut type_names = Vec::new();
    for item in syntax_tree.items {
        if let syn::Item::Struct(item_struct) = item {
            for field in item_struct.fields {
                extract_type_names(&field.ty, &mut type_names);
            }
        } else {
            panic!("Unexpected item in syntax tree");
        }
    }
    // Check if any of the extracted type names match the target name.
    type_names.iter().any(|name| name == target_name)
}

fn extract_type_names(ty: &syn::Type, type_names: &mut Vec<String>) {
    match ty {
        syn::Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                type_names.push(segment.ident.to_string());
                // Recursively handle nested types
                if let syn::PathArguments::AngleBracketed(
                    angle_bracketed_args,
                ) = &segment.arguments
                {
                    for arg in &angle_bracketed_args.args {
                        match arg {
                            syn::GenericArgument::Type(nested_ty) => {
                                extract_type_names(nested_ty, type_names);
                            }
                            _ => {
                                log::debug!(
                                    "Unexpected generic argument -- not yet \
                                     supported"
                                );
                            }
                        }
                    }
                }
            }
        }
        syn::Type::Tuple(type_tuple) => {
            for elem_ty in &type_tuple.elems {
                extract_type_names(elem_ty, type_names);
            }
        }
        _ => {
            // Path and Tuple catches majority of syntax for now,
            // but should expand this to handle any valid case over time
            log::debug!("Unexpected type -- not yet supported");
        }
    }
}
