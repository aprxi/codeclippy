use std::fmt::{Display, Formatter};

use super::{Identifiable, RustFunction};
use crate::helpers::generate_id;
use crate::writers::ClippyWriter;

#[derive(Debug, Clone)]
pub struct RustImpl {
    id: String,
    pub for_type: String,
    pub functions: Vec<RustFunction>,
}

impl RustImpl {
    pub fn new_with_data(
        for_type: String,
        functions: Vec<RustFunction>,
    ) -> Self {
        RustImpl {
            id: generate_id(&for_type),
            for_type,
            functions,
        }
    }
}

impl Identifiable for RustImpl {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.for_type
    }

    fn print(&self, writer: &mut Box<dyn ClippyWriter>) {
        let _ = write!(writer, "{}", self);
    }

    fn visibility(&self) -> bool {
        true // TODO: validate if safe to assume impl is always public
             // if cant assume, should probably convert to Option<bool>
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
