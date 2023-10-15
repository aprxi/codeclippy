
use crate::tree::RootNode;
use crate::types::{
    Identifiable, RustType,
};
use super::Dependents;
use super::extract_dependents::*;

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
                fn_dependency_on_target(func, target_item.name())
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

