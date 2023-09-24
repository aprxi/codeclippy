use super::dependencies::Dependencies;
use super::TreeNode;

pub struct RootNode {
    filename: String,
    dependencies: Dependencies,
    children: Vec<TreeNode>,
}

impl RootNode {
    pub fn new(filename: String) -> Self {
        RootNode {
            filename,
            dependencies: Dependencies::default(),
            children: Vec::new(),
        }
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn dependencies(&self) -> &Dependencies {
        &self.dependencies
    }

    pub fn set_dependencies(&mut self, dependencies: Dependencies) {
        self.dependencies = dependencies;
    }

    pub fn children(&self) -> &Vec<TreeNode> {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut Vec<TreeNode> {
        &mut self.children
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
