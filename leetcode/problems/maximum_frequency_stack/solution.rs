use std::cmp::max;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

struct FreqStack {
    freq: HashMap<i32, i32>,
    occurence: HashMap<i32, Vec<i32>>,
    top: i32,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {

    fn new() -> Self {
        Self {
            freq: HashMap::new(),
            occurence: HashMap::new(),
            top: 0,
        }
    }
    
    fn push(&mut self, val: i32) {
        self.freq
            .entry(val)
            .and_modify(|x| *x += 1)
            .or_insert(1);
        let f = self.freq[&val];
        self.top = max(self.top, f);
        self.occurence
            .entry(f)
            .and_modify(|arr| arr.push(val))
            .or_insert(vec![val]);
    }
    
    fn pop(&mut self) -> i32 {
        let mut ret = -1;
        if let Some(arr) = self.occurence
            .get_mut(&self.top) {
            if let Some(x) = arr.pop() {
                ret = x;
            }
            if arr.is_empty() {
                self.top -= 1;
            }
        }
        self.freq
            .entry(ret)
            .and_modify(|f| *f -= 1);
        return ret;
    }
}

/**
 * Your FreqStack object will be instantiated and called as such:
 * let obj = FreqStack::new();
 * obj.push(val);
 * let ret_2: i32 = obj.pop();
 */