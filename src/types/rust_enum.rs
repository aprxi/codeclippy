use std::fmt::{Display, Formatter};

use super::{Identifiable, RustFunction, Visibility};
use crate::helpers::generate_id;
use crate::writers::ClippyWriter;

#[derive(Debug, Clone)]
pub struct RustEnum {
    id: String,
    pub visibility: Visibility,
    name: String,
    pub variants: Vec<(String, Vec<String>)>,
    pub methods: Vec<RustFunction>,
}

impl Identifiable for RustEnum {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn print(&self, writer: &mut Box<dyn ClippyWriter>) {
        let _ = writeln!(writer, "{}", self);
    }
}

impl RustEnum {
    pub fn new_with_data(
        name: String,
        visibility: Visibility,
        variants: Vec<(String, Vec<String>)>,
        methods: Vec<RustFunction>,
    ) -> Self {
        RustEnum {
            id: generate_id(&name),
            name,
            visibility,
            variants,
            methods,
        }
    }
}

impl Display for RustEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut variants = String::new();
        for (variant, fields) in &self.variants {
            let mut fields = fields.join(", ");
            if !fields.is_empty() {
                fields = format!("({})", fields);
            }
            variants.push_str(&format!("{}{}\n", variant, fields));
        }
        let mut methods = String::new();
        for method in &self.methods {
            methods.push_str(&format!("{}\n", method));
        }
        write!(
            f,
            "{}enum {} {{\n{}\n}}\n{}",
            self.visibility, self.name, variants, methods
        )
    }
}
