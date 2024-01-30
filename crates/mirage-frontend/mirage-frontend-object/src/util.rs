use crate::stringify::Stringify;
use std::{fmt::Debug, slice::Iter};


/// A list of items.
/// Syntax: { items... }
#[derive(Clone, Debug)]
pub struct List<T>
where
    T: Stringify + Debug + Clone,
{
    inner: Vec<T>,
}




impl<T> List<T>
where
    T: Stringify + Debug + Clone
{
    pub fn from_vec(vec: Vec<T>) -> Self {
        Self {
            inner: vec
        }
    }

    pub fn new() -> Self {
        let vec = Vec::new();
        Self {
            inner: vec
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

    pub fn iter(&self) -> Iter<T> {
        self.inner.iter()
    }
}

impl<T: Stringify + Debug + Clone> Iterator for List<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .iter()
            .next()
            .cloned()
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
