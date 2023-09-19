use syn::visit::Visit;
use uuid::Uuid;

use crate::rust_types::{RustFunction, Visibility};

pub struct FunctionCallVisitor {
    pub functions: Vec<RustFunction>,
    pub instantiated_structs: Vec<String>,
}

impl<'ast> Visit<'ast> for FunctionCallVisitor {
    fn visit_expr_call(&mut self, expr_call: &'ast syn::ExprCall) {
        if let syn::Expr::Path(expr_path) = &*expr_call.func {
            if let Some(last_segment) = expr_path.path.segments.last() {
                let function_name = last_segment.ident.to_string();
                let called_function = RustFunction {
                    id: Uuid::new_v4().to_string(),
                    visibility: Visibility::Restricted,
                    name: function_name,
                    inputs: vec![],
                    output: None,
                    block: None,
                    functions: vec![],
                    instantiated_structs: vec![],
                };
                self.functions.push(called_function);
            }
        }
        syn::visit::visit_expr_call(self, expr_call);
    }

    fn visit_expr_struct(&mut self, expr_struct: &'ast syn::ExprStruct) {
        let struct_name = expr_struct.path.segments[0].ident.to_string();
        self.instantiated_structs.push(struct_name.clone());

        syn::visit::visit_expr_struct(self, expr_struct);
    }
}

impl Default for FunctionCallVisitor {
    fn default() -> Self {
        FunctionCallVisitor {
            functions: Vec::new(),
            instantiated_structs: Vec::new(),
        }
    }
}
