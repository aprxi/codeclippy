use std::fmt::{Display, Formatter};

use super::{Identifiable, RustFunction, Visibility};
use crate::helpers::generate_id;
use crate::writers::ClippyWriter;

#[derive(Debug, Clone)]
pub struct RustImpl {
    id: String,
    pub for_type: String,
    pub methods: Option<Vec<RustFunction>>,
}

impl RustImpl {
    pub fn new_with_data(for_type: String, methods: Vec<RustFunction>) -> Self {
        RustImpl {
            id: generate_id(&for_type),
            for_type,
            methods: Some(methods),
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

    fn visibility(&self) -> &Visibility {
        &Visibility::Public // assume Public
    }
}

impl Display for RustImpl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut methods_str = String::new();
        if let Some(methods) = &self.methods {
            for method in methods {
                methods_str.push_str(&format!("{}\n", method));
            }
        }
        write!(f, "impl {} {{\n{}\n}}", self.for_type, methods_str)
    }
}
