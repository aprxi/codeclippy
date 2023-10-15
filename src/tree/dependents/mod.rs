mod find_dependents;
mod extract_dependents;

pub use find_dependents::find_dependents;

use std::collections::HashMap;
use log;
use crate::tree::TreeNode;
use crate::types::RustType;
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
