use std::fmt;
use std::fmt::{Display, Write};

use super::format::pretty_code_fmt;
use super::{Identifiable, RustFunction, Visibility};
use crate::helpers::generate_id;
use crate::writers::ClippyWriter;

#[derive(Debug, Clone)]
pub struct RustTrait {
    id: String,
    visibility: Visibility,
    name: String,
    methods: Option<Vec<RustFunction>>,
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

    fn visibility(&self) -> &Visibility {
        &self.visibility
    }
}

impl RustTrait {
    pub fn new_with_data(
        name: String,
        visibility: Visibility,
        methods: Vec<RustFunction>,
    ) -> Self {
        RustTrait {
            id: generate_id(&name),
            name,
            visibility,
            methods: Some(methods),
        }
    }

    pub fn methods(&self) -> Option<&Vec<RustFunction>> {
        self.methods.as_ref()
    }

    pub fn trait_block_str(&self) -> String {
        let mut trait_str = String::new();
        let visibility = if self.visibility.to_string().is_empty() {
            String::from("")
        } else {
            format!("{} ", self.visibility)
        };
        write!(&mut trait_str, "{}trait {} {{\n", visibility, self.name)
            .unwrap();
        if let Some(methods) = &self.methods {
            for method in methods {
                write!(&mut trait_str, "    {}", method).unwrap();
            }
        }
        write!(&mut trait_str, "}}\n").unwrap();
        pretty_code_fmt(&mut trait_str);
        trait_str
    }
}

impl Display for RustTrait {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let trait_str = self.trait_block_str();
        write!(f, "{}", trait_str)
    }
}
