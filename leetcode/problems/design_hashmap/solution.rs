struct MyHashMap {
    data: Vec<Vec<(i32,i32)>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {

    fn new() -> Self {
        Self {
            data: vec![Vec::new();1000], 
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        let hash_key = key as usize%self.data.len();
        match self.data[hash_key].binary_search_by_key(&key, |&(k,v)| k) {
            Ok(i) => self.data[hash_key][i] = (key, value),
            Err(i) => self.data[hash_key].insert(i, (key, value))
        }
    }
    
    fn get(&self, key: i32) -> i32 {
        let hash_key = key as usize%self.data.len();
        match self.data[hash_key].binary_search_by_key(&key, |&(k,v)| k) {
            Ok(i) => {
                let (k,v) = self.data[hash_key][i];
                v
            },
            Err(i) => -1
        }
    }
    
    fn remove(&mut self, key: i32) {
        let hash_key = key as usize%self.data.len();
        match self.data[hash_key].binary_search_by_key(&key, |&(k,v)| k) {
            Ok(i) => {
                self.data[hash_key].remove(i);
            },
            _ => (),
        }
    }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */