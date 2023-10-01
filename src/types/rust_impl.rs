use std::fmt::{Display, Formatter};

use super::{Identifiable, RustFunction};
use crate::writers::ClippyWriter;

#[derive(Debug, Clone)]
pub struct RustImpl {
    pub id: String,
    pub for_type: String,
    pub functions: Vec<RustFunction>,
}

impl Identifiable for RustImpl {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.for_type
    }

    fn print(&self, writer: &mut Box<dyn ClippyWriter>) {
        let _ = writeln!(writer, "{}", self);
    }
}

impl Display for RustImpl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut functions = String::new();
        for function in &self.functions {
            functions.push_str(&format!("{}\n", function));
        }
        write!(f, "impl {} {{\n{}\n}}", self.for_type, functions)
    }
}
