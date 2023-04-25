use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct SmallestInfiniteSet {
    heap: BinaryHeap<Reverse<i32>>,
    head: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {

    fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
            head: 1
        }
    }

    fn pop_all(&mut self, x: i32) {
        while let Some(Reverse(y)) = self.heap.peek() {
            if x == *y {
                self.heap.pop();
                continue;
            }
            break;
        }
    }
    
    fn pop_smallest(&mut self) -> i32 {
        if let Some(Reverse(x)) = self.heap.pop() {
            self.pop_all(x);
            return x;
        }
        let ret = self.head;
        self.head += 1;
        return ret;
    }
    
    fn add_back(&mut self, num: i32) {
        if num >= self.head {
            return;
        }
        self.heap.push(Reverse(num));
    }
}

/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */