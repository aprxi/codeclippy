use std::collections::HashMap;

use uuid::Uuid;

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

    pub fn local_registry(&self) -> &LocalRegistry {
        &self.local_registry
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
        self.local_registry.name_to_id.contains_key(name)
    }

    pub fn add_child(&mut self, child: TreeNode) {
        self.children.push(child);
    }
}

pub struct LocalRegistry {
    structs_by_id: HashMap<Uuid, TreeNode>,
    name_to_id: HashMap<String, Uuid>, /* map struct name to its UUID
    * TODO: add enums, traits, impls, etc. */
}

impl Default for LocalRegistry {
    fn default() -> Self {
        LocalRegistry {
            structs_by_id: HashMap::new(),
            name_to_id: HashMap::new(),
        }
    }
}

impl LocalRegistry {
    pub fn register_struct(
        &mut self,
        rust_struct_name: String,
        node: TreeNode,
    ) -> Uuid {
        // Generate a new UUID for the struct
        let id = Uuid::new_v4();

        // Insert the struct using the UUID
        self.structs_by_id.insert(id, node);

        // Map the struct name to its UUID
        self.name_to_id.insert(rust_struct_name, id);

        id
    }

    pub fn print(&self) {
        for (id, struct_node) in &self.structs_by_id {
            println!("UUID: {:?}, TreeNode: {:?}", id, struct_node);
        }
    }

    pub fn get_struct_by_id(&self, id: &Uuid) -> Option<&TreeNode> {
        self.structs_by_id.get(id)
    }

    pub fn get_struct_by_name(&self, name: &str) -> Option<&TreeNode> {
        if let Some(id) = self.name_to_id.get(name) {
            return self.get_struct_by_id(id);
        }
        None
    }
}
