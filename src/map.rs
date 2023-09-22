use std::path::Path;

use crate::file_visitor::RustFileVisitor;
use crate::files::map_files_in_directory;
use crate::tree::TreeBuilder;
use crate::print_config::PrintConfigBuilder;
use crate::tree::RootNode;

pub fn source_map(directory: &str, filter: Option<&str>, maxdepth: Option<usize>) {
    let project_directory = Path::new(directory);
    let file_map = map_files_in_directory(project_directory, maxdepth);
    let file_paths: Vec<&str> = file_map.iter().map(AsRef::as_ref).collect();

    let mut visitors = RustFileVisitor::read_files(file_paths).unwrap();

    let debug = false;
    let use_full_path = false;

    let mut tree_builder = TreeBuilder::new();
    let trees: Vec<RootNode> = visitors
        .iter_mut()
        .map(|visitor| tree_builder.build_tree(visitor))
        .collect();

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
            let mut config = PrintConfigBuilder::new()
                .depth(0)
                .filter(filter.clone().map(|s| s.to_string()))
                .path(vec![root.filename().to_string()])
                .debug(debug)
                .is_linked(false)
                .use_full_path(use_full_path)
                .build();

            tree_builder.link_missing_structs(&mut root, &mut config);
        }

        for child in root.children() {
            let child_config = PrintConfigBuilder::new()
                .depth(1)
                .filter(filter.clone().map(|s| s.to_string()))
                .path(vec![
                    root.filename().to_string(),
                    child.name().to_string(),
                ])
                .debug(debug)
                .is_linked(false)
                .use_full_path(use_full_path)
                .build();
            child.print(child_config);
        }

        root.local_registry().print();
    }
}

