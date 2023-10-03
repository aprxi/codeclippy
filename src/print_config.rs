use std::env;

#[derive(Debug, Clone, PartialEq)]
pub struct PrintConfig {
    filter: Option<String>,
    path: Vec<String>,
    debug: bool,
    is_linked: bool,
    use_full_path: bool,
}

impl PrintConfig {
    pub fn filter(&self) -> &Option<String> {
        &self.filter
    }

    pub fn path(&self) -> &Vec<String> {
        &self.path
    }

    pub fn is_linked(&self) -> bool {
        self.is_linked
    }

    pub fn use_full_path(&self) -> bool {
        self.use_full_path
    }

    pub fn add_to_path(&mut self, name: String) {
        self.path.push(name);
    }
}

pub struct PrintConfigBuilder {
    filter: Option<String>,
    path: Vec<String>,
    debug: bool,
    is_linked: bool,
    use_full_path: bool,
}

impl PrintConfigBuilder {
    pub fn new() -> Self {
        let debug = match env::var("DEBUG") {
            Ok(value) => value == "true",
            Err(_) => false,
        };

        PrintConfigBuilder {
            filter: None,
            path: Vec::new(),
            debug,
            is_linked: false,
            use_full_path: false,
        }
    }

    pub fn filter(mut self, filter: Option<String>) -> Self {
        self.filter = filter;
        self
    }

    pub fn path(mut self, path: Vec<String>) -> Self {
        self.path = path;
        self
    }

    pub fn is_linked(mut self, is_linked: bool) -> Self {
        self.is_linked = is_linked;
        self
    }

    pub fn use_full_path(mut self, use_full_path: bool) -> Self {
        self.use_full_path = use_full_path;
        self
    }

    pub fn build(self) -> PrintConfig {
        PrintConfig {
            filter: self.filter,
            path: self.path,
            debug: self.debug,
            is_linked: self.is_linked,
            use_full_path: self.use_full_path,
        }
    }
}
