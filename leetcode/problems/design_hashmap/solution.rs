struct MyHashMap {
    data: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {

    fn new() -> Self {
        Self {
            data: vec![-1; 100003]
        }
    }

    fn hash(&self, key: i32) -> usize {
        (key as usize) % self.data.len()
    }
    
    fn put(&mut self, key: i32, value: i32) {
        let key = self.hash(key);
        self.data[key] = value;
    }
    
    fn get(&self, key: i32) -> i32 {
        let key = self.hash(key);
        self.data[key]
    }
    
    fn remove(&mut self, key: i32) {
        let key = self.hash(key);
        self.data[key] = -1;
    }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */