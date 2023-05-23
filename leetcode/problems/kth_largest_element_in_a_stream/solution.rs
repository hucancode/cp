const INF: i32 = 10000;
struct KthLargest {
    data: Vec<i32>,
    cap: usize,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, mut nums: Vec<i32>) -> Self {
        nums.sort_by(|a, b| b.cmp(a));
        nums.resize(k as usize, -INF);
        Self {
            data: nums,
            cap: k as usize,
        }
    }
    
    fn add(&mut self, val: i32) -> i32 {
        let i = self.data
            .binary_search_by(|x| val.cmp(&x))
            .unwrap_or_else(|e| e);
        self.data.insert(i, val);
        self.data.resize(self.cap, -INF);
        *self.data.last().unwrap_or(&0)
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */