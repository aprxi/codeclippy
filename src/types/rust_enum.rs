use super::{Identifiable, RustFunction, Visibility};
use crate::helpers::generate_id;

#[derive(Debug, Clone)]
pub struct RustEnum {
    id: String,
    pub visibility: Visibility,
    name: String,
    pub variants: Vec<(String, Vec<String>)>,
    pub methods: Vec<RustFunction>,
}

impl Identifiable for RustEnum {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl RustEnum {
    pub fn new_with_data(
        name: String,
        visibility: Visibility,
        variants: Vec<(String, Vec<String>)>,
        methods: Vec<RustFunction>,
    ) -> Self {
        RustEnum {
            id: generate_id(&name),
            name,
            visibility,
            variants,
            methods,
        }
    }
}
