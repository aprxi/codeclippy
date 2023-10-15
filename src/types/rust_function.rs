use std::collections::HashSet;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Write};
use std::{fmt, fs};

use proc_macro2::LineColumn;
use quote::quote;
use syn::visit::Visit;

use super::format::pretty_code_fmt;
use super::{Identifiable, Visibility};
use crate::function_visitor::FunctionCallVisitor;
use crate::helpers::generate_id;
use crate::localfs::FilePath;
use crate::writers::ClippyWriter;

#[derive(Clone)]
pub struct RustFunction {
    id: String,
    name: String,
    visibility: Visibility,
    inputs: Vec<(String, String)>,
    output: Option<String>,
    file_path: Option<FilePath>,
    block: Option<Box<syn::Block>>,
    methods: Option<Vec<RustFunction>>,
    instantiated_items: HashSet<String>,
}

impl RustFunction {
    pub fn new(visibility: Visibility, name: &str) -> Self {
        Self {
            id: generate_id(name),
            name: name.to_string(),
            visibility,
            inputs: Vec::new(),
            output: None,
            file_path: None,
            block: None,
            methods: None,
            instantiated_items: HashSet::new(),
        }
    }

    pub fn new_with_data(
        name: &str,
        visibility: Visibility,
        inputs: Vec<(String, String)>,
        output: Option<String>,
        file_path: Option<FilePath>,
        block: Option<Box<syn::Block>>,
    ) -> Self {
        Self {
            id: generate_id(name),
            name: name.to_string(),
            visibility,
            inputs,
            output,
            file_path,
            block,
            methods: None,
            instantiated_items: HashSet::new(),
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

    pub fn instantiated_items(&self) -> &HashSet<String> {
        &self.instantiated_items
    }

    pub fn signature_str(&self) -> String {
        let mut signature = String::new();

        // Write function signature
        write!(
            &mut signature,
            "{}fn {}(",
            if self.visibility().to_string().is_empty() {
                String::from("")
            } else {
                format!("{} ", self.visibility)
            },
            self.name
        )
        .unwrap();

        // Write inputs
        let inputs: Vec<String> = self
            .inputs
            .iter()
            .map(|(name, typ)| {
                if typ.is_empty() {
                    name.clone() // if type is empty, just clone the name
                } else {
                    format!("{}: {}", name, typ) // else, format with ":"
                }
            })
            .collect();
        write!(&mut signature, "{})", inputs.join(", ")).unwrap();

        // Write output type
        if let Some(output) = &self.output {
            write!(&mut signature, "-> {}", output).unwrap();
        }

        // Temporarily add an empty body and format, so we can parse the
        // signature through pretty_code_fmt separately
        signature.push_str(" {}\n");
        pretty_code_fmt(&mut signature);

        // Remove the temporary (emtpy) body to get clean signature
        signature.trim_end_matches(" {}\n").to_string()
    }

    fn body_str(&self) -> String {
        let mut body = String::new();

        // Write function body
        if let Some(file_path) = &self.file_path {
            let real_path = file_path.real_path();
            if let Some(ref block) = self.block {
                match print_extracted_code(block, &real_path) {
                    Ok(code) => {
                        pretty_code_fmt(&mut body);
                        write!(&mut body, "{}\n", code).unwrap();
                    }
                    Err(_) => {
                        write!(&mut body, "Error extracting code").unwrap()
                    }
                }
            } else {
                // no function body
                write!(&mut body, ";").unwrap();
            }
        }
        body
    }
    pub fn function_block_str(&self) -> String {
        let mut full_function = String::new();

        // Concatenate signature and body
        full_function.push_str(&self.signature_str());
        full_function.push_str(&self.body_str());
        full_function
    }
}

impl Identifiable for RustFunction {
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

impl Debug for RustFunction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "RustFunction {{")?;
        writeln!(f, "  id: {},", self.id)?;
        writeln!(f, "  visibility: {:?},", self.visibility)?;
        writeln!(f, "  name: {},", self.name)?;
        writeln!(f, "  inputs: {:?},", self.inputs)?;
        writeln!(f, "  output: {:?},", self.output)?;
        writeln!(f, "  block: {},", self.block.is_some())?;
        writeln!(f, "  methods: [")?;
        for func in self.methods.as_deref().unwrap_or_default() {
            writeln!(f, "    {},", func)?;
        }
        writeln!(f, "  ],")?;
        writeln!(f, "  instantiated_items: {:?},", self.instantiated_items)?;
        write!(f, "}}")
    }
}

impl Display for RustFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.function_block_str())
    }
}

impl RustFunction {
    pub fn extract_function_body(&mut self) {
        if let Some(ref block) = self.block {
            let mut body_visitor = FunctionCallVisitor::default();
            body_visitor.visit_block(block);
            match &mut self.methods {
                Some(methods) => methods.extend(body_visitor.functions),
                None => self.methods = Some(body_visitor.functions),
            }
            self.instantiated_items
                .extend(body_visitor.instantiated_items);
        }
    }
}

fn print_extracted_code(
    block: &syn::Block,
    source_path: &str,
) -> Result<String, std::fmt::Error> {
    let tokens: proc_macro2::TokenStream = quote! { #block };
    let group_span = tokens
        .into_iter()
        .next()
        .and_then(|token| match token {
            proc_macro2::TokenTree::Group(group) => Some(group.span()),
            _ => None,
        })
        .expect("Expected a Group");

    let start = group_span.start();
    let end = group_span.end();

    if let Ok(extracted_code) = extract_code_from_block(start, end, source_path)
    {
        Ok(extracted_code)
    } else {
        Err(std::fmt::Error)
    }
}

fn extract_code_from_block(
    start: LineColumn,
    end: LineColumn,
    source_path: &str,
) -> Result<String, Box<dyn Error>> {
    let file_content = fs::read_to_string(source_path)?;
    let lines: Vec<&str> = file_content.lines().collect();

    // subtract 1 to convert to 0-based indexing
    let extracted_code: String = lines[(start.line - 1)..=(end.line - 1)]
        .iter()
        .enumerate()
        .map(|(idx, line)| {
            let is_first_line = idx == 0;
            let is_last_line = idx == (end.line - start.line);

            match (is_first_line, is_last_line, line.is_empty()) {
                (_, _, true) => String::new(),
                // single line
                (true, true, _) => {
                    let end_column = std::cmp::min(end.column, line.len());
                    line[start.column..end_column].to_string()
                }
                // first line
                (true, false, _) => line[start.column..].to_string(),
                // last line
                (false, true, _) => {
                    let end_column = std::cmp::min(end.column, line.len());
                    line[..end_column].to_string()
                }
                // middle lines
                _ => line.to_string(),
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    Ok(extracted_code)
}
