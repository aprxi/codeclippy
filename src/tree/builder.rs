use std::collections::HashMap;

use log;

use super::initialize::ChunkInitializer;
use crate::file_visitor::{NodeKind, RustFileVisitor};
use crate::print_config::{PrintConfig, PrintConfigBuilder};
use crate::registry::{GlobalRegistry, RegistryKind};
use crate::rust_types::RustStruct;
use crate::tree::{Dependencies, RootNode, TreeNode};

pub struct TreeBuilder {
    visitors: Vec<RustFileVisitor>,
    global_registry: GlobalRegistry,
    use_full_path: bool,
}

impl TreeBuilder {
    pub fn new(visitors: Vec<RustFileVisitor>, use_full_path: bool) -> Self {
        TreeBuilder {
            visitors,
            global_registry: GlobalRegistry::default(),
            use_full_path,
        }
    }

    pub fn use_full_path(&self) -> bool {
        self.use_full_path
    }

    pub fn initialize_chunks(
        &mut self,
        filter: Option<&str>,
        link_dependencies: bool,
    ) -> Vec<RootNode> {
        let validate = !self.use_full_path;
        let mut chunks: Vec<RootNode> = self
            .visitors
            .iter_mut()
            .map(|visitor| {
                let mut initializer = ChunkInitializer::new(visitor);
                initializer.initialize_tree(&mut self.global_registry)
            })
            .collect();

        if validate {
            self.validate_chunks_for_conflicts(&chunks, filter);
        }

        if link_dependencies {
            self.link_dependencies(&mut chunks, filter, self.use_full_path);
        }
        chunks
    }

    pub fn link_dependencies(
        &mut self,
        chunks: &mut Vec<RootNode>,
        filter: Option<&str>,
        use_full_path: bool,
    ) {
        for mut root in chunks {
            if let Some(filter_str) = filter {
                let config = PrintConfigBuilder::new()
                    .depth(0)
                    .filter(Some(filter_str.to_string()))
                    .path(vec![root.filename().to_string()])
                    .is_linked(false)
                    .use_full_path(use_full_path)
                    .build();

                self.add_dependencies(&mut root, &mut config.clone());
            }
        }
    }

    fn validate_chunks_for_conflicts(
        &self,
        chunks: &[RootNode],
        filter: Option<&str>,
    ) {
        if let Some(filter_str) = filter {
            let first_component = filter_str.split("::").next().unwrap();
            let potential_conflicts: Vec<_> = chunks
                .iter()
                .filter(|tree| tree.has_child_named(first_component))
                .collect();

            if potential_conflicts.len() > 1 {
                panic!(
                    "Potential conflict found. More than one chunk has a node \
                     named {}. Please specify a more specific filter.",
                    first_component
                );
            }
        }
    }

    pub fn add_dependencies(&self, root: &mut RootNode, config: &PrintConfig) {
        find_dependencies(root, &self.global_registry, config);
    }
}

fn find_dependencies(
    root: &mut RootNode,
    registry: &GlobalRegistry,
    config: &PrintConfig,
) {
    let mut dependencies = Dependencies::default();
    let local_items_map: HashMap<String, TreeNode> = root
        .children()
        .iter()
        .map(|item| (item.name().to_string(), item.clone()))
        .collect();

    for child in root.children_mut().iter_mut() {
        let mut inner_config = config.clone();
        inner_config.add_to_path(child.name().to_string());

        find_dependencies_recursive(
            child,
            registry,
            &mut dependencies,
            &local_items_map,
            &inner_config,
        );
    }

    log::debug!("Found {} dependencies", dependencies.len());
    root.set_dependencies(dependencies);
}

fn find_dependencies_recursive(
    tree: &mut TreeNode,
    global_registry: &GlobalRegistry,
    dependencies: &mut Dependencies,
    local_items_map: &HashMap<String, TreeNode>,
    config: &PrintConfig,
) {
    log::debug!("processing node: {}", tree.name());
    process_function_node(tree);

    let nodes_to_add = collect_missing_structs(
        tree,
        global_registry,
        local_items_map,
        dependencies,
    );

    // Process child nodes directly here
    for child in tree.children_mut() {
        let mut inner_config = config.clone();
        inner_config.add_to_path(child.name().to_string());

        if child.should_print(&inner_config) {
            // Recursively call this function on each child
            find_dependencies_recursive(
                child,
                global_registry,
                dependencies,
                local_items_map,
                &inner_config,
            );
        } else {
            log::debug!("Skipping node: {}", child.name());
        }
    }

    // Adding nodes at the end to avoid infinite recursion
    for node in nodes_to_add {
        log::debug!("Adding node: {}", node.name());
        tree.add_child(node);
    }
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
    local_items_map: &HashMap<String, TreeNode>,
    dependencies: &mut Dependencies,
) -> Vec<TreeNode> {
    let mut nodes_to_add = Vec::new();

    if let Some(func) = &tree.function {
        // List of all instantiated structs from the function
        let instantiated_struct_names: Vec<_> =
            func.instantiated_structs.iter().cloned().collect();

        for name in &instantiated_struct_names {
            // Check if the struct exists in local items
            if let Some(local_item) = local_items_map.get(name) {
                let node = (*local_item).clone();
                dependencies.register_item(
                    node.id().to_string(),
                    node.clone(),
                    None,
                );
                nodes_to_add.push(convert_to_linknode(node));
            }
            // If not in the local items, try global registry
            else if let Some(registry_item) =
                global_registry.get_item_by_name(name)
            {
                let RegistryKind::Struct(rust_struct) = &registry_item.item();
                let node = create_struct_node_from_registry(rust_struct);
                dependencies.register_item(
                    node.id().to_string(),
                    node.clone(),
                    registry_item.source(),
                );
                nodes_to_add.push(convert_to_linknode(node));
            }
        }
    }
    nodes_to_add
}

fn create_struct_node_from_registry(s: &RustStruct) -> TreeNode {
    let mut node = TreeNode::new(&s.id, &s.name, NodeKind::Struct);
    node.fields = Some(s.fields.clone());

    for method in &s.methods {
        let method_node =
            TreeNode::new(&method.id, &method.name, NodeKind::Function);
        node.add_child(method_node);
    }

    node
}

fn convert_to_linknode(s: TreeNode) -> TreeNode {
    TreeNode::new(s.id(), s.name(), NodeKind::Link)
}
