use std::collections::{HashMap, HashSet};

use crate::file_visitor::{NodeKind, RustFileVisitor};
use crate::registry::GlobalRegistry;
use crate::rust_types::{
    RustEnum, RustFunction, RustStruct, RustTrait, Visibility,
};
use crate::tree::TreeNode;

pub fn populate_tree(
    visitor: &mut RustFileVisitor,
    global_registry: &mut GlobalRegistry,
) -> TreeNode {
    let mut root = TreeNode::new(visitor.current_file(), NodeKind::Root);
    let mut visited = HashSet::new();

    // Populate functions
    for rust_function in &visitor.functions {
        root.add_child(create_function_node(
            visitor,
            rust_function,
            &mut visited,
        ));
    }

    // Populate structs
    for rust_struct in &visitor.structs {
        root.add_child(create_struct_node(visitor, rust_struct, &mut visited));
        // if struct is public, add to global registry so it can be referenced
        // from other files
        if rust_struct.visibility == Visibility::Public {
            println!("Registering public struct {}", rust_struct.name);
            global_registry.register_struct(rust_struct.clone());
        }
    }

    // Populate enums
    for rust_enum in &visitor.enums {
        root.add_child(create_enum_node(rust_enum));
    }

    // Populate traits
    for rust_trait in &visitor.traits {
        root.add_child(create_trait_node(rust_trait));
    }

    root
}

fn create_function_node(
    visitor: &RustFileVisitor,
    func: &RustFunction,
    visited: &mut HashSet<String>,
) -> TreeNode {
    let mut node = TreeNode::new(func.name.clone(), NodeKind::Function);
    node.function = Some(func.clone());

    for called_func in &func.functions {
        node.add_child(create_function_node(visitor, called_func, visited));
    }

    for instantiated_struct_name in &func.instantiated_structs {
        if !visited.contains(instantiated_struct_name) {
            if let Some(s) = visitor
                .structs
                .iter()
                .find(|s| s.name == *instantiated_struct_name)
            {
                visited.insert(s.name.clone());
                let mut linked_node =
                    TreeNode::new(s.name.clone(), NodeKind::Struct);
                linked_node.link =
                    Some(Box::new(create_struct_node(visitor, s, visited)));
                node.add_child(linked_node);
            }
        }
    }
    node
}

fn create_struct_node(
    visitor: &RustFileVisitor,
    s: &RustStruct,
    visited: &mut HashSet<String>,
) -> TreeNode {
    visited.insert(s.name.clone());
    let mut node = TreeNode::new(s.name.clone(), NodeKind::Struct);
    node.fields = Some(s.fields.clone());

    for method in &s.methods {
        node.add_child(create_function_node(visitor, method, visited));
    }

    node
}

fn create_enum_node(e: &RustEnum) -> TreeNode {
    let mut node = TreeNode::new(e.name.clone(), NodeKind::Enum);
    for variant in &e.variants {
        let variant_node = TreeNode::new(variant.0.clone(), NodeKind::Variant);
        node.add_child(variant_node);
    }
    node
}

fn create_trait_node(t: &RustTrait) -> TreeNode {
    let mut node = TreeNode::new(t.name.clone(), NodeKind::Trait);
    for method in &t.methods {
        let method_node =
            TreeNode::new(method.name.clone(), NodeKind::Function);
        node.add_child(method_node);
    }
    node
}

fn create_struct_node_from_registry(s: &RustStruct) -> TreeNode {
    let mut node = TreeNode::new(s.name.clone(), NodeKind::Struct);
    node.fields = Some(s.fields.clone());

    for method in &s.methods {
        let method_node =
            TreeNode::new(method.name.clone(), NodeKind::Function);
        node.add_child(method_node);
    }

    node
}

pub fn link_missing_structs(tree: &mut TreeNode, registry: &GlobalRegistry) {
    let mut added_structs = HashSet::new();
    let mut local_registry = LocalRegistry::default();

    // Register all local structs to the local registry first
    for child in &tree.children {
        if matches!(child.kind(), NodeKind::Struct) {
            local_registry
                .register_struct(child.name().to_string(), child.clone());
        }
    }

    link_missing_structs_recursive(
        tree,
        registry,
        &mut added_structs,
        &mut local_registry,
    );
}

fn link_missing_structs_recursive(
    tree: &mut TreeNode,
    global_registry: &GlobalRegistry,
    added_structs: &mut HashSet<String>,
    local_registry: &mut LocalRegistry,
) {
    match tree.kind() {
        NodeKind::Function => {
            if let Some(func) = &mut tree.function {
                func.extract_function_body();
            }
        }
        _ => {}
    }

    // If the current tree node is a struct, add its name to added_structs
    if matches!(tree.kind(), NodeKind::Struct) {
        added_structs.insert(tree.name().to_string());
    }

    let mut structs_to_add = Vec::new();

    if let Some(func) = &tree.function {
        for instantiated_struct_name in &func.instantiated_structs {
            if !added_structs.contains(instantiated_struct_name) {
                if let Some(existing_node) =
                    local_registry.get_struct(instantiated_struct_name)
                {
                    // TreeNode exists in the local registry, so clone it
                    structs_to_add.push(existing_node.clone());
                    added_structs.insert(instantiated_struct_name.clone());
                } else if let Some(rust_struct) =
                    global_registry.get_struct(instantiated_struct_name)
                {
                    // TreeNode doesn't exist in local registry,
                    // but the RustStruct exists in global registry
                    let struct_node =
                        create_struct_node_from_registry(rust_struct);
                    structs_to_add.push(struct_node);
                    added_structs.insert(instantiated_struct_name.clone());
                }
            }
        }
    }

    for struct_node in structs_to_add {
        tree.add_child(struct_node);
    }

    for child in &mut tree.children {
        link_missing_structs_recursive(
            child,
            global_registry,
            added_structs,
            local_registry,
        );
    }
}

pub struct LocalRegistry {
    structs: HashMap<String, TreeNode>,
    // TODO: add enums, traits, impls, etc.
}

impl Default for LocalRegistry {
    fn default() -> Self {
        LocalRegistry {
            structs: HashMap::new(),
        }
    }
}

impl LocalRegistry {
    pub fn register_struct(
        &mut self,
        rust_struct_name: String,
        node: TreeNode,
    ) {
        self.structs.insert(rust_struct_name, node);
    }

    pub fn get_struct(&self, name: &str) -> Option<&TreeNode> {
        self.structs.get(name)
    }
}
