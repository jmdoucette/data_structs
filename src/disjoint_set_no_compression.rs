use std::collections::HashMap;
use std::hash::Hash;

pub struct DisjointSetNoCompression<T: Hash + Eq + Copy> {
    parents: HashMap<T, Option<T>>,
    sizes: HashMap<T, usize>,
}

impl<T: Hash + Eq + Copy> DisjointSetNoCompression<T> {
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
        if self.parents.contains_key(&x) {
            panic!("disjoint set already contains this key");
        }

        self.parents.insert(x, None);
        self.sizes.insert(x, 1);
    }

    pub fn union(&mut self, x: T, y: T) {
        let sx = self.find(x);
        let sy = self.find(y);

        // x and y are already in the same set, no work needed
        if sx == sy {
            return;
        }
        // x is in the larger set, x becomes ys parent
        else if self.sizes.get(&sx) >= self.sizes.get(&sy) {
            self.parents.insert(sy, Some(sx));
            let sy_size = self.sizes.remove(&sy).unwrap();
            if let Some(sx_size) = self.sizes.get_mut(&sx) {
                *sx_size += sy_size;
            }
        }
        // y is in the larger set, y becomes xs parent
        else {
            self.parents.insert(sx, Some(sy));
            let sx_size = self.sizes.remove(&sx).unwrap();
            if let Some(sy_size) = self.sizes.get_mut(&sy) {
                *sy_size += sx_size;
            }
        }
    }

    fn find(&mut self, x: T) -> T {
        let mut curr = x;
        loop {
            match self.parents.get(&curr).unwrap() {
                Some(next) => curr = *next,
                None => return curr,
            }
        }
    }

    pub fn same_set(&mut self, x: T, y: T) -> bool {
        self.find(x) == self.find(y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_len() {
        let mut ds = DisjointSetNoCompression::new();
        assert!(ds.is_empty());
        assert_eq!(ds.len(), 0);

        ds.add(1);
        ds.add(2);
        assert!(!ds.is_empty());
        assert_eq!(ds.len(), 2);
    }

    #[test]
    #[should_panic]
    fn test_add() {
        let mut ds = DisjointSetNoCompression::new();
        ds.add(1);
        ds.add(1);
    }

    #[test]
    fn test_union_and_same_set() {
        let mut ds = DisjointSetNoCompression::new();
        for i in 0..8 {
            ds.add(i);
        }
        assert!(!ds.same_set(0, 2));
        assert!(!ds.same_set(0, 2));
        assert!(!ds.same_set(4, 0));

        ds.union(2, 4);
        assert!(ds.same_set(2, 4));
        assert!(ds.same_set(4, 2));

        ds.union(4, 2);
        assert!(ds.same_set(2, 4));
        assert!(ds.same_set(4, 2));

        ds.union(2, 6);
        assert!(ds.same_set(2, 6));
        assert!(ds.same_set(6, 4));

        ds.union(0, 7);
        ds.union(5, 0);
        assert!(!ds.same_set(5, 2));
        assert!(ds.same_set(6, 4));

        ds.union(5, 6);
        ds.union(1, 3);
        assert!(ds.same_set(7, 2));
        assert!(ds.same_set(1, 3));
        assert!(!ds.same_set(1, 7));
        assert!(!ds.same_set(3, 0));
    }
}
