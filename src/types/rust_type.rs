use super::{RustEnum, RustFunction, RustStruct, RustTrait};
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
    fn print(&self, writer: &mut Box<dyn ClippyWriter>);
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
}
