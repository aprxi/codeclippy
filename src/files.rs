use std::fs;
use std::path::Path;

pub fn map_files_in_directory(
    directory: &Path,
    maxdepth: Option<usize>,
) -> Vec<String> {
    let mut file_map = Vec::new();

    if directory.is_dir() && maxdepth.map(|depth| depth > 0).unwrap_or(true) {
        for entry in fs::read_dir(directory).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                file_map.extend(map_files_in_directory(
                    &path,
                    maxdepth.map(|depth| depth - 1),
                ));
            } else if path.extension().unwrap_or_default() == "rs" {
                file_map.push(path.display().to_string());
            }
        }
    }
    file_map
}
