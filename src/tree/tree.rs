use std::collections::HashSet;

use crate::file_visitor::NodeKind;
use crate::print_config::{PrintConfig, PrintConfigBuilder};
use crate::rust_types::RustFunction;

#[derive(Debug, Clone)]
pub struct TreeNode {
    name: String,
    kind: NodeKind,
    pub children: Vec<TreeNode>,
    pub fields: Option<Vec<(String, String)>>,
    pub function: Option<RustFunction>,
    pub link: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(name: String, kind: NodeKind) -> Self {
        TreeNode {
            name,
            kind,
            children: Vec::new(),
            fields: None,
            function: None,
            link: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> &NodeKind {
        &self.kind
    }

    pub fn add_child(&mut self, child: TreeNode) {
        self.children.push(child);
    }

    pub fn print(&self, config: PrintConfig) {
        let mut printed_methods = HashSet::new();

        if !self.should_print(&config) {
            // despite not printing this node, it may still have children
            self.print_children(&config, &mut printed_methods);
            return;
        }
        if let Some(linked_node) = &self.link {
            self.print_linked_node(linked_node, &config);
            return;
        }

        if config.debug() {
            let full_path = config.path().join("::");
            custom_println(config.depth(), &format!("[{}]", full_path));
        }

        match &self.kind {
            NodeKind::Function => self.print_function(&config),
            NodeKind::Struct => {
                self.print_struct(&config, &mut printed_methods);
            }
            NodeKind::Enum => self.print_enum(&config),
            NodeKind::Trait => self.print_trait(&config),
            NodeKind::Variant => self.print_variant(&config),
        }

        self.print_children(&config, &mut printed_methods);
    }

    fn print_children(
        &self,
        config: &PrintConfig,
        printed_methods: &mut HashSet<String>,
    ) {
        for child in &self.children {
            if printed_methods.contains(&child.name) {
                continue;
            }

            let child_depth = match (config.depth(), &self.kind, &child.kind) {
                (0, _, _) => 0,
                (_, &NodeKind::Function, &NodeKind::Function) => config.depth(),
                _ => config.depth() + 1,
            };

            let mut child_config = config.clone();
            child_config.set_depth(child_depth);
            child_config.add_to_path(child.name.clone());
            child.print(child_config);

            if matches!(child.kind, NodeKind::Function) {
                printed_methods.insert(child.name.clone());
            }
        }
    }

    fn print_struct(
        &self,
        config: &PrintConfig,
        printed_methods: &mut HashSet<String>,
    ) {
        custom_println(config.depth(), &format!("{} (Struct)", self.name));

        if let Some(fields) = &self.fields {
            if !fields.is_empty() {
                custom_println(config.depth() + 1, "Fields:");
                for (name, type_) in fields {
                    custom_println(
                        config.depth() + 2,
                        &format!("{}: {}", name, type_),
                    );
                }
            }
        }
        if self
            .children
            .iter()
            .any(|child| matches!(child.kind, NodeKind::Function))
        {
            custom_println(config.depth() + 1, "Methods:");
            for child in &self.children {
                if matches!(child.kind, NodeKind::Function) {
                    let child_config = PrintConfigBuilder::new()
                        .depth(config.depth() + 2)
                        .filter(config.filter().clone())
                        .path(
                            [config.path().clone(), vec![child.name.clone()]]
                                .concat(),
                        )
                        .is_linked(config.is_linked())
                        .use_full_path(config.use_full_path())
                        .build();

                    child.print(child_config);
                    printed_methods.insert(child.name.clone());
                }
            }
        }
    }

    pub fn should_print(&self, config: &PrintConfig) -> bool {
        let full_path = config.path().join("::");

        let filter_path = if config.use_full_path() {
            full_path
        } else {
            // skip first element (filename)
            full_path
                .split("::")
                .skip(1)
                .collect::<Vec<&str>>()
                .join("::")
        };

        config.filter().as_deref().map_or(true, |f| {
            filter_path.starts_with(f) || self.name().starts_with(f)
        })
    }

    fn print_linked_node(&self, linked_node: &TreeNode, config: &PrintConfig) {
        if config.is_linked() {
            // If we're already printing a linked node,
            // don't print further linked nodes to prevent a recursive loop
            return;
        }

        let linked_config = PrintConfigBuilder::new()
            .depth(config.depth())
            .filter(None)
            .path(vec![linked_node.name.clone()])
            .is_linked(true)
            .use_full_path(config.use_full_path())
            .build();

        linked_node.print(linked_config);
    }

    fn print_function(&self, config: &PrintConfig) {
        if let Some(function_data) = &self.function {
            let inputs: Vec<String> = function_data
                .inputs
                .iter()
                .map(|(name, type_)| format!("{}: {}", name, type_))
                .collect();
            let output = function_data
                .output
                .as_ref()
                .map_or_else(String::new, |output_type| {
                    format!(" -> {}", output_type)
                });
            custom_println(
                config.depth(),
                &format!("{}({}){}", self.name, inputs.join(", "), output),
            );
        } else {
            // no function data found -- just print function
            custom_println(config.depth(), &format!("{}()", self.name));
        }
    }

    fn print_enum(&self, config: &PrintConfig) {
        custom_println(config.depth(), &format!("{} (Enum)", self.name));
    }

    fn print_trait(&self, config: &PrintConfig) {
        custom_println(config.depth(), &format!("{} (Trait)", self.name));
    }

    fn print_variant(&self, config: &PrintConfig) {
        custom_println(config.depth(), &format!("{} (Variant)", self.name));
    }
}

fn custom_println(depth: usize, message: &str) {
    match depth {
        0..=1 => println!("{}", message),
        _ => {
            let padding = (0..depth - 2).map(|_| "│   ").collect::<String>();
            println!("{}└── {}", padding, message);
        }
    }
}
