use std::collections::HashMap;

use log;

use crate::tree::{RootNode, TreeNode};
use crate::types::{Identifiable, RustStruct, RustType};
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
                check_dependency_on_target(enu, target_item)
            }
            RustType::Trait(trt) => {
                check_dependency_on_target(trt, target_item)
            }
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

fn struct_dependency_on_target(strct: &RustStruct, target_name: &str) -> bool {
    // Get struct in readable (cleaned, validated) code format.
    let struct_fields_code = strct.struct_base_block_str();
    // Parse struct code into syntax tree.
    let syntax_tree =
        syn::parse_file(&struct_fields_code).expect("Unable to parse code");

    let mut type_names = Vec::new();
    for item in syntax_tree.items {
        match item {
            syn::Item::Struct(item_struct) => {
                for field in item_struct.fields {
                    extract_type_names(&field.ty, &mut type_names);
                }
            }
            _ => {
                panic!("Unexpected item in syntax tree");
            }
        }
    }
    // Check if target_name is in type_names
    type_names.iter().any(|name| name == target_name)
}

fn extract_type_names(ty: &syn::Type, names: &mut Vec<String>) {
    match ty {
        syn::Type::Path(type_path) => {
            let path = &type_path.path;
            let name = path
                .segments
                .iter()
                .map(|seg| seg.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");
            names.push(name);
        }
        syn::Type::Tuple(type_tuple) => {
            for elem_ty in &type_tuple.elems {
                extract_type_names(elem_ty, names);
            }
        }
        // Path and Tuple catches majority of syntax for now,
        // but should expand this to handle any valid case over time
        _ => {}
    }
}
