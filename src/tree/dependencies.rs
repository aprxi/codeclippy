use std::collections::HashMap;

use log;

use super::TreeNode;
use crate::types::RustType;

pub struct Dependencies {
    items_by_id: HashMap<String, Dependency>,
}

impl Default for Dependencies {
    fn default() -> Self {
        Dependencies {
            items_by_id: HashMap::new(),
        }
    }
}

impl Dependencies {
    pub fn register_item(
        &mut self,
        id: String,
        node: TreeNode,
        source: Option<&str>,
    ) {
        log::debug!("adding dependency: {}", node.clone().name());
        let dependency = Dependency {
            node,
            source: source.map(|s| s.to_string()),
        };
        self.items_by_id.insert(id.clone(), dependency);
    }

    pub fn len(&self) -> usize {
        self.items_by_id.len()
    }

    pub fn print(&self) {
        for (id, dependency) in &self.items_by_id {
            self.print_dependency(id, dependency);
        }
    }

    fn print_dependency(&self, id: &String, dependency: &Dependency) {
        println!(
            "{},{},{}",
            id,
            dependency.node().name(),
            dependency.source().unwrap_or(""),
        );

        match dependency.node().rtype() {
            RustType::Function(rust_function) => {
                println!("{}", rust_function);
            }
            RustType::Struct(rust_struct) => {
                println!("{}", rust_struct);
            }
            _ => {
                log::error!(
                    "not supported yet: {:?}",
                    dependency.node().rtype()
                );
            }
        }
    }
}

pub struct Dependency {
    node: TreeNode,
    source: Option<String>,
}

impl Dependency {
    pub fn node(&self) -> &TreeNode {
        &self.node
    }
    pub fn source(&self) -> Option<&str> {
        self.source.as_ref().map(|s| s.as_str())
    }
}
