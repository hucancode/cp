use std::cmp::Ordering;
use std::cmp::max;
struct MyCalendarThree {
    starts: Vec<i32>,
    ends: Vec<i32>,
    max_overlap: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {

    fn new() -> Self {
        Self {
            starts: Vec::new(),
            ends: Vec::new(),
            max_overlap: 0,
        }
    }

    fn insert(&mut self, start_time: i32, end_time: i32) {
        match self.starts.binary_search(&start_time) {
            Ok(i) => self.starts.insert(i, start_time),
            Err(i) => self.starts.insert(i, start_time),
        };
        match self.ends.binary_search(&end_time) {
            Ok(i) => self.ends.insert(i, end_time),
            Err(i) => self.ends.insert(i, end_time),
        };
        let mut i = self.starts.partition_point(|&x| x < start_time);
        while i < self.starts.len() && self.starts[i] < end_time {
            let started = self.starts.partition_point(|&x| x < self.starts[i]+1);
            let ended = self.ends.partition_point(|&x| x <= self.starts[i]);
            self.max_overlap = max(self.max_overlap, (started - ended) as i32);
            i = self.starts.partition_point(|&x| x <= self.starts[i]);
        }
    }
    
    fn book(&mut self, start_time: i32, end_time: i32) -> i32 {
        self.insert(start_time, end_time);
        return self.max_overlap;
    }
}

/**
 * Your MyCalendarThree object will be instantiated and called as such:
 * let obj = MyCalendarThree::new();
 * let ret_1: i32 = obj.book(startTime, endTime);
 */