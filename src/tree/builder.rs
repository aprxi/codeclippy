
use super::initialize::ChunkInitializer;
use crate::file_visitor::RustFileVisitor;
use crate::print_config::{PrintConfig, PrintConfigBuilder};
use crate::registry::GlobalRegistry;
use crate::tree::RootNode;

use super::dependencies::find_dependencies;

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

    pub fn initialize_chunks(
        &mut self,
        filter: Option<&str>,
        link_dependencies: bool,
    ) -> Vec<RootNode> {
        let mut chunks: Vec<RootNode> = self
            .visitors
            .iter_mut()
            .map(|visitor| {
                let mut initializer = ChunkInitializer::new(visitor);
                initializer.initialize_tree(&mut self.global_registry)
            })
            .collect();

        if !self.use_full_path {
            self.validate_chunks_for_conflicts(&chunks, filter);
        }

        if link_dependencies {
            self.link_dependencies(&mut chunks, filter, self.use_full_path);
        }
        chunks
    }

    pub fn add_dependencies(&self, root: &mut RootNode, config: &PrintConfig) {
        find_dependencies(root, &self.global_registry, config);
    }

    fn link_dependencies(
        &mut self,
        chunks: &mut Vec<RootNode>,
        filter: Option<&str>,
        use_full_path: bool,
    ) {
        for mut root in chunks {
            if let Some(filter_str) = filter {
                let config = PrintConfigBuilder::new()
                    .filter(Some(filter_str.to_string()))
                    .path(vec![root.file_path().relative_path().to_string()])
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
}
