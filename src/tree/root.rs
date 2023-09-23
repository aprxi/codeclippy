use super::local_registry::LocalRegistry;
use super::TreeNode;

pub struct RootNode {
    filename: String,
    local_registry: LocalRegistry,
    children: Vec<TreeNode>,
}

impl RootNode {
    pub fn new(filename: String) -> Self {
        RootNode {
            filename,
            local_registry: LocalRegistry::default(),
            children: Vec::new(),
        }
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn set_local_registry(&mut self, local_registry: LocalRegistry) {
        self.local_registry = local_registry;
    }

    pub fn children(&self) -> &Vec<TreeNode> {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut Vec<TreeNode> {
        &mut self.children
    }

    pub fn has_node_named(&self, name: &str) -> bool {
        self.local_registry.name_to_id().contains_key(name)
    }

    pub fn add_child(&mut self, child: TreeNode) {
        self.children.push(child);
    }
}

