use std::collections::HashSet;

use crate::file_visitor::RustFileVisitor;
use crate::registry::GlobalRegistry;
use crate::tree::{RootNode, TreeNode};
use crate::types::{
    RustEnum, RustFunction, RustStruct, RustTrait, RustType, Visibility,
};

pub struct ChunkInitializer<'a> {
    visitor: &'a mut RustFileVisitor,
}

impl<'a> ChunkInitializer<'a> {
    pub fn new(visitor: &'a mut RustFileVisitor) -> Self {
        ChunkInitializer { visitor }
    }

    pub fn initialize_tree(
        &mut self,
        global_registry: &mut GlobalRegistry,
    ) -> RootNode {
        let mut root = RootNode::new(self.visitor.file_path().clone());
        let mut visited = HashSet::new();

        self.add_functions(&mut root, &mut visited);
        self.add_structs(&mut root, global_registry, &mut visited);
        self.add_enums(&mut root);
        self.add_traits(&mut root);

        root
    }

    fn add_functions(
        &self,
        root: &mut RootNode,
        visited: &mut HashSet<String>,
    ) {
        for rust_function in &self.visitor.functions {
            root.add_child(create_function_node(
                self.visitor,
                rust_function,
                visited,
            ));
        }
    }

    fn add_structs(
        &mut self,
        root: &mut RootNode,
        global_registry: &mut GlobalRegistry,
        visited: &mut HashSet<String>,
    ) {
        for rust_struct in &self.visitor.structs {
            root.add_child(create_struct_node(
                self.visitor,
                rust_struct,
                visited,
            ));
            if *rust_struct.visibility() == Visibility::Public {
                global_registry.register_struct(
                    rust_struct.clone(),
                    Some(root.file_path().relative_path().as_str()),
                );
            }
        }
    }

    fn add_enums(&self, root: &mut RootNode) {
        for rust_enum in &self.visitor.enums {
            root.add_child(create_enum_node(rust_enum));
        }
    }

    fn add_traits(&self, root: &mut RootNode) {
        for rust_trait in &self.visitor.traits {
            root.add_child(create_trait_node(rust_trait));
        }
    }
}

fn create_function_node(
    visitor: &RustFileVisitor,
    func: &RustFunction,
    visited: &mut HashSet<String>,
) -> TreeNode {
    let mut node = TreeNode::new(RustType::Function(func.clone()));
    for called_func in func.functions() {
        node.add_child(create_function_node(visitor, called_func, visited));
    }

    for instantiated_struct_name in func.instantiated_items() {
        if let Some(child_node) = create_linked_struct_node(
            visitor,
            instantiated_struct_name,
            visited,
        ) {
            node.add_child(child_node);
        }
    }

    node
}

fn create_linked_struct_node(
    visitor: &RustFileVisitor,
    struct_name: &String,
    visited: &mut HashSet<String>,
) -> Option<TreeNode> {
    if !visited.contains(struct_name) {
        if let Some(s) =
            visitor.structs.iter().find(|s| s.name() == *struct_name)
        {
            visited.insert(s.name().to_string());
            let mut linked_node = TreeNode::new(RustType::Struct(s.clone()));
            linked_node.link =
                Some(Box::new(create_struct_node(visitor, s, visited)));
            return Some(linked_node);
        }
    }
    None
}

fn create_struct_node(
    visitor: &RustFileVisitor,
    s: &RustStruct,
    visited: &mut HashSet<String>,
) -> TreeNode {
    visited.insert(s.name().to_string());
    let mut node = TreeNode::new(RustType::Struct(s.clone()));
    for method in s.methods() {
        node.add_child(create_function_node(visitor, method, visited));
    }

    node
}

fn create_enum_node(e: &RustEnum) -> TreeNode {
    TreeNode::new(RustType::Enum(e.clone()))
}

fn create_trait_node(t: &RustTrait) -> TreeNode {
    let mut node = TreeNode::new(RustType::Trait(t.clone()));
    for method in &t.methods {
        let method_node = TreeNode::new(RustType::Function(method.clone()));
        node.add_child(method_node);
    }
    node
}
