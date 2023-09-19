use std::collections::HashSet;

use crate::file_visitor::{NodeKind, RustFileVisitor};
use crate::print_config::PrintConfig;
use crate::registry::GlobalRegistry;
use crate::rust_types::{
    RustEnum, RustFunction, RustStruct, RustTrait, Visibility,
};
use crate::tree::{LocalRegistry, RootNode, TreeNode};

pub fn populate_tree(
    visitor: &mut RustFileVisitor,
    global_registry: &mut GlobalRegistry,
) -> RootNode {
    let mut root = RootNode::new(visitor.current_file());
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

pub fn link_missing_structs(
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
        if child.should_print(config) {
            link_missing_structs_recursive(
                child,
                registry,
                &mut local_registry,
                config,
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
    match tree.kind() {
        NodeKind::Function => {
            if let Some(func) = &mut tree.function {
                func.extract_function_body();
            }
        }
        _ => {}
    }

    let mut structs_to_add = Vec::new();

    if let Some(func) = &tree.function {
        for instantiated_struct_name in &func.instantiated_structs {
            if local_registry
                .get_struct_by_name(instantiated_struct_name)
                .is_none()
            {
                if let Some(rust_struct) =
                    global_registry.get_struct_by_name(instantiated_struct_name)
                {
                    println!(
                        "Found struct {} in global registry",
                        instantiated_struct_name
                    );
                    let new_node =
                        create_struct_node_from_registry(rust_struct);
                    structs_to_add.push(new_node.clone());

                    local_registry.register_struct(
                        instantiated_struct_name.clone(),
                        new_node,
                    );
                }
            }
        }
    }
    // move all structs from temporary vector to tree node
    for struct_node in structs_to_add {
        tree.add_child(struct_node);
    }

    for child in &mut tree.children {
        if child.should_print(config) {
            link_missing_structs_recursive(
                child,
                global_registry,
                local_registry,
                config,
            );
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
