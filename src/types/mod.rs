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
    Enum(RustEnum),
    Trait(RustTrait),
}

pub trait Identifiable {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
}


#[derive(Debug, Clone)]
pub struct RustEnum {
    pub id: String,
    pub visibility: Visibility,
    pub name: String,
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

impl Identifiable for RustType {
    fn id(&self) -> &str {
        match self {
            RustType::Function(func) => func.id(),
            RustType::Struct(strct) => strct.id(),
            RustType::Enum(enu) => enu.id(),
            RustType::Trait(trt) => trt.id(),
        }
    }

    fn name(&self) -> &str {
        match self {
            RustType::Function(func) => func.name(),
            RustType::Struct(strct) => strct.name(),
            RustType::Enum(enu) => enu.name(),
            RustType::Trait(trt) => trt.name(),
        }
    }
}


#[derive(Debug, Clone)]
pub struct RustTrait {
    pub id: String,
    pub visibility: Visibility,
    pub name: String,
    pub methods: Vec<RustFunction>,
    pub implementors: Vec<String>,
}

impl Identifiable for RustTrait {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }
}

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
