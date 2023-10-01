use clap::{value_parser, Arg, ArgAction, Command};

pub use super::ls_handler::handle_ls;

pub fn ls_subcommand() -> Command {
    Command::new("ls")
        .about("List code objects")
        .arg(
            Arg::new("uri")
                .index(1)
                .required(true)
                .help("Path to code files. E.g. src/"),
        )
        .arg(Arg::new("query").long("query").short('q').help(
            "Search or filter objects based on a pattern. E.g. 'foo', \
             'foo.*', '.*bar'",
        ))
        .arg(
            Arg::new("deps")
                .long("deps")
                .short('d')
                .help("Include dependencies in the output.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("clip")
                .long("clip")
                .value_name("uri")
                .num_args(0..=1)
                .default_missing_value("clipboard://")
                .help(
                    "Copy the output to the clipboard, or the specified URI \
                     if provided.",
                ),
        )
        .arg(
            Arg::new("maxdepth")
                .value_parser(value_parser!(usize))
                .long("maxdepth")
                .help("Set max depth of subdirectories to traverse"),
        )
}
