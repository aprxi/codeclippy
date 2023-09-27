use std::collections::HashSet;
use std::fmt;

use syn::visit::Visit;

use crate::function_visitor::FunctionCallVisitor;

#[derive(Clone)]
pub struct RustFunction {
    pub id: String,
    pub visibility: Visibility,
    pub name: String,
    pub inputs: Vec<(String, String)>,
    pub output: Option<String>,
    pub block: Option<Box<syn::Block>>,
    pub functions: Vec<RustFunction>,
    pub instantiated_items: HashSet<String>,
}

impl std::fmt::Debug for RustFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "RustFunction {{")?;
        writeln!(f, "  id: {},", self.id)?;
        writeln!(f, "  visibility: {:?},", self.visibility)?;
        writeln!(f, "  name: {},", self.name)?;
        writeln!(f, "  inputs: {:?},", self.inputs)?;
        writeln!(f, "  output: {:?},", self.output)?;
        writeln!(f, "  block: {},", self.block.is_some())?;
        writeln!(f, "  functions: [")?;
        for func in &self.functions {
            writeln!(f, "    {},", func)?;
        }
        writeln!(f, "  ],")?;
        writeln!(f, "  instantiated_items: {:?},", self.instantiated_items)?;
        write!(f, "}}")
    }
}

impl fmt::Display for RustFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "RustFunction {{")?;
        writeln!(f, "  id: {},", self.id)?;
        writeln!(f, "  visibility: {:?},", self.visibility)?;
        writeln!(f, "  name: {},", self.name)?;
        writeln!(f, "  inputs: {:?},", self.inputs)?;
        writeln!(f, "  output: {:?},", self.output)?;
        writeln!(f, "  functions: [")?;
        for func in &self.functions {
            writeln!(f, "    {},", func)?;
        }
        writeln!(f, "  ],")?;
        writeln!(f, "  instantiated_items: {:?},", self.instantiated_items)?;
        write!(f, "}}")
    }
}

impl RustFunction {
    pub fn extract_function_body(&mut self) {
        if let Some(ref block) = self.block {
            let mut body_visitor = FunctionCallVisitor::default();
            body_visitor.visit_block(block);

            self.functions.extend(body_visitor.functions);
            self.instantiated_items
                .extend(body_visitor.instantiated_items);
        }
    }
}

#[derive(Debug, Clone)]
pub struct RustStruct {
    pub id: String,
    pub visibility: Visibility,
    pub name: String,
    pub fields: Vec<(String, String)>,
    pub methods: Vec<RustFunction>,
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

impl fmt::Display for RustStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "RustStruct {{")?;
        writeln!(f, "  id: {},", self.id)?;
        writeln!(f, "  visibility: {:?},", self.visibility)?;
        writeln!(f, "  name: {},", self.name)?;
        writeln!(f, "  fields: {:?},", self.fields)?;
        writeln!(f, "  methods: [")?;
        for method in &self.methods {
            writeln!(f, "    {},", method)?;
        }
        writeln!(f, "  ],")?;
        write!(f, "}}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Restricted,
    Inherited,
}
