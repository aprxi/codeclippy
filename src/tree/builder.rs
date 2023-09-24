use super::initialize::ChunkInitializer;
use crate::file_visitor::{NodeKind, RustFileVisitor};
use crate::print_config::{PrintConfig, PrintConfigBuilder};
use crate::registry::{GlobalRegistry, RegistryKind};
use crate::rust_types::RustStruct;
use crate::tree::{LocalRegistry, RootNode, TreeNode};

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
                .filter(|tree| tree.has_node_named(first_component))
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
    let mut local_registry = LocalRegistry::default();

    // Register all local structs to the local registry first
    for child in root.children() {
        if matches!(child.kind(), NodeKind::Struct) {
            local_registry.register_item(
                child.name().to_string(),
                child.clone(),
                None,
            );
        }
    }

    for child in root.children_mut().iter_mut() {
        let mut inner_config = config.clone();
        inner_config.add_to_path(child.name().to_string());

        if child.should_print(&inner_config) {
            find_dependencies_recursive(
                child,
                registry,
                &mut local_registry,
                &inner_config,
            );
        }
    }
    root.set_local_registry(local_registry);
}

fn find_dependencies_recursive(
    tree: &mut TreeNode,
    global_registry: &GlobalRegistry,
    local_registry: &mut LocalRegistry,
    config: &PrintConfig,
) {
    process_function_node(tree);

    let structs_to_add =
        collect_missing_structs(tree, global_registry, local_registry);

    process_child_nodes(tree, global_registry, local_registry, config);

    // ensure to add childs last to prevent infinite recursion
    for struct_node in structs_to_add {
        tree.add_child(struct_node);
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
    local_registry: &mut LocalRegistry,
) -> Vec<TreeNode> {
    if let Some(func) = &tree.function {
        // List of all instantiated structs from the function
        let instantiated_struct_names: Vec<_> =
            func.instantiated_structs.iter().cloned().collect();

        // Create an empty vector to collect nodes
        let mut nodes_to_add = Vec::new();

        for name in &instantiated_struct_names {
            // Check if the struct exists in local registry
            if let Some(local_item) = local_registry.get_item_by_name(name) {
                let local_node = local_item.node().clone();
                nodes_to_add.push(local_node.clone());
            }
            // If not in the local registry, try the global registry
            else if let Some(registry_item) =
                global_registry.get_item_by_name(name)
            {
                let RegistryKind::Struct(rust_struct) = &registry_item.item();
                let node = create_struct_node_from_registry(rust_struct);
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
            find_dependencies_recursive(
                child,
                global_registry,
                local_registry,
                &inner_config,
            );
        }
    }
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
