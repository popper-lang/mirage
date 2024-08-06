use crate::stringify::Stringify;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Flags {
    pub inner: Vec<Flag>
}

impl Flags {
    pub fn new(inner: Vec<Flag>) -> Self {
        Self {
            inner
        }
    }

    pub fn push(&mut self, flag: Flag) {
        self.inner.push(flag);
    }
    pub fn contains(&self, flag: &Flag) -> bool {
        self.inner.contains(flag)
    }
}

impl Stringify for Flags {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for flag in &self.inner {
            s.push_str(&flag.to_string());
            s.push(' ');
        }
        s
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Flag {
    pub name: String
}

impl Flag {
    pub fn not_loadable() -> Self {
        Self {
            name: "not_loadable".to_string()
        }
    }
    pub fn new(name: String) -> Self {
        Self {
            name
        }
    }
}

impl Stringify for Flag {
    fn to_string(&self) -> String {
        format!("#{}", self.name)
    }
}
