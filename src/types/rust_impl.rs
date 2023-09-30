use super::{Identifiable, RustFunction};

#[derive(Debug, Clone)]
pub struct RustImpl {
    pub id: String,
    pub for_type: String,
    pub functions: Vec<RustFunction>,
}

impl Identifiable for RustImpl {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.for_type
    }
}
