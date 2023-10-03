use crate::print_config::{PrintConfig, PrintConfigBuilder};
use crate::types::{Identifiable, RustFunction, RustStruct, RustType};
use crate::writers::ClippyWriter;

#[derive(Debug, Clone)]
pub struct TreeNode {
    id: String,
    name: String,
    rtype: RustType,
    children: Option<Vec<TreeNode>>,
    pub link: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(rtype: RustType) -> Self {
        TreeNode {
            id: rtype.id().into(),
            name: rtype.name().into(),
            rtype,
            children: None,
            link: None,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn rtype(&self) -> &RustType {
        &self.rtype
    }

    pub fn rtype_mut(&mut self) -> &mut RustType {
        &mut self.rtype
    }

    pub fn children_mut(&mut self) -> &mut Vec<TreeNode> {
        self.children.get_or_insert_with(Vec::new)
    }

    pub fn add_child(&mut self, child: TreeNode) {
        self.children_mut().push(child);
    }

    pub fn print(
        &self,
        writer: &mut Box<dyn ClippyWriter>,
        config: PrintConfig,
        as_tree: bool,
    ) -> bool {
        if !self.should_print(&config) {
            // despite not printing this node, it may still have children
            return self.print_children(writer, &config, as_tree);
        }
        if let Some(linked_node) = &self.link {
            return self.print_linked_node(writer, linked_node, &config);
        }

        if config.debug() {
            let full_path = config.path().join("::");
            custom_println(config.depth(), &format!("[{}]", full_path));
        }

        if as_tree {
            match &self.rtype {
                RustType::Function(rust_function) => {
                    self.print_function(&config, rust_function)
                }
                RustType::Struct(rust_struct) => {
                    self.print_struct(writer, &config, rust_struct);
                }
                RustType::Enum(_) => self.print_enum(&config),
                RustType::Trait(_) => self.print_trait(&config),
            }
        } else {
            self.rtype.print(writer);
        }
        true // any of the print_ functions will print something
    }

    fn print_children(
        &self,
        writer: &mut Box<dyn ClippyWriter>,
        config: &PrintConfig,
        as_tree: bool,
    ) -> bool {
        let mut has_printed = false;

        if let Some(children) = &self.children {
            for child in children {
                let child_depth =
                    match (config.depth(), &self.rtype, &child.rtype) {
                        (0, _, _) => 0,
                        (_, &RustType::Function(_), &RustType::Function(_)) => {
                            config.depth()
                        }
                        _ => config.depth() + 1,
                    };

                let mut child_config = config.clone();
                child_config.set_depth(child_depth);
                child_config.add_to_path(child.name.clone());
                let child_printed = child.print(writer, child_config, as_tree);
                has_printed = has_printed || child_printed;
            }
        }
        has_printed
    }

    fn print_struct(
        &self,
        writer: &mut Box<dyn ClippyWriter>,
        config: &PrintConfig,
        rust_struct: &RustStruct,
    ) {
        custom_println(
            config.depth(),
            &format!("struct {}", self.name),
        );

        if !rust_struct.fields().is_empty() {
            custom_println(config.depth() + 1, "Fields:");
            for (name, type_) in rust_struct.fields() {
                custom_println(
                    config.depth() + 2,
                    &format!("{}: {}", name, type_),
                );
            }
        }

        if let Some(children) = &self.children {
            if children
                .iter()
                .any(|child| matches!(child.rtype, RustType::Function(_)))
            {
                custom_println(config.depth() + 1, "Methods:");
                for child in children {
                    if matches!(child.rtype, RustType::Function(_)) {
                        let child_config = PrintConfigBuilder::new()
                            .depth(config.depth() + 2)
                            .filter(config.filter().clone())
                            .path(
                                [
                                    config.path().clone(),
                                    vec![child.name.clone()],
                                ]
                                .concat(),
                            )
                            .is_linked(config.is_linked())
                            .use_full_path(config.use_full_path())
                            .build();

                        child.print(writer, child_config, true);
                    }
                }
            }
        }
    }

    pub fn should_print(&self, config: &PrintConfig) -> bool {
        let current_path = config.path().join("::");

        let filter_path = if config.use_full_path() {
            current_path
        } else {
            // remove first element (filename) in case search is not
            // scoped to single file
            current_path
                .split("::")
                .skip(1)
                .collect::<Vec<&str>>()
                .join("::")
        };
        config.filter().as_deref().map_or(true, |f| {
            filter_path == f
                || (filter_path.starts_with(f)
                    && filter_path[f.len()..].starts_with("::"))
                || self.name() == f
        })
    }

    fn print_linked_node(
        &self,
        writer: &mut Box<dyn ClippyWriter>,
        linked_node: &TreeNode,
        config: &PrintConfig,
    ) -> bool {
        if config.is_linked() {
            // If we're already printing a linked node,
            // don't print further linked nodes to prevent a recursive loop
            return false;
        }

        let linked_config = PrintConfigBuilder::new()
            .depth(config.depth())
            .filter(None)
            .path(vec![linked_node.name.clone()])
            .is_linked(true)
            .use_full_path(config.use_full_path())
            .build();

        linked_node.print(writer, linked_config, true)
    }

    fn print_function(
        &self,
        config: &PrintConfig,
        rust_function: &RustFunction,
    ) {
        let inputs: Vec<String> = rust_function
            .inputs()
            .iter()
            .map(|(name, type_)| format!("{}: {}", name, type_))
            .collect();
        let output = rust_function
            .output()
            .as_ref()
            .map_or_else(String::new, |output_type| {
                format!(" -> {}", output_type)
            });
        custom_println(
            config.depth(),
            &format!(
                "fn {}({}){}",
                self.name,
                inputs.join(", "),
                output,
            ),
        );
    }

    fn print_enum(&self, config: &PrintConfig) {
        custom_println(config.depth(), &format!("enum {}", self.name));
    }

    fn print_trait(&self, config: &PrintConfig) {
        custom_println(config.depth(), &format!("trait {}", self.name));
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
