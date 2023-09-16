use clap::{value_parser, Arg, Command};

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
        .arg(
            Arg::new("name").long("name").short('n').help(
                "Filter objects based on name. E.g. 'foo', 'foo.*', '.*bar'",
            ),
        )
        .arg(
            Arg::new("maxdepth")
                .value_parser(value_parser!(usize))
                .long("maxdepth")
                .help("Set max depth of subdirectories to traverse"),
        )
}
