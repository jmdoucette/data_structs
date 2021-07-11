use std::collections::HashMap;

pub struct DisjointSet<T: Eq> {
    parents: HashMap<T, Option<T>>,
    sizes: HashMap<T, usize>,
}

impl<T: Eq> DisjointSet<T> {
    pub fn new() -> Self {
        Self {
            parents: HashMap::new(),
            sizes: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.parents.is_empty()
    }

    pub fn len(&self) -> usize {
        self.parents.len()
    }

    pub fn add(&mut self, x: T) {
        todo!();
    }

    pub fn union(&mut self, x: T, y: T) {

    }

    fn find(&mut self, x: T) -> T {
        todo!();
    }

    pub fn same_component(&mut self, x: T, y: T) -> bool {
        self.find(x) == self.find(y)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(DisjointSet::<i32>::new().is_empty(), true);
    }
}