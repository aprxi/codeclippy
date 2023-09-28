use std::collections::HashSet;
use std::fmt;
use std::fs;
use syn::visit::Visit;

use proc_macro2::LineColumn;
use quote::quote;

use crate::function_visitor::FunctionCallVisitor;

#[derive(Clone)]
pub struct RustFunction {
    pub id: String,
    pub visibility: Visibility,
    pub name: String,
    pub inputs: Vec<(String, String)>,
    pub output: Option<String>,
    pub source: Option<String>,
    pub block: Option<Box<syn::Block>>,
    pub functions: Vec<RustFunction>,
    pub instantiated_items: HashSet<String>,
}

impl std::fmt::Debug for RustFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "RustFunction {{")?;
        writeln!(f, "  id: {},", self.id)?;
        writeln!(f, "  visibility: {:?},", self.visibility)?;
        writeln!(f, "  name: {},", self.name)?;
        writeln!(f, "  inputs: {:?},", self.inputs)?;
        writeln!(f, "  output: {:?},", self.output)?;
        writeln!(f, "  block: {},", self.block.is_some())?;
        writeln!(f, "  functions: [")?;
        for func in &self.functions {
            writeln!(f, "    {},", func)?;
        }
        writeln!(f, "  ],")?;
        writeln!(f, "  instantiated_items: {:?},", self.instantiated_items)?;
        write!(f, "}}")
    }
}

//impl fmt::Display for RustFunction {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        writeln!(f, "RustFunction {{")?;
//        writeln!(f, "  id: {},", self.id)?;
//        writeln!(f, "  visibility: {:?},", self.visibility)?;
//        writeln!(f, "  name: {},", self.name)?;
//        writeln!(f, "  inputs: {:?},", self.inputs)?;
//        writeln!(f, "  output: {:?},", self.output)?;
//        writeln!(f, "  functions: [")?;
//        for func in &self.functions {
//            writeln!(f, "    {},", func)?;
//        }
//        writeln!(f, "  ],")?;
//        writeln!(f, "  instantiated_items: {:?},", self.instantiated_items)?;
//        write!(f, "}}")
//    }
//}

impl fmt::Display for RustFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write function signature
        write!(f, "fn {}(", self.name)?;

        // Write inputs
        let inputs: Vec<String> = self
            .inputs
            .iter()
            .map(|(name, typ)| format!("{}: {}", name, typ))
            .collect();
        write!(f, "{}", inputs.join(", "))?;

        // Write output type
        if let Some(output) = &self.output {
            write!(f, " -> {} ", output)?;
        }

        write!(f, "{{")?;

        // Write function body
        if let Some(block) = &self.block {
            let tokens: proc_macro2::TokenStream = quote! { #block };
            if let Some(group_span) =
                tokens.into_iter().next().and_then(|token| match token {
                    proc_macro2::TokenTree::Group(group) => Some(group.span()),
                    _ => None,
                })
            {
                let start = group_span.start();
                let end = group_span.end();

                if let Some(source) = &self.source {
                    if let Ok(extracted_code) =
                        self.extract_code_from_block(&start, &end, source)
                    {
                        writeln!(
                            f,
                            "\n    {}",
                            extracted_code.replace("\n", "\n    ").trim_start()
                        )?;
                    }
                }
            }
        } else {
            writeln!(f, "\n    // No body")?;
        }

        writeln!(f, "}}")
    }
}


impl RustFunction {
    fn extract_code_from_block(
        &self,
        start: &proc_macro2::LineColumn,
        end: &proc_macro2::LineColumn,
        source_path: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let file_content = fs::read_to_string(source_path)?;
        let lines: Vec<&str> = file_content.lines().collect();

        let extracted_code: String = lines[(start.line - 1)..=(end.line - 1)]
            .iter()
            .enumerate()
            .map(|(idx, line)| {
                let is_first_line = idx == 0;
                let is_last_line = idx == (end.line - start.line);

                match (is_first_line, is_last_line, line.is_empty()) {
                    (_, _, true) => String::new(),
                    (true, true, _) => {
                        let end_column = std::cmp::min(end.column, line.len());
                        line[start.column..end_column].to_string() // single line
                    }
                    (true, false, _) => line[start.column..].to_string(), // first line
                    (false, true, _) => {
                        let end_column = std::cmp::min(end.column, line.len());
                        line[..end_column].to_string() // last line
                    }
                    _ => line.to_string(), // middle lines
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        Ok(extracted_code)
    }
}


impl RustFunction {
    pub fn extract_function_body(&mut self) {
        if let Some(ref block) = self.block {
            let mut body_visitor = FunctionCallVisitor::default();
            body_visitor.visit_block(block);

            self.functions.extend(body_visitor.functions);
            self.instantiated_items
                .extend(body_visitor.instantiated_items);
        }
    }
}

#[derive(Debug, Clone)]
pub struct RustStruct {
    pub id: String,
    pub visibility: Visibility,
    pub name: String,
    pub fields: Vec<(String, String)>,
    pub methods: Vec<RustFunction>,
}

#[derive(Debug, Clone)]
pub struct RustEnum {
    pub id: String,
    pub visibility: Visibility,
    pub name: String,
    pub variants: Vec<(String, Vec<String>)>,
    pub methods: Vec<RustFunction>,
}

#[derive(Debug, Clone)]
pub struct RustTrait {
    pub id: String,
    pub visibility: Visibility,
    pub name: String,
    pub methods: Vec<RustFunction>,
    pub implementors: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct RustImpl {
    pub id: String,
    pub for_type: String,
    pub functions: Vec<RustFunction>,
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

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Restricted,
    Inherited,
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Visibility::Public => write!(f, "pub"),
            _ => write!(f, ""),
        }
    }
}
