use std::collections::HashMap;

use crate::rust_types::RustStruct;

pub struct GlobalRegistry {
    items_by_id: HashMap<String, RegistryItem>,
    name_to_id_mapping: HashMap<String, String>,
    // TODO: add enums, traits, impls based on requirements
}

impl Default for GlobalRegistry {
    fn default() -> Self {
        GlobalRegistry {
            items_by_id: HashMap::new(),
            name_to_id_mapping: HashMap::new(),
        }
    }
}

impl GlobalRegistry {
    pub fn register_struct(
        &mut self,
        rust_struct: RustStruct,
        source: Option<&str>,
    ) {
        if self.name_to_id_mapping.contains_key(&rust_struct.name) {
            panic!(
                "Detected more than one public item with the name '{}'. It is \
                 not supported to have multiple public items with the same \
                 name in the global registry.",
                rust_struct.name
            );
        }
        let registry_item = RegistryItem {
            item: RegistryKind::Struct(rust_struct.clone()),
            source: source.map(|s| s.to_string()),
        };
        self.name_to_id_mapping
            .insert(rust_struct.name.clone(), rust_struct.id.clone());
        self.items_by_id
            .insert(rust_struct.id.clone(), registry_item);
    }

    pub fn get_item_by_name(&self, name: &str) -> Option<&RegistryItem> {
        if let Some(id) = self.name_to_id_mapping.get(name) {
            return self.get_item_by_id(id);
        }
        None
    }

    pub fn get_item_by_id(&self, id: &str) -> Option<&RegistryItem> {
        self.items_by_id.get(id)
    }
}

pub enum RegistryKind {
    Struct(RustStruct),
}

pub struct RegistryItem {
    item: RegistryKind,
    source: Option<String>,
}

impl RegistryItem {
    pub fn item(&self) -> &RegistryKind {
        &self.item
    }
    pub fn source(&self) -> Option<&str> {
        self.source.as_ref().map(|s| s.as_str())
    }
}
