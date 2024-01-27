use crate::Stringify;
use std::fmt::Debug;


/// A list of items.
/// Syntax: { items... }
#[derive(Debug, Clone)]
pub struct List<T: Stringify + Debug + Clone> {
    inner: Vec<T>
}

impl<T: Stringify + Debug + Clone> List<T> {
    pub fn new() -> Self {
        Self {
            inner: Vec::new()
        }
    }

    pub fn push(&mut self, item: T) {
        self.inner.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T: Stringify + Debug + Clone> Stringify for List<T> {
    fn to_string(&self) -> String {
        let mut items = String::from("{");
        for (i, item) in self.inner.iter().enumerate() {
            items.push_str(item.to_string().as_str());
            if i != self.inner.len() - 1 {
                items.push_str(",");
            }
        }

        items.push_str("}");
        items
    }
}

impl<T> PartialEq for List<T>
    where T: Stringify + Debug + Clone + PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}
