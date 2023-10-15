use std::collections::HashSet;

use syn::visit::Visit;
use syn::ExprPath;

use crate::types::{RustFunction, Visibility};

pub struct FunctionCallVisitor {
    pub functions: Vec<RustFunction>,
    pub instantiated_items: HashSet<String>,
}

impl<'ast> Visit<'ast> for FunctionCallVisitor {
    fn visit_expr_call(&mut self, expr_call: &'ast syn::ExprCall) {
        if let syn::Expr::Path(expr_path) = &*expr_call.func {
            if let Some(last_segment) = expr_path.path.segments.last() {
                let function_name = last_segment.ident.to_string();
                self.instantiated_items.insert(function_name.clone());
                let called_function =
                    RustFunction::new(Visibility::Restricted, &function_name);
                self.functions.push(called_function);
            }
        }
        syn::visit::visit_expr_call(self, expr_call);
    }

    fn visit_expr_struct(&mut self, expr_struct: &'ast syn::ExprStruct) {
        let struct_name = expr_struct.path.segments[0].ident.to_string();
        self.instantiated_items.insert(struct_name.clone());
        syn::visit::visit_expr_struct(self, expr_struct);
    }
}

impl Default for FunctionCallVisitor {
    fn default() -> Self {
        FunctionCallVisitor {
            functions: Vec::new(),
            instantiated_items: HashSet::new(),
        }
    }
}

pub struct ItemNameVisitor {
    pub item_names: HashSet<String>,
}

impl ItemNameVisitor {
    pub fn new() -> Self {
        ItemNameVisitor {
            item_names: HashSet::new(),
        }
    }
}

impl<'ast> Visit<'ast> for ItemNameVisitor {
    fn visit_expr_path(&mut self, node: &'ast ExprPath) {
        if let Some(ident) = node.path.get_ident() {
            self.item_names.insert(ident.to_string());
        }
        syn::visit::visit_expr_path(self, node);
    }
}
