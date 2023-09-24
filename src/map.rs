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

    // load file contents
    let visitors = RustFileVisitor::read_files(file_paths).unwrap();

    // TODO: setting to true allows internal pub statements having the same
    // name. However, linking part still needs to be fixed for this to set to
    // true permanently
    let use_full_path = false;

    // TODO: currently default on based on filter used, we may want to make
    // this a cli option
    let link_dependencies = filter.is_some();

    let mut builder = TreeBuilder::new(visitors, use_full_path);
    let file_chunks = builder.initialize_chunks(filter, link_dependencies);

    for root in &file_chunks {
        let mut has_printed = false;
        for child in root.children() {
            let child_config = PrintConfigBuilder::new()
                .depth(1)
                .filter(filter.clone().map(|s| s.to_string()))
                .path(vec![
                    root.filename().to_string(),
                    child.name().to_string(),
                ])
                .is_linked(false)
                .use_full_path(builder.use_full_path())
                .build();

            let child_printed = child.print(child_config);
            has_printed = has_printed || child_printed;
        }

        // TODO: print only dependencies based on ID
        if has_printed {
            println!("Source: {}", root.filename());
            println!("Dependencies:");
            root.dependencies().print();
        }
    }
}
