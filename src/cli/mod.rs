use std::env;

use clap::Command;

mod ls;
mod ls_handler;
use ls::*;

const PROGRAM_NAME: &str = "codeclippy";

pub fn run_cli(args: Vec<String>) {
    env_logger::init();
    let app = Command::new(PROGRAM_NAME)
        .version(env!("CARGO_PKG_VERSION"))
        .arg_required_else_help(true)
        .about(format!(
            "List code objects\n\nExample:\n {} ls \
             src/",
            PROGRAM_NAME
        ))
        .subcommand(ls_subcommand());

    let matches = app.try_get_matches_from(args).unwrap_or_else(|e| {
        e.exit();
    });

    match matches.subcommand() {
        Some(("ls", matches)) => {
            handle_ls(matches);
        }
        _ => {
            eprintln!("No valid subcommand provided");
        }
    }
}
