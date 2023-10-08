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
    fields: Option<Vec<(String, String)>>,
    methods: Option<Vec<RustFunction>>,
}

impl RustStruct {
    pub fn new(id: &str, visibility: Visibility, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            visibility,
            fields: None,
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
        match &mut self.fields {
            Some(existing_fields) => existing_fields.extend(fields),
            None => self.fields = Some(fields),
        }
    }

    pub fn add_methods(&mut self, methods: Vec<RustFunction>) {
        match &mut self.methods {
            Some(existing_methods) => existing_methods.extend(methods),
            None => self.methods = Some(methods),
        }
    }

    pub fn struct_base_block_str(&self) -> String {
        let mut fields_str = String::new();
        let visibility = if self.visibility.to_string().is_empty() {
            String::from("")
        } else {
            format!("{} ", self.visibility)
        };
        write!(&mut fields_str, "{}struct {} {{\n", visibility, self.name)
            .unwrap();
        if let Some(fields) = &self.fields {
            for (field_name, field_type) in fields {
                write!(
                    &mut fields_str,
                    "    {}: {},\n",
                    field_name, field_type
                )
                .unwrap();
            }
        }
        write!(&mut fields_str, "}}\n").unwrap();
        pretty_code_fmt(&mut fields_str);
        fields_str
    }

    pub fn struct_impl_block_str(&self) -> String {
        let mut methods_str = String::new();
        if let Some(methods) = &self.methods {
            write!(&mut methods_str, "impl {} {{\n", self.name).unwrap();
            for method in methods {
                write!(&mut methods_str, "{}\n", method).unwrap();
            }
            write!(&mut methods_str, "}}\n").unwrap();
        }
        pretty_code_fmt(&mut methods_str);
        methods_str
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
        let mut struct_str = self.struct_base_block_str();
        struct_str.push_str(&self.struct_impl_block_str());

        write!(f, "{}", struct_str)
    }
}
