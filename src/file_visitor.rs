use std::fs;

use syn::__private::ToTokens;
use syn::visit::Visit;
use syn::{File, ImplItem, Item, TraitItem};

use crate::helpers::generate_id;
use crate::rust_types::{
    RustEnum, RustFunction, RustImpl, RustStruct, RustTrait, Visibility,
};

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    Function,
    Struct,
    Enum,
    Trait,
    Variant,
	Link,
}

#[derive(Debug, Clone)]
pub struct RustFileVisitor {
    current_file: String,
    pub functions: Vec<RustFunction>,
    pub structs: Vec<RustStruct>,
    pub enums: Vec<RustEnum>,
    pub traits: Vec<RustTrait>,
    pub impls: Vec<RustImpl>,
}

impl RustFileVisitor {
    pub fn new(file_name: String) -> Self {
        RustFileVisitor {
            current_file: file_name,
            functions: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
            traits: Vec::new(),
            impls: Vec::new(),
        }
    }

    pub fn current_file(&self) -> String {
        self.current_file.clone()
    }
}

impl RustFileVisitor {
    pub fn read_files(
        file_paths: Vec<&str>,
    ) -> Result<Vec<RustFileVisitor>, Box<dyn std::error::Error>> {
        let mut visitors = Vec::new();

        for file_path in file_paths {
            let content = fs::read_to_string(file_path)?;
            let mut visitor = RustFileVisitor::new(file_path.to_string());
            let syntax_tree: File = syn::parse_file(&content)?;
            visitor.visit_file(&syntax_tree);
            // associate methods with their structs and enums
            visitor.associate_methods();
            visitors.push(visitor);
        }

        Ok(visitors)
    }

    #[allow(dead_code)]
    pub fn print_collected_data(&self) {
        Self::print_items("Functions", &self.functions);
        Self::print_items("Structs", &self.structs);
        Self::print_items("Enums", &self.enums);
        Self::print_items("Traits", &self.traits);
        Self::print_items("Impls", &self.impls);
    }

    fn associate_methods_with_structs(&mut self) {
        for rust_impl in &self.impls {
            if let Some(struct_to_update) = self
                .structs
                .iter_mut()
                .find(|struct_item| struct_item.name == rust_impl.for_type)
            {
                struct_to_update
                    .methods
                    .extend_from_slice(&rust_impl.functions);
            }
        }
    }

    fn associate_methods_with_enums(&mut self) {
        for rust_impl in &self.impls {
            if let Some(e) =
                self.enums.iter_mut().find(|e| e.name == rust_impl.for_type)
            {
                e.methods.extend_from_slice(&rust_impl.functions);
            }
        }
    }

    fn associate_methods(&mut self) {
        self.associate_methods_with_structs();
        self.associate_methods_with_enums();
    }

    #[allow(dead_code)]
    fn print_items<T: std::fmt::Debug>(label: &str, items: &[T]) {
        println!("\n{}:", label);
        for item in items {
            println!("{:?}", item);
        }
    }
}

impl<'ast> Visit<'ast> for RustFileVisitor {
    fn visit_item(&mut self, item: &'ast Item) {
        if let Item::Impl(impl_item) = item {
            self.visit_item_impl(impl_item);
        }

        match item {
            Item::Fn(func) => {
                let rust_function = extract_function(
                    &func.sig,
                    Some(&func.vis),
                    Some(func.block.clone()),
                );
                self.functions.push(rust_function);
            }
            Item::Struct(struct_item) => {
                let fields = struct_item
                    .fields
                    .iter()
                    .map(|field| {
                        (
                            field.ident.as_ref().unwrap().to_string(),
                            format!("{}", field.ty.to_token_stream()),
                        )
                    })
                    .collect::<Vec<_>>();
                let rust_struct = RustStruct {
                    id: generate_id(&struct_item.ident.to_string()),
                    visibility: visibility_to_local_version(&struct_item.vis),
                    name: struct_item.ident.to_string(),
                    fields,
                    methods: vec![],
                };
                self.structs.push(rust_struct);
            }
            Item::Enum(enum_item) => {
                let variants = enum_item
                    .variants
                    .iter()
                    .map(|variant| {
                        let associated_data = variant
                            .fields
                            .iter()
                            .map(|field| {
                                format!("{}", field.ty.to_token_stream())
                            })
                            .collect();
                        (variant.ident.to_string(), associated_data)
                    })
                    .collect::<Vec<_>>();
                let rust_enum = RustEnum {
                    id: generate_id(&enum_item.ident.to_string()),
                    visibility: visibility_to_local_version(&enum_item.vis),
                    name: enum_item.ident.to_string(),
                    variants,
                    methods: vec![],
                };
                self.enums.push(rust_enum);
            }
            Item::Trait(trait_item) => {
                let methods = trait_item
                    .items
                    .iter()
                    .filter_map(|item| {
                        if let TraitItem::Fn(method) = item {
                            Some(extract_function(&method.sig, None, None))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                let rust_trait = RustTrait {
                    id: generate_id(&trait_item.ident.to_string()),
                    visibility: visibility_to_local_version(&trait_item.vis),
                    name: trait_item.ident.to_string(),
                    methods,
                    implementors: vec![],
                };
                self.traits.push(rust_trait);
            }
            _ => {}
        }
    }

    fn visit_item_impl(&mut self, impl_item: &'ast syn::ItemImpl) {
        let for_type = format!("{}", impl_item.self_ty.to_token_stream());

        let mut functions = Vec::new();
        for item in &impl_item.items {
            if let ImplItem::Fn(func) = item {
                let rust_function = extract_function(
                    &func.sig,
                    Some(&func.vis),
                    Some(Box::new(func.block.clone())),
                );
                functions.push(rust_function.clone());
            }
        }

        let rust_impl = RustImpl {
            id: generate_id(&for_type),
            for_type,
            functions,
        };
        self.impls.push(rust_impl);
    }
}

fn extract_function(
    sig: &syn::Signature,
    vis: Option<&syn::Visibility>,
    block: Option<Box<syn::Block>>,
) -> RustFunction {
    let inputs_vec = sig
        .inputs
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Receiver(_) => ("self".into(), "".into()),
            syn::FnArg::Typed(pat_type) => {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    (
                        pat_ident.ident.to_string(),
                        pat_type.ty.to_token_stream().to_string(),
                    )
                } else {
                    ("".into(), pat_type.ty.to_token_stream().to_string())
                }
            }
        })
        .collect();

    let output_option = match &sig.output {
        syn::ReturnType::Default => None,
        syn::ReturnType::Type(_, ty) => Some(ty.to_token_stream().to_string()),
    };

    RustFunction {
        id: generate_id(&sig.ident.to_string()),
        visibility: vis
            .map_or(Visibility::Restricted, visibility_to_local_version),
        name: sig.ident.to_string(),
        inputs: inputs_vec,
        output: output_option,
        block,
        functions: vec![],
        instantiated_structs: vec![],
    }
}

fn visibility_to_local_version(vis: &syn::Visibility) -> Visibility {
    match vis {
        syn::Visibility::Public(_) => Visibility::Public,
        syn::Visibility::Restricted(_) => Visibility::Restricted,
        syn::Visibility::Inherited => Visibility::Inherited,
    }
}
