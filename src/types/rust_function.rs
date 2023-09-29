use std::collections::HashSet;
use std::error::Error;
use std::fmt::Write;
use std::{fmt, fs};

use proc_macro2::LineColumn;
use quote::quote;
use syn::visit::Visit;

use super::Visibility;
use super::format::pretty_code_fmt;
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

impl fmt::Display for RustFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut formatted = String::new();

        // Write function signature
        write!(
            &mut formatted,
            "{}fn {}(",
            if self.visibility.to_string().is_empty() {
                String::from("")
            } else {
                format!("{} ", self.visibility)
            },
            self.name
        )?;

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
        write!(&mut formatted, "{})", inputs.join(", "))?;

        // Write output type
        if let Some(output) = &self.output {
            write!(&mut formatted, "-> {}", output)?;
        }
        write!(&mut formatted, "\n")?;

        // Write function body
        if let Some(source) = &self.source {
            match print_extracted_code(&self.block.as_ref().unwrap(), &source) {
                Ok(code) => {
                    write!(&mut formatted, "{}\n", code)?;
                    pretty_code_fmt(&mut formatted);
                }
                Err(_) => write!(&mut formatted, "Error extracting code")?,
            }
        }
        // Write the final formatted string to the formatter
        write!(f, "{}", formatted)
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
