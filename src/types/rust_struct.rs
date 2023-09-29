use std::fmt;

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
        write!(
            f,
            "{}struct {} {{\n",
            if self.visibility.to_string().is_empty() {
                String::from("")
            } else {
                format!("{} ", self.visibility)
            },
            self.name
        )?;

        for (field_name, field_type) in &self.fields {
            write!(f, "    {}: {},\n", field_name, field_type)?;
        }
        write!(f, "}}\n")?;

        // TODO: re-implement display for RustFunction,  and then re-use
        // it here -- use raw block, see dependencies.rs
        for method in &self.methods {
            write!(f, "{}", method)?;
        }
        write!(f, "\n")
    }
}
