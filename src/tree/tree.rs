use crate::print_config::{PrintConfig, PrintConfigBuilder};
use crate::types::{Identifiable, RustType};
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

    pub fn children(&self) -> &Option<Vec<TreeNode>> {
        &self.children
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
    ) -> bool {
        if !self.should_print(&config) {
            // despite not printing this node, it may still have children
            return self.print_children(writer, &config);
        }
        if let Some(linked_node) = &self.link {
            return self.print_linked_node(writer, linked_node, &config);
        }

        self.rtype.print(writer);
        true // any of the print_ functions will print something
    }

    fn print_children(
        &self,
        writer: &mut Box<dyn ClippyWriter>,
        config: &PrintConfig,
    ) -> bool {
        let mut has_printed = false;

        if let Some(children) = &self.children {
            for child in children {
                let mut child_config = config.clone();
                child_config.add_to_path(child.name.clone());
                let child_printed = child.print(writer, child_config);
                has_printed = has_printed || child_printed;
            }
        }
        has_printed
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
            .filter(None)
            .path(vec![linked_node.name.clone()])
            .is_linked(true)
            .use_full_path(config.use_full_path())
            .build();

        linked_node.print(writer, linked_config)
    }
}
