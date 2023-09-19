use std::collections::HashMap;

use crate::rust_types::RustStruct;

pub struct GlobalRegistry {
    structs_by_id: HashMap<String, RustStruct>,
    name_to_id_mapping: HashMap<String, String>,
    // TODO: add enums, traits, impls based on requirements
}

impl Default for GlobalRegistry {
    fn default() -> Self {
        GlobalRegistry {
            structs_by_id: HashMap::new(),
            name_to_id_mapping: HashMap::new(),
        }
    }
}

impl GlobalRegistry {
    pub fn register_struct(&mut self, rust_struct: RustStruct) {
        if self.name_to_id_mapping.contains_key(&rust_struct.name) {
            panic!(
                "Detected more than one public struct with the name '{}'. It \
                 is not supported to have multiple public structs with the \
                 same name in the global registry.",
                rust_struct.name
            );
        }
        self.name_to_id_mapping
            .insert(rust_struct.name.clone(), rust_struct.id.clone());
        self.structs_by_id
            .insert(rust_struct.id.clone(), rust_struct);
    }

    pub fn get_struct_by_name(&self, name: &str) -> Option<&RustStruct> {
        if let Some(id) = self.name_to_id_mapping.get(name) {
            return self.structs_by_id.get(id);
        }
        None
    }

    pub fn get_struct_by_id(&self, id: &str) -> Option<&RustStruct> {
        self.structs_by_id.get(id)
    }
}
