use std::collections::HashMap;

use crate::rust_types::RustStruct;

pub struct GlobalRegistry {
    structs: HashMap<String, RustStruct>,
    // TODO: add enums, traits, impls
}

impl Default for GlobalRegistry {
    fn default() -> Self {
        GlobalRegistry {
            structs: HashMap::new(),
        }
    }
}

impl GlobalRegistry {
    pub fn register_struct(&mut self, rust_struct: RustStruct) {
        if self.structs.contains_key(&rust_struct.name) {
            panic!(
                "Detected more than one public struct with the name '{}'. It \
                 is not supported to have multiple public structs with the \
                 same name in the global registry.",
                rust_struct.name
            );
        }
        self.structs.insert(rust_struct.name.clone(), rust_struct);
    }

    pub fn get_struct(&self, name: &str) -> Option<&RustStruct> {
        self.structs.get(name)
    }
}
