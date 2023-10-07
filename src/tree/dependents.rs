use std::collections::HashMap;

use log;

use crate::tree::{RootNode, TreeNode};
use crate::types::{Identifiable, RustFunction, RustType};
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

        // process impl methods regardless of rust type (struct, enum, trait),
        // as these can handled in the same way as an independent function
        if let Some(methods) = rust_type.methods() {
            process_methods(methods, target_item, &mut dependent_items);
        }
        //
        match rust_type {
            RustType::Function(func) => {
                check_dependency_on_target(func, target_item);
            }
            RustType::Struct(strct) => {
                check_dependency_on_target(strct, target_item);
            }
            RustType::Enum(enu) => {
                check_dependency_on_target(enu, target_item);
            }
            RustType::Trait(trt) => {
                check_dependency_on_target(trt, target_item);
            }
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
    // TODO: determine if dependent_item is used in target_item
    log::debug!("Checking: {}", _dependent_item.name());
    return false;
}

fn process_methods(
    methods: &Vec<RustFunction>,
    target_item: &dyn Identifiable,
    dependent_items: &mut Vec<Box<dyn Identifiable>>,
) {
    for method in methods {
        dependent_items.extend(find_identifiable_items(
            Box::new(RustType::Function(method.clone())),
            target_item,
        ));
    }
}
