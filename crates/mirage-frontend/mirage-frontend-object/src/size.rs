/// A size of an object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Size {
    size: usize,
}

impl Size {
    pub fn new(size: usize) -> Self {
        Self { size }
    }

    pub fn of<T: Sized>() -> Self {
        Self {
            size: std::mem::size_of::<T>(),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl std::iter::Sum for Size {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self {
            size: iter.map(|s| s.size).sum(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SizeEnum {
    Size8,
    Size16,
    Size32,
    Size64,
}
