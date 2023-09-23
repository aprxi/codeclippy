use std::collections::HashSet;

use crate::file_visitor::{NodeKind, RustFileVisitor};
use crate::registry::GlobalRegistry;
use crate::rust_types::{
    RustEnum, RustFunction, RustStruct, RustTrait, Visibility,
};
use crate::tree::{RootNode, TreeNode};

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
        let mut root = RootNode::new(self.visitor.current_file());
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
            if rust_struct.visibility == Visibility::Public {
                global_registry.register_struct(rust_struct.clone());
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
    let mut node = TreeNode::new(func.name.clone(), NodeKind::Function);
    node.function = Some(func.clone());

    for called_func in &func.functions {
        node.add_child(create_function_node(visitor, called_func, visited));
    }

    for instantiated_struct_name in &func.instantiated_structs {
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
        if let Some(s) = visitor.structs.iter().find(|s| s.name == *struct_name)
        {
            visited.insert(s.name.clone());
            let mut linked_node =
                TreeNode::new(s.name.clone(), NodeKind::Struct);
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
