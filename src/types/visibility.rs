use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Restricted,
    Inherited,
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Visibility::Public => write!(f, "pub"),
            _ => write!(f, ""),
        }
    }
}
