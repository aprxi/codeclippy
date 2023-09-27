use std::collections::HashMap;
use std::error::Error;
use std::fs;

use log;
use proc_macro2::LineColumn;
use quote::quote;

use super::TreeNode;
use crate::file_visitor::NodeKind;
use crate::rust_types::RustStruct;

pub struct Dependencies {
    items_by_id: HashMap<String, Dependency>,
}

impl Default for Dependencies {
    fn default() -> Self {
        Dependencies {
            items_by_id: HashMap::new(),
        }
    }
}

impl Dependencies {
    pub fn register_item(
        &mut self,
        id: String,
        node: TreeNode,
        source: Option<&str>,
    ) {
        log::debug!("adding dependency: {}", node.clone().name());
        let dependency = Dependency {
            node,
            source: source.map(|s| s.to_string()),
        };
        self.items_by_id.insert(id.clone(), dependency);
    }

    pub fn len(&self) -> usize {
        self.items_by_id.len()
    }

    pub fn print(&self) {
        for (id, dependency) in &self.items_by_id {
            self.print_dependency(id, dependency);
        }
    }

    fn print_dependency(&self, id: &String, dependency: &Dependency) {
        println!(
            "{},{},{}",
            id,
            dependency.node().name(),
            dependency.source().unwrap_or(""),
        );

        match dependency.node().kind() {
            NodeKind::Function => {
                let _ = dependency.node().function.as_ref().map_or(
                    (),
                    |function| {
                        function.block.as_ref().map_or((), |block| {
                            self.print_extracted_code(block, &dependency);
                        })
                    },
                );
            }
            NodeKind::Struct => {
                let rust_struct: &RustStruct =
                    dependency.node().rust_struct.as_ref().unwrap();
                println!("{}", rust_struct);
            }
            _ => {
                log::error!(
                    "not supported yet: {:?}",
                    dependency.node().kind()
                );
            }
        }
    }

    fn print_extracted_code(
        &self,
        block: &syn::Block,
        dependency: &Dependency,
    ) {
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

        let _ = dependency.source().map_or((), |source_path| {
            if let Ok(extracted_code) =
                self.extract_code_from_block(start, end, source_path)
            {
                println!("Extracted Code:\n{}", extracted_code);
            }
        });
    }

    fn extract_code_from_block(
        &self,
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
}

pub struct Dependency {
    node: TreeNode,
    source: Option<String>,
}

impl Dependency {
    pub fn node(&self) -> &TreeNode {
        &self.node
    }
    pub fn source(&self) -> Option<&str> {
        self.source.as_ref().map(|s| s.as_str())
    }
}
