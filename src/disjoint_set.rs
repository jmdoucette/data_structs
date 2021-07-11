use std::collections::HashMap;

struct DisjointSet {
    parents: HashMap<i32, Option<i32>>,
    sizes: HashMap<i32, usize>,
}

impl DisjointSet {
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

    pub fn add(&mut self, x: i32) {
        todo!();
    }

    fn find(&mut self, x: i32) -> i32 {
        todo!();
    }

    pub fn same_component(&mut self, x: i32, y: i32) -> bool {
        self.find(x) == self.find(y)
    }
}