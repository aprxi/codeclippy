use std::fmt;
use std::fmt::Write;

use super::format::pretty_code_fmt;
use super::{RustFunction, Visibility};

#[derive(Debug, Clone)]
pub struct RustStruct {
    pub id: String,
    pub visibility: Visibility,
    pub name: String,
    pub fields: Vec<(String, String)>,
    pub methods: Vec<RustFunction>,
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
