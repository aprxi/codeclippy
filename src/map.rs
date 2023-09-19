use std::path::Path;

use crate::file_visitor::RustFileVisitor;
use crate::files::map_files_in_directory;
use crate::populate::{link_missing_structs, populate_tree};
use crate::print_config::PrintConfigBuilder;
use crate::registry::GlobalRegistry;
use crate::tree::RootNode;

pub fn source_map(
    directory: &str,
    filter: Option<&str>,
    maxdepth: Option<usize>,
) {
    let project_directory = Path::new(directory);
    let file_map = map_files_in_directory(project_directory, maxdepth);
    let file_paths: Vec<&str> = file_map.iter().map(AsRef::as_ref).collect();

    let mut visitors = RustFileVisitor::read_files(file_paths).unwrap();
    let mut global_registry = GlobalRegistry::default();

    // TODO: derive from environment
    let debug = false;

    // TODO: derive from filter input
    // default to false for now
    let use_full_path = false;

    // Populate the trees
    let trees: Vec<RootNode> = visitors
        .iter_mut()
        .map(|visitor| populate_tree(visitor, &mut global_registry))
        .collect();

    // if filter is used on non full path search, check for possible
    // duplicate matches -- we can only return one single match
    if !use_full_path && filter.is_some() {
        let filter_str = filter.unwrap();

        let first_component = filter_str.split("::").next().unwrap();
        let potential_conflicts: Vec<_> = trees
            .iter()
            .filter(|tree| tree.has_node_named(first_component))
            .collect();

        if potential_conflicts.len() > 1 {
            panic!(
                "Potential conflict found. More than one tree has a node \
                 named {}. Please specify a more specific filter.",
                first_component
            );
        }
    }

    for mut root in trees {
        println!("{}", root.filename());
        if filter.is_some() {
            let config = PrintConfigBuilder::new()
                .depth(0)
                .filter(filter.clone().map(|s| s.to_string()))
                .path(vec![])
                .debug(debug)
                .is_linked(false)
                .use_full_path(use_full_path)
                .build();

            link_missing_structs(&mut root, &mut global_registry, &config);
        }

        for child in root.children() {
            let child_config = PrintConfigBuilder::new()
                .depth(0)
                .filter(filter.clone().map(|s| s.to_string()))
                .path(vec![child.name().to_string()])
                .debug(debug)
                .is_linked(false)
                .use_full_path(use_full_path)
                .build();
            child.print(child_config);
        }

        root.local_registry().print();
    }
}
