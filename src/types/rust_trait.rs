use std::fmt::{Display, Formatter};
use std::fmt;
use std::fmt::Write;

use super::{Identifiable, RustFunction, Visibility};
use crate::helpers::generate_id;
use crate::writers::ClippyWriter;
use super::format::pretty_code_fmt;

#[derive(Debug, Clone)]
pub struct RustTrait {
    id: String,
    visibility: Visibility,
    name: String,
    methods: Vec<RustFunction>,
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
    ) -> Self {
        RustTrait {
            id: generate_id(&name),
            name,
            visibility,
            methods,
        }
    }

    pub fn methods(&self) -> &Vec<RustFunction> {
        &self.methods
    }
}

impl Display for RustTrait {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut trait_str = String::new();

        let visibility = if self.visibility.to_string().is_empty() {
            String::from("")
        } else {
            format!("{} ", self.visibility)
        };
        write!(&mut trait_str, "{}trait {} {{\n", visibility, self.name)?;

        for method in &self.methods {
            write!(&mut trait_str, "    {}", method)?;
        }
        write!(&mut trait_str, "}}\n")?;
        pretty_code_fmt(&mut trait_str);
        write!(f, "{}", trait_str)
    }
}
