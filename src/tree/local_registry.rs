use std::collections::HashMap;

use super::TreeNode;
use crate::helpers::generate_id;

pub struct LocalRegistry {
    items_by_id: HashMap<String, LocalRegistryItem>,
    name_to_id: HashMap<String, String>,
}

impl Default for LocalRegistry {
    fn default() -> Self {
        LocalRegistry {
            items_by_id: HashMap::new(),
            name_to_id: HashMap::new(),
        }
    }
}

impl LocalRegistry {
    pub fn name_to_id(&self) -> &HashMap<String, String> {
        &self.name_to_id
    }

    pub fn register_item(
        &mut self,
        tree_node_name: String,
        node: TreeNode,
        source: Option<&str>,
    ) {
        let id = generate_id(&tree_node_name);
        let registry_item = LocalRegistryItem {
            node,
            source: source.map(|s| s.to_string()),
        };

        self.items_by_id.insert(id.clone(), registry_item);
        self.name_to_id.insert(tree_node_name, id);
    }

    pub fn get_item_by_id(&self, id: &str) -> Option<&LocalRegistryItem> {
        self.items_by_id.get(id)
    }

    pub fn get_item_by_name(&self, name: &str) -> Option<&LocalRegistryItem> {
        if let Some(id) = self.name_to_id.get(name) {
            return self.get_item_by_id(id);
        }
        None
    }

    pub fn print(&self) {
        for (id, registry_item) in &self.items_by_id {
            println!(
                "Name: {}\nID: {:?}\nSource: {}\nContents:\n{:?}",
                registry_item.node().name(),
                id,
                registry_item.source().unwrap_or(""),
                registry_item.node()
            );
        }
    }
}

pub struct LocalRegistryItem {
    node: TreeNode,
    source: Option<String>,
}

impl LocalRegistryItem {
    pub fn node(&self) -> &TreeNode {
        &self.node
    }
    pub fn source(&self) -> Option<&str> {
        self.source.as_ref().map(|s| s.as_str())
    }
}
