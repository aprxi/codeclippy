use std::collections::HashMap;
use std::error::Error;
use std::fs;

use log;
use proc_macro2::LineColumn;
use quote::quote;

//use quote::ToTokens;
use super::TreeNode;

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
            println!(
                "{},{},{}",
                id,
                dependency.node().name(),
                dependency.source().unwrap_or(""),
            );

            if let Some(function) = &dependency.node().function {
                if let Some(block) = &function.block {
                    let tokens: proc_macro2::TokenStream = quote! { #block };
                    let group_span =
                        if let proc_macro2::TokenTree::Group(group) =
                            tokens.into_iter().next().unwrap()
                        {
                            group.span()
                        } else {
                            panic!("Expected a Group");
                        };

                    let start = group_span.start();
                    let end = group_span.end();

                    if let Some(source_path) = &dependency.source() {
                        if let Ok(extracted_code) = self
                            .extract_code_from_block(start, end, source_path)
                        {
                            println!("Extracted Code:\n{}", extracted_code);
                        }
                    }
                }
            }
        }
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
                    _ =>  line.to_string(),
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
