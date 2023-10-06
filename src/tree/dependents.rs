use std::collections::HashMap;

use log;

use crate::tree::{RootNode, TreeNode};
use crate::types::{Identifiable, RustType};
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
    pub fn register_item(
        &mut self,
        node: TreeNode,
        source: Option<&str>,
    ) {
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

    for child in root_node.children_mut().iter_mut() {
        let dependent_item = child.rtype();
        let is_dependent = check_dependency_on_target(
            dependent_item,
            target_item,
        );
        if is_dependent {
            dependents.register_item(
                child.clone(),
                Some(&source),
            );
        }
    }
    root_node.set_dependents(dependents);
}


fn check_dependency_on_target(
    _dependent_item: &dyn Identifiable,
    _target_item: &dyn Identifiable,
) -> bool {
    // TODO: determine if dependent_item is used in target_item
    return false;
}

