use std::collections::HashMap;
use std::cmp::Ordering;

struct FrequencyTracker {
    items: HashMap<i32, i32>,
    freq: HashMap<i32, i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrequencyTracker {

    fn new() -> Self {
        Self {
            items: HashMap::new(),
            freq: HashMap::new(),
        }
    }
    
    fn add(&mut self, number: i32) {
        let f = self.items
            .entry(number)
            .or_default();
        self.freq
            .entry(*f)
            .and_modify(|c| *c -= 1);
        *f += 1;
        self.freq
            .entry(*f)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }
    
    fn delete_one(&mut self, number: i32) {
        let f = self.items
            .entry(number)
            .or_default();
        if *f == 0 {
            return;
        }
        self.freq
            .entry(*f)
            .and_modify(|c| *c -= 1);
        *f -= 1;
        self.freq
            .entry(*f)
            .and_modify(|c| *c += 1);
    }
    
    fn has_frequency(&self, f: i32) -> bool {
        if let Some(&c) = self.freq.get(&f) {
            c != 0
        } else {
            false
        }
    }
}

/**
 * Your FrequencyTracker object will be instantiated and called as such:
 * let obj = FrequencyTracker::new();
 * obj.add(number);
 * obj.delete_one(number);
 * let ret_3: bool = obj.has_frequency(frequency);
 */