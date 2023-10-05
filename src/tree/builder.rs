use super::dependencies::find_dependencies;
use super::initialize::ChunkInitializer;
use crate::file_visitor::RustFileVisitor;
use crate::print_config::PrintConfigBuilder;
use crate::registry::GlobalRegistry;
use crate::tree::{RootNode, TreeNode};
use crate::types::Identifiable;

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

    pub fn initialize_root_nodes(
        &mut self,
        filter: Option<&str>,
        link_dependencies: bool,
        link_dependents: bool,
    ) -> Vec<RootNode> {
        let mut root_nodes: Vec<RootNode> = self
            .visitors
            .iter_mut()
            .map(|visitor| {
                let mut initializer = ChunkInitializer::new(visitor);
                initializer.initialize_tree(&mut self.global_registry)
            })
            .collect();

        if !self.use_full_path {
            self.validate_chunks_for_conflicts(&root_nodes, filter);
        }

        if link_dependencies {
            self.link_dependencies(&mut root_nodes, filter, self.use_full_path);
        }

        if link_dependents {
            let filter_path = if !self.use_full_path {
                filter
                    .expect("Filter must be set when using full path")
                    .split("::")
                    .collect::<Vec<&str>>()
            } else {
                // remove first element (filename) in case search is
                // scoped to single file
                filter
                    .expect("Filter must be set when using full path")
                    .split("::")
                    .skip(1)
                    .collect::<Vec<&str>>()
            };

            // Find the root and target node that matches the filter path
            match find_root_node(&root_nodes, &filter_path[0]) {
                Some(root_index) => {
                    // Check if main element is public
                    let rust_item = root_nodes[root_index]
                        .find_child_by_name(&filter_path[0])
                        .expect("Rust item not found")
                        .rtype().clone();

                    // TODO: if searching for sub-element (e.g. method of a
                    // struct, first check if sub-element is public. If sub
                    // is not public, we only need to check rust_item (self))

                    if rust_item.is_public() {
                        // If rust item is public, assume it can be called
                        // in any root node
                        for root_node in &mut root_nodes {
                            self.link_dependents(
                                root_node,
                                &rust_item,
                                filter_path.clone(),
                            );
                        }
                    } else {
                        // If the rust item is not public, traverse only
                        // through the node in which it was found
                        let root_node = &mut root_nodes[root_index];
                        self.link_dependents(root_node, &rust_item, filter_path);
                    }
                }
                None => panic!("No node found for path {:?}", filter_path),
            }
        }

        root_nodes
    }

    fn link_dependencies(
        &mut self,
        root_nodes: &mut Vec<RootNode>,
        filter: Option<&str>,
        use_full_path: bool,
    ) {
        for root in root_nodes {
            if let Some(filter_str) = filter {
                let config = PrintConfigBuilder::new()
                    .filter(Some(filter_str.to_string()))
                    .path(vec![root.file_path().relative_path().to_string()])
                    .is_linked(false)
                    .use_full_path(use_full_path)
                    .build();

                find_dependencies(root, &self.global_registry, &config);
            }
        }
    }

    fn link_dependents(
        &mut self,
        root_node: &mut RootNode,
        rust_item: &dyn Identifiable,
        filter_path: Vec<&str>,
    ) {
        // TODO: for each root node, find if the filter path matches
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
}

fn find_root_node(
    root_nodes: &[RootNode],
    target_name: &str,
) -> Option<usize> {
    for (i, root_node) in root_nodes.iter().enumerate() {
        if root_node.find_child_by_name(target_name).is_some() {
            return Some(i);
        }
    }
    None
}

fn find_node_by_path<'a>(
    node: &'a TreeNode,
    path: &[&str],
) -> Option<&'a TreeNode> {
    if path.is_empty() {
        return Some(node);
    }

    // Check if there are children and find the next node
    let next_node = node
        .children()
        .as_ref()?
        .iter()
        .find(|child| child.name() == path[0])?;

    // Recursive call with the found node and the rest of the path
    find_node_by_path(next_node, &path[1..])
}
