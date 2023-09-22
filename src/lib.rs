pub(crate) mod file_visitor;
pub(crate) mod files;
pub(crate) mod function_visitor;
pub(crate) mod helpers;
pub(crate) mod map;
pub(crate) mod print_config;
pub(crate) mod registry;
pub(crate) mod rust_types;
pub(crate) mod tree;

mod cli;
pub use cli::run_cli;
