mod format;
mod rust_function;
mod rust_struct;
mod visibility;

pub use rust_function::RustFunction;
pub use rust_struct::RustStruct;
pub use visibility::Visibility;

#[derive(Debug, Clone)]
pub enum RustType {
    Function(RustFunction),
    Struct(RustStruct),
    Enum,
    Trait,
    Variant,
    Link,
}

#[derive(Debug, Clone)]
pub struct RustEnum {
    pub id: String,
    pub visibility: Visibility,
    pub name: String,
    pub variants: Vec<(String, Vec<String>)>,
    pub methods: Vec<RustFunction>,
}

#[derive(Debug, Clone)]
pub struct RustTrait {
    pub id: String,
    pub visibility: Visibility,
    pub name: String,
    pub methods: Vec<RustFunction>,
    pub implementors: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct RustImpl {
    pub id: String,
    pub for_type: String,
    pub functions: Vec<RustFunction>,
}
