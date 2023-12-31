use super::{RustEnum, RustFunction, RustStruct, RustTrait, Visibility};
use crate::writers::ClippyWriter;

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
    fn as_rust_type(&self) -> Option<&RustType> {
        None
    }
    fn methods(&self) -> Option<&Vec<RustFunction>> {
        None
    }
    fn print(&self, writer: &mut Box<dyn ClippyWriter>);
    fn is_public(&self) -> bool {
        *self.visibility() == Visibility::Public
    }
    fn visibility(&self) -> &Visibility;
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

    fn print(&self, writer: &mut Box<dyn ClippyWriter>) {
        match self {
            RustType::Function(func) => func.print(writer),
            RustType::Struct(strct) => strct.print(writer),
            RustType::Enum(enu) => enu.print(writer),
            RustType::Trait(trt) => trt.print(writer),
        }
    }

    fn visibility(&self) -> &Visibility {
        match self {
            RustType::Function(func) => func.visibility(),
            RustType::Struct(strct) => strct.visibility(),
            RustType::Enum(enu) => enu.visibility(),
            RustType::Trait(trt) => trt.visibility(),
        }
    }
    fn as_rust_type(&self) -> Option<&RustType> {
        Some(self)
    }

    fn methods(&self) -> Option<&Vec<RustFunction>> {
        match self {
            RustType::Function(func) => func.methods(),
            RustType::Struct(strct) => strct.methods(),
            RustType::Enum(enu) => enu.methods(),
            RustType::Trait(trt) => trt.methods(),
        }
    }
}
