use std::collections::HashMap;

use log;

use crate::tree::{RootNode, TreeNode};
use crate::types::{Identifiable, RustStruct, RustEnum, RustType};
use crate::writers::ClippyWriter;

pub struct Dependents {
    items_by_id: HashMap<String, Dependent>,
}

impl Default for Dependents {
    fn default() -> Self {
        Dependents {
            items_by_id: HashMap::new(),
        }
    }
}

impl Dependents {
    pub fn register_item(&mut self, node: TreeNode, source: Option<&str>) {
        log::debug!("adding dependent: {}", node.clone().name());
        let node_id = node.id().to_string();
        let dependent = Dependent::new(node, source);
        self.items_by_id.insert(node_id, dependent);
    }

    pub fn len(&self) -> usize {
        self.items_by_id.len()
    }

    pub fn print(&self, writer: &mut Box<dyn ClippyWriter>) {
        for (_, dependent) in &self.items_by_id {
            self.print_dependent(writer, dependent);
        }
    }

    fn print_dependent(
        &self,
        writer: &mut Box<dyn ClippyWriter>,
        dependent: &Dependent,
    ) {
        let _ = writeln!(writer, "@{}:", dependent.source().unwrap_or(""),);

        match dependent.node().rtype() {
            RustType::Function(rust_function) => {
                let _ = write!(writer, "{}", rust_function);
            }
            RustType::Struct(rust_struct) => {
                let _ = write!(writer, "{}", rust_struct);
            }
            RustType::Enum(rust_enum) => {
                let _ = write!(writer, "{}", rust_enum);
            }
            RustType::Trait(rust_trait) => {
                let _ = write!(writer, "{}", rust_trait);
            }
        }
    }
}

pub struct Dependent {
    node: TreeNode,
    source: Option<String>,
}

impl Dependent {
    pub fn new(node: TreeNode, source: Option<&str>) -> Self {
        Dependent {
            node,
            source: source.map(|s| s.to_string()),
        }
    }

    pub fn node(&self) -> &TreeNode {
        &self.node
    }
    pub fn source(&self) -> Option<&str> {
        self.source.as_ref().map(|s| s.as_str())
    }
}

pub fn find_dependents(
    root_node: &mut RootNode,
    target_item: &dyn Identifiable,
    _filter_path: Vec<&str>,
) {
    let mut dependents = Dependents::default();
    let source = root_node.file_path().real_path();

    for node in root_node.children_mut().iter_mut() {
        let rust_item = node.rtype();

        let items_found =
            find_identifiable_items(Box::new(rust_item.clone()), target_item);
        if !items_found.is_empty() {
            // TODO: for now register complete item if a dependency is found.
            // ideally we should register only parts that have the
            // target_item as a direct dependency (example: a method of a
            // struct instead of a full struct).
            log::debug!(
                "Found {} dependents for {}",
                items_found.len(),
                target_item.name()
            );
            dependents.register_item(
                node.clone(), // TODO: why not register RustItem instead
                Some(&source),
            );
        }
    }
    root_node.set_dependents(dependents);
}

fn find_identifiable_items(
    item: Box<dyn Identifiable>,
    target_item: &dyn Identifiable,
) -> Vec<Box<dyn Identifiable>> {
    // item cant depend on itself
    if item.id() == target_item.id() {
        return vec![];
    }
    let mut dependent_items = Vec::new();
    if let Some(rust_type) = item.as_rust_type() {
        // impl methods on each type (struct, enum, trait), and nested
        // functions can be processed as a RustFunction
        // call this function recursively for each
        if let Some(methods) = rust_type.methods() {
            for method in methods {
                dependent_items.extend(find_identifiable_items(
                    Box::new(RustType::Function(method.clone())),
                    target_item,
                ));
            }
        }

        // check item for dependency on target
        // note excluding (impl) methods as this is already done
        let is_dependent = match rust_type {
            RustType::Function(func) => {
                check_dependency_on_target(func, target_item)
            }
            RustType::Struct(strct) => {
                struct_dependency_on_target(strct, target_item.name())
            }
            RustType::Enum(enu) => {
                enum_dependency_on_target(enu, target_item.name())
            }
            // Trait should not have additinonal dependents
            // (methods are already checked as functions above)
            RustType::Trait(_) => false,
        };
        if is_dependent {
            dependent_items.push(item);
        }
    } else {
        panic!("item is not a RustType: {:?}", item.name());
    }
    dependent_items
}

fn check_dependency_on_target(
    _dependent_item: &dyn Identifiable,
    _target_item: &dyn Identifiable,
) -> bool {
    // placeholder - each type requires its own parser function
    log::debug!("Checking: {}", _dependent_item.name());
    return false;
}


fn enum_dependency_on_target(enu: &RustEnum, target_name: &str) -> bool {
    // get clean syntax tree via syn library by feeding it formatted code
    let enum_variants_block = enu.enum_base_block_str();
    let syntax_tree =
        syn::parse_file(&enum_variants_block).expect("Unable to parse code");

    let mut type_names = Vec::new();
    for item in syntax_tree.items {
        if let syn::Item::Enum(enum_item) = item {
            extract_type_names_from_enum(&enum_item, &mut type_names);
        } else {
            panic!("Unexpected item in syntax tree");
        }
    }
    // Check if any of the extracted type names match the target name.
    type_names.iter().any(|name| name == target_name)
}


fn extract_type_names_from_enum(enum_item: &syn::ItemEnum, type_names: &mut Vec<String>) {
    for variant in &enum_item.variants {
        for field in &variant.fields {
            extract_type_names(&field.ty, type_names);
        }
    }
}


fn struct_dependency_on_target(strct: &RustStruct, target_name: &str) -> bool {
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
