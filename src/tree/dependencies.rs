use std::collections::HashMap;

use log;

use crate::print_config::PrintConfig;
use crate::registry::{GlobalRegistry, RegistryKind};
use crate::tree::{RootNode, TreeNode};
use crate::types::{RustStruct, RustType};
use crate::writers::ClippyWriter;

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

    pub fn print(&self, writer: &mut Box<dyn ClippyWriter>) {
        for (_, dependency) in &self.items_by_id {
            self.print_dependency(writer, dependency);
        }
    }

    fn print_dependency(
        &self,
        writer: &mut Box<dyn ClippyWriter>,
        dependency: &Dependency,
    ) {
        let _ = writeln!(writer, "@{}:", dependency.source().unwrap_or(""),);

        match dependency.node().rtype() {
            RustType::Function(rust_function) => {
                let _ = write!(writer, "{}", rust_function);
            }
            RustType::Struct(rust_struct) => {
                let _ = write!(writer, "{}", rust_struct);
            }
            RustType::Enum(rust_enum) => {
                let _ = write!(writer, "{}", rust_enum);
            }
            RustType::Trait(rust_trait) => {
                let _ = write!(writer, "{}", rust_trait);
            }
        }
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

pub fn find_dependencies(
    root: &mut RootNode,
    registry: &GlobalRegistry,
    config: &PrintConfig,
) {
    let mut dependencies = Dependencies::default();
    let local_items_map: HashMap<String, TreeNode> = root
        .children()
        .iter()
        .map(|item| (item.name().to_string(), item.clone()))
        .collect();

    for child in root.children_mut().iter_mut() {
        let mut inner_config = config.clone();
        inner_config.add_to_path(child.name().to_string());

        find_dependencies_recursive(
            child,
            registry,
            &mut dependencies,
            &local_items_map,
            &inner_config,
        );
    }
    root.set_dependencies(dependencies);
}

fn find_dependencies_recursive(
    tree: &mut TreeNode,
    global_registry: &GlobalRegistry,
    dependencies: &mut Dependencies,
    local_items_map: &HashMap<String, TreeNode>,
    config: &PrintConfig,
) {
    log::debug!("Processing node: {}", tree.name());
    process_function_node(tree);

    // get first item from vector
    // if node is printed, collects its dependencies
    if tree.should_print(config) {
        collect_dependencies(
            tree,
            global_registry,
            local_items_map,
            dependencies,
            config,
        );
    }

    // Process child nodes directly here
    for child in tree.children_mut() {
        let mut inner_config = config.clone();
        inner_config.add_to_path(child.name().to_string());

        if child.should_print(&inner_config) {
            // Recursively call this function on each child
            find_dependencies_recursive(
                child,
                global_registry,
                dependencies,
                local_items_map,
                &inner_config,
            );
        } else {
            log::debug!("Skipping node: {}", child.name());
        }
    }
}

fn process_function_node(tree: &mut TreeNode) {
    if let RustType::Function(rust_function) = tree.rtype_mut() {
        rust_function.extract_function_body();
    }
}

fn collect_dependencies(
    tree: &TreeNode,
    global_registry: &GlobalRegistry,
    local_items_map: &HashMap<String, TreeNode>,
    dependencies: &mut Dependencies,
    config: &PrintConfig,
) {
    log::debug!("Collecting dependencies for node: {}", tree.name());
    match &tree.rtype() {
        RustType::Function(rust_function) => {
            // List of all instantiated structs from the function
            let instantiated_struct_names: Vec<_> =
                rust_function.instantiated_items().iter().cloned().collect();

            for name in &instantiated_struct_names {
                // if name is in config.path, skip as cant depend on self
                if config.path().contains(&name) {
                    continue;
                }
                // Check if the struct exists in local items
                if let Some(local_item) = local_items_map.get(name) {
                    let node = (*local_item).clone();
                    let source = config.path().first().map(|s| s.as_str());
                    dependencies.register_item(
                        node.id().to_string(),
                        node.clone(),
                        source,
                    );
                }
                // If not in the local items, try global registry
                else if let Some(registry_item) =
                    global_registry.get_item_by_name(name)
                {
                    let RegistryKind::Struct(rust_struct) =
                        &registry_item.item();
                    let node = create_struct_node_from_registry(rust_struct);
                    dependencies.register_item(
                        node.id().to_string(),
                        node.clone(),
                        registry_item.source(),
                    );
                }
            }
        }
        RustType::Struct(rust_struct) => {
            log::warn!(
                "Dependencies of struct: {} not yet handled",
                rust_struct.name()
            );
        }
        _ => {
            // currently just support functions and structs
            panic!("Unhandled RustType variant: {:?}", tree.rtype());
        }
    }
}

fn create_struct_node_from_registry(s: &RustStruct) -> TreeNode {
    let mut node = TreeNode::new(RustType::Struct(s.clone()));
    for method in s.methods() {
        let method_node = TreeNode::new(RustType::Function(method.clone()));
        node.add_child(method_node);
    }

    node
}
