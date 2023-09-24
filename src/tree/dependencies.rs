use std::collections::HashMap;

use super::TreeNode;

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
        let dependency = Dependency {
            node,
            source: source.map(|s| s.to_string()),
        };
        self.items_by_id.insert(id.clone(), dependency);
    }

    pub fn get_item_by_id(&self, id: &str) -> Option<&Dependency> {
        self.items_by_id.get(id)
    }

    pub fn print(&self) {
        for (id, dependency) in &self.items_by_id {
            println!(
                "{},{},{}",
                id,
                dependency.node().name(),
                dependency.source().unwrap_or(""),
            );
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
