use std::collections::VecDeque;

struct MedianFinder {
    data: VecDeque<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
 // [1,3,5] + 2 = [1,2,3,5] = [2,3]
 // [1,3] + 2 = [1,2,3]
impl MedianFinder {

    fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }
    
    fn add_num(&mut self, num: i32) {
        let i = self.data.partition_point(|&x| x <= num);
        self.data.insert(i, num);
    }
    
    fn find_median(&self) -> f64 {
        let n = self.data.len();
        if n%2 == 1 {
            return self.data[n/2] as f64;
        } else {
            let a = self.data[n/2] as f64;
            let b = self.data[n/2-1] as f64;
            (a+b)*0.5
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
