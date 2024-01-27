use crate::Stringify;

/// An identifier
/// Example: x, y, z
#[derive(Debug, Clone, PartialEq)]
pub struct Ident {
    inner: String
}

impl Ident {
    pub fn new(inner: String) -> Self {
        Self {
            inner
        }
    }
}

impl Stringify for Ident {
    fn to_string(&self) -> String {
        self.inner.clone()
    }
}



impl Into<Ident> for String {
    fn into(self) -> Ident {
        Ident::new(self)
    }
}

impl Into<Ident> for &str {
    fn into(self) -> Ident {
        Ident::new(self.to_string())
    }
}

impl Into<String> for Ident {
    fn into(self) -> String {
        self.inner
    }
}
