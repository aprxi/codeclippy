
use std::collections::HashSet;

use crate::file_visitor::{NodeKind, RustFileVisitor};
use crate::print_config::PrintConfig;
use crate::registry::GlobalRegistry;
use crate::rust_types::{
    RustEnum, RustFunction, RustStruct, RustTrait, Visibility,
};
use crate::tree::{LocalRegistry, RootNode, TreeNode};

pub struct TreeBuilder {
    global_registry: GlobalRegistry,
}

impl TreeBuilder {
    pub fn new() -> Self {
        TreeBuilder {
            global_registry: GlobalRegistry::default(),
        }
    }

    pub fn build_tree(&mut self, visitor: &mut RustFileVisitor) -> RootNode {
        let mut root = RootNode::new(visitor.current_file());
        let mut visited = HashSet::new();

        for rust_function in &visitor.functions {
            root.add_child(create_function_node(visitor, rust_function, &mut visited));
        }
        for rust_struct in &visitor.structs {
            root.add_child(create_struct_node(visitor, rust_struct, &mut visited));
            if rust_struct.visibility == Visibility::Public {
                self.global_registry.register_struct(rust_struct.clone());
            }
        }
        for rust_enum in &visitor.enums {
            root.add_child(create_enum_node(rust_enum));
        }
        for rust_trait in &visitor.traits {
            root.add_child(create_trait_node(rust_trait));
        }

        root
    }

    pub fn link_missing_structs(&self, root: &mut RootNode, config: &PrintConfig) {
        link_missing_structs(root, &self.global_registry, config);
    }
}


fn link_missing_structs(
    root: &mut RootNode,
    registry: &GlobalRegistry,
    config: &PrintConfig,
) {
    let mut local_registry = LocalRegistry::default();

    // Register all local structs to the local registry first
    for child in root.children() {
        if child.should_print(config)
            && matches!(child.kind(), NodeKind::Struct)
        {
            local_registry
                .register_struct(child.name().to_string(), child.clone());
        }
    }

    for child in root.children_mut().iter_mut() {
        let mut inner_config = config.clone();
        inner_config.add_to_path(child.name().to_string());

        if child.should_print(&inner_config) {
            link_missing_structs_recursive(
                child,
                registry,
                &mut local_registry,
                &inner_config,
            );
        }
    }
    root.set_local_registry(local_registry);
}

fn link_missing_structs_recursive(
    tree: &mut TreeNode,
    global_registry: &GlobalRegistry,
    local_registry: &mut LocalRegistry,
    config: &PrintConfig,
) {
    process_function_node(tree);

    let structs_to_add = collect_missing_structs(tree, global_registry, local_registry);
    for struct_node in structs_to_add {
        tree.add_child(struct_node);
    }

    process_child_nodes(tree, global_registry, local_registry, config);
}

fn process_function_node(tree: &mut TreeNode) {
    if let NodeKind::Function = tree.kind() {
        if let Some(func) = &mut tree.function {
            func.extract_function_body();
        }
    }
}

fn collect_missing_structs(
    tree: &TreeNode,
    global_registry: &GlobalRegistry,
    local_registry: &mut LocalRegistry,
) -> Vec<TreeNode> {
    if let Some(func) = &tree.function {
        // First, gather names of missing structs from the local registry
        let missing_struct_names: Vec<_> = func
            .instantiated_structs
            .iter()
            .filter(|name| local_registry.get_struct_by_name(name).is_none())
            .cloned()
            .collect();

        // Then, for each missing struct name, retrieve it from the global registry
        // and register it in the local registry
        let mut nodes_to_add = Vec::new();
        for name in missing_struct_names {
            if let Some(rust_struct) = global_registry.get_struct_by_name(&name) {
                let node = create_struct_node_from_registry(rust_struct);
                local_registry.register_struct(name.clone(), node.clone());
                nodes_to_add.push(node);
            }
        }
        return nodes_to_add;
    }

    Vec::new()
}

fn process_child_nodes(
    tree: &mut TreeNode,
    global_registry: &GlobalRegistry,
    local_registry: &mut LocalRegistry,
    config: &PrintConfig,
) {
    for child in &mut tree.children {
        let mut inner_config = config.clone();
        inner_config.add_to_path(child.name().to_string());

        if child.should_print(&inner_config) {
            link_missing_structs_recursive(
                child,
                global_registry,
                local_registry,
                &inner_config,
            );
        }
    }
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
        if let Some(child_node) = create_linked_struct_node(visitor, instantiated_struct_name, visited) {
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
        if let Some(s) = visitor.structs.iter().find(|s| s.name == *struct_name) {
            visited.insert(s.name.clone());
            let mut linked_node = TreeNode::new(s.name.clone(), NodeKind::Struct);
            linked_node.link = Some(Box::new(create_struct_node(visitor, s, visited)));
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

