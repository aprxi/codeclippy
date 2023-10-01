pub(crate) mod file_visitor;
pub(crate) mod function_visitor;
pub(crate) mod helpers;
pub(crate) mod localfs;
pub(crate) mod map;
pub(crate) mod print_config;
pub(crate) mod registry;
pub(crate) mod tree;
pub(crate) mod types;
pub(crate) mod writers;

mod cli;
pub use cli::run_cli;
