use std::path::Path;

use crate::file_visitor::RustFileVisitor;
use crate::localfs::map_files_in_directory;
use crate::tree::TreeBuilder;
use crate::writers::{BufferedWriter, ClippyWriter};

pub fn list_map(
    directory: &str,
    filter: Option<&str>,
    writer: &mut Box<dyn ClippyWriter>,
    show_dependencies: bool,
    maxdepth: Option<usize>,
) {
    let base_directory = Path::new(directory);
    let file_map = map_files_in_directory(base_directory, maxdepth);
    // collect relative paths from base_directory
    let file_paths: Vec<&str> = file_map.iter().map(AsRef::as_ref).collect();

    // load file contents
    let visitors =
        RustFileVisitor::read_files(base_directory, file_paths).unwrap();

    let use_full_path = if let Some(filter) = filter {
        let first_element = filter.split("::").next().unwrap_or("");
        let real_path = base_directory.join(first_element);
        real_path.exists()
    } else {
        false
    };

    let link_dependencies = show_dependencies && filter.is_some();
    let mut builder = TreeBuilder::new(visitors, use_full_path);
    let file_chunks = builder.initialize_chunks(filter, link_dependencies);

    for root in &file_chunks {
        let mut buffered_writer: Box<dyn ClippyWriter> =
            Box::new(BufferedWriter::new());
        root.print(&mut buffered_writer, filter, use_full_path);

        if let Some(buffered_data) = buffered_writer.get_buffer() {
            let buffered_str = String::from_utf8_lossy(buffered_data);
            let _ = writeln!(
                writer,
                "@{}:\n{}",
                root.file_path().relative_path(),
                buffered_str
            );
        }

        if show_dependencies && root.dependencies().len() > 0 {
            root.dependencies().print(writer);
        }
    }
}
