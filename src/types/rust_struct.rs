use std::fmt;
use std::fmt::Write;

use super::format::pretty_code_fmt;
use super::{Identifiable, RustFunction, Visibility};
use crate::writers::ClippyWriter;

#[derive(Debug, Clone)]
pub struct RustStruct {
    id: String,
    name: String,
    visibility: Visibility,
    fields: Vec<(String, String)>,
    methods: Option<Vec<RustFunction>>,
}

impl RustStruct {
    pub fn new(id: &str, visibility: Visibility, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            visibility,
            fields: Vec::new(),
            methods: None,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn methods(&self) -> Option<&Vec<RustFunction>> {
        self.methods.as_ref()
    }

    pub fn add_fields(&mut self, fields: Vec<(String, String)>) {
        self.fields.extend(fields);
    }

    pub fn add_methods(&mut self, methods: Vec<RustFunction>) {
        match &mut self.methods {
            Some(existing_methods) => existing_methods.extend(methods),
            None => self.methods = Some(methods),
        }
    }
}

impl Identifiable for RustStruct {
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

impl fmt::Display for RustStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut struct_str = String::new();

        // Constructing the raw struct string representation
        let visibility = if self.visibility.to_string().is_empty() {
            String::from("")
        } else {
            format!("{} ", self.visibility)
        };
        write!(&mut struct_str, "{}struct {} {{\n", visibility, self.name)?;

        for (field_name, field_type) in &self.fields {
            write!(&mut struct_str, "    {}: {},\n", field_name, field_type)?;
        }
        write!(&mut struct_str, "}}\n")?;

        if let Some(methods) = &self.methods {
            write!(&mut struct_str, "impl {} {{\n", self.name)?;
            for method in methods {
                write!(&mut struct_str, "{}\n", method)?;
            }
            write!(&mut struct_str, "}}\n")?;
        }

        // Pretty format the raw struct string representation.
        pretty_code_fmt(&mut struct_str);
        write!(f, "{}", struct_str)
    }
}
