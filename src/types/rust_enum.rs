use std::fmt;
use std::fmt::{Display, Formatter, Write};

use super::format::pretty_code_fmt;
use super::{Identifiable, RustFunction, Visibility};
use crate::helpers::generate_id;
use crate::writers::ClippyWriter;

#[derive(Debug, Clone)]
pub struct RustEnum {
    id: String,
    visibility: Visibility,
    name: String,
    variants: Vec<(String, Vec<String>)>,
    methods: Option<Vec<RustFunction>>,
}

impl Identifiable for RustEnum {
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

impl RustEnum {
    pub fn new_with_data(
        name: String,
        visibility: Visibility,
        variants: Vec<(String, Vec<String>)>,
    ) -> Self {
        RustEnum {
            id: generate_id(&name),
            name,
            visibility,
            variants,
            methods: None,
        }
    }

    pub fn add_methods(&mut self, methods: Vec<RustFunction>) {
        match &mut self.methods {
            Some(existing_methods) => existing_methods.extend(methods),
            None => self.methods = Some(methods),
        }
    }

    pub fn methods(&self) -> Option<&Vec<RustFunction>> {
        self.methods.as_ref()
    }
}

impl Display for RustEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut enum_str = String::new();

        let visibility = if self.visibility.to_string().is_empty() {
            String::from("")
        } else {
            format!("{} ", self.visibility)
        };
        write!(&mut enum_str, "{}enum {} {{\n", visibility, self.name)?;

        for (variant, fields) in &self.variants {
            let fields_str = if fields.is_empty() {
                String::from("")
            } else {
                format!("({})", fields.join(", "))
            };
            write!(&mut enum_str, "    {}{},\n", variant, fields_str)?;
        }
        write!(&mut enum_str, "}}\n")?;

        match &self.methods {
            Some(methods) => {
                write!(&mut enum_str, "impl {} {{\n", self.name)?;
                for method in methods {
                    write!(&mut enum_str, "{}\n", method)?;
                }
                write!(&mut enum_str, "}}\n")?;
            }
            None => {}
        }

        pretty_code_fmt(&mut enum_str);
        write!(f, "{}", enum_str)
    }
}
