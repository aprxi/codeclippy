use std::path::Path;

use crate::file_visitor::RustFileVisitor;
use crate::files::map_files_in_directory;
use crate::print_config::PrintConfigBuilder;
use crate::tree::TreeBuilder;

pub fn source_map(
    directory: &str,
    filter: Option<&str>,
    maxdepth: Option<usize>,
) {
    let project_directory = Path::new(directory);
    let file_map = map_files_in_directory(project_directory, maxdepth);
    let file_paths: Vec<&str> = file_map.iter().map(AsRef::as_ref).collect();

    let visitors = RustFileVisitor::read_files(file_paths).unwrap();

    let debug = false;
    let use_full_path = false;
    let link_dependencies = true;

    let mut builder = TreeBuilder::new(visitors, use_full_path);
    let chunks = builder.initialize_chunks(filter, link_dependencies);

    for root in &chunks {
        println!("{}", root.filename());
        if let Some(filter_str) = filter {
            for child in root.children() {
                let child_config = PrintConfigBuilder::new()
                    .depth(1)
                    .filter(Some(filter_str.to_string()))
                    .path(vec![
                        root.filename().to_string(),
                        child.name().to_string(),
                    ])
                    .debug(debug)
                    .is_linked(false)
                    .use_full_path(builder.use_full_path())
                    .build();
                child.print(child_config);
            }
        }
        root.local_registry().print();
    }
}
