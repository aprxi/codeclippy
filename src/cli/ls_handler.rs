use crate::map::source_map;

pub fn handle_ls(ls_matches: &clap::ArgMatches) {
    let uri = ls_matches.get_one::<String>("uri").unwrap().to_string();
    let filter_name = ls_matches
        .get_one::<String>("name")
        .map(ToString::to_string);

    let maxdepth = ls_matches.get_one::<usize>("maxdepth");
    println!("maxdepth: {:?}", maxdepth);
    source_map(&uri, filter_name.as_deref(), maxdepth.copied());
}
