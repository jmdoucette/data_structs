pub struct BloomFilter<T: Hash + Eq> {
    arr: Vec<u8>,
    k: usize,
}

impl<T: Hash + Eq> for BloomFilter<T> {
    fn new(m: usize, k usize) -> Self {
        Self {
            arr: Vec::with_capacity(m),
            k,
        }
    }

    fn add(x: T) {
        todo!();
    }

    fn might_contain(x: T) -> bool {
        todo!();
    }

    fn 

}