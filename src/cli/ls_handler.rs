use crate::map::list_map;
use crate::writers::*;

pub fn handle_ls(ls_matches: &clap::ArgMatches) {
    let uri = ls_matches.get_one::<String>("uri").unwrap().to_string();
    let filter_name = ls_matches
        .get_one::<String>("query")
        .map(ToString::to_string);

    let show_dependencies =
        *ls_matches.get_one::<bool>("deps").unwrap_or(&false);

    let target_uri = ls_matches
        .get_one::<String>("clip")
        .map(ToString::to_string);

    let mut writer: Box<dyn ClippyWriter> = match target_uri.as_deref() {
        Some("clipboard://") => Box::new(ClipboardWriter::new()),
        None => Box::new(StdoutWriter::new()),
        _ => panic!("Target URI for --clip not supported"),
    };

    let maxdepth = ls_matches.get_one::<usize>("maxdepth");
    list_map(
        &uri,
        filter_name.as_deref(),
        &mut writer,
        show_dependencies,
        maxdepth.copied(),
    );
}
