use super::dependencies::Dependencies;
use super::{TreeNode, Dependents};
use crate::localfs::FilePath;
use crate::print_config::PrintConfigBuilder;
use crate::writers::ClippyWriter;

pub struct RootNode {
    file_path: FilePath,
    dependencies: Dependencies,
    dependents: Dependents,
    children: Vec<TreeNode>,
}

impl RootNode {
    pub fn new(file_path: FilePath) -> Self {
        RootNode {
            file_path,
            dependencies: Dependencies::default(),
            dependents: Dependents::default(),
            children: Vec::new(),
        }
    }

    pub fn file_path(&self) -> &FilePath {
        &self.file_path
    }

    pub fn dependencies(&self) -> &Dependencies {
        &self.dependencies
    }

    pub fn set_dependencies(&mut self, dependencies: Dependencies) {
        self.dependencies = dependencies;
    }

    pub fn dependents(&self) -> &Dependents {
        &self.dependents
    }

    pub fn set_dependents(&mut self, dependents: Dependents) {
        self.dependents = dependents;
    }

    pub fn children(&self) -> &Vec<TreeNode> {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut Vec<TreeNode> {
        &mut self.children
    }

    pub fn print(
        &self,
        writer: &mut Box<dyn ClippyWriter>,
        filter: Option<&str>,
        use_full_path: bool,
    ) {
        for child in self.children() {
            let config = PrintConfigBuilder::new()
                .filter(filter.map(|s| s.to_string()))
                .path(vec![
                    self.file_path.relative_path().to_string(),
                    child.name().to_string(),
                ])
                .is_linked(false)
                .use_full_path(use_full_path)
                .build();

            child.print(writer, config);
        }
    }

    pub fn find_child_by_name(&self, name: &str) -> Option<&TreeNode> {
        self.children.iter().find(|&child| child.name() == name)
    }

    pub fn has_child_named(&self, name: &str) -> bool {
        self.find_child_by_name(name).is_some()
    }

    pub fn add_child(&mut self, child: TreeNode) {
        self.children.push(child);
    }
}
