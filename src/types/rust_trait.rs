use std::fmt::{Display, Formatter};

use crate::writers::ClippyWriter;
use super::{Identifiable, RustFunction, Visibility};
use crate::helpers::generate_id;

#[derive(Debug, Clone)]
pub struct RustTrait {
    id: String,
    pub visibility: Visibility,
    name: String,
    pub methods: Vec<RustFunction>,
    pub implementors: Vec<String>,
}

impl Identifiable for RustTrait {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn print(&self, writer: &mut Box<dyn ClippyWriter>) {
        let _ = write!(writer, "{}", self);
    }
}

impl RustTrait {
    pub fn new_with_data(
        name: String,
        visibility: Visibility,
        methods: Vec<RustFunction>,
        implementors: Vec<String>,
    ) -> Self {
        RustTrait {
            id: generate_id(&name),
            name,
            visibility,
            methods,
            implementors,
        }
    }
}

impl Display for RustTrait {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut methods = String::new();
        for method in &self.methods {
            methods.push_str(&format!("{}\n", method));
        }
        write!(
            f,
            "{}trait {} {{\n{}\n}}",
            self.visibility, self.name, methods
        )
    }
}
