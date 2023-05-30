struct MyHashSet {
    data: Vec<bool>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {

    fn new() -> Self {
        Self {
            data: vec![false; 100003],
        }
    }

    fn hash(&self, key: i32) -> usize {
        (key as usize) % self.data.len()
    }
    
    fn add(&mut self, key: i32) {
        let key = self.hash(key);
        self.data[key] = true;
    }
    
    fn remove(&mut self, key: i32) {
        let key = self.hash(key);
        self.data[key] = false;
    }
    
    fn contains(&self, key: i32) -> bool {
        let key = self.hash(key);
        self.data[key]
    }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */