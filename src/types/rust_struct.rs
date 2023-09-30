use std::fmt;
use std::fmt::Write;

use super::format::pretty_code_fmt;
use super::{Identifiable, RustFunction, Visibility};

#[derive(Debug, Clone)]
pub struct RustStruct {
    id: String,
    name: String,
    visibility: Visibility,
    fields: Vec<(String, String)>,
    methods: Vec<RustFunction>,
}

impl RustStruct {
    pub fn new(id: &str, visibility: Visibility, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            visibility,
            fields: Vec::new(),
            methods: Vec::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn visibility(&self) -> &Visibility {
        &self.visibility
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn fields(&self) -> &Vec<(String, String)> {
        &self.fields
    }

    pub fn methods(&self) -> &Vec<RustFunction> {
        &self.methods
    }

    pub fn add_fields(&mut self, fields: Vec<(String, String)>) {
        self.fields.extend(fields);
    }

    pub fn add_methods(&mut self, methods: Vec<RustFunction>) {
        self.methods.extend(methods);
    }
}

impl Identifiable for RustStruct {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn print(&self) {
        println!("{}", self);
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

        if !self.methods.is_empty() {
            write!(&mut struct_str, "impl {} {{\n", self.name)?;
            for method in &self.methods {
                write!(&mut struct_str, "    {}\n", method)?;
            }
            write!(&mut struct_str, "}}\n")?;
        }

        // Pretty format the raw struct string representation.
        pretty_code_fmt(&mut struct_str);
        write!(f, "{}", struct_str)
    }
}
