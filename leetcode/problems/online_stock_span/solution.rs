struct StockSpanner {
    peaks: Vec<(i32, i32)>,
    day: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {

    fn new() -> Self {
        Self {
            peaks: Vec::new(),
            day: 0,
        }
    }
    
    fn next(&mut self, price: i32) -> i32 {
        self.day += 1;
        let mut last_peak_day = 0;
        while let Some(&(day, x)) = self.peaks.last() {
            if x > price {
                last_peak_day = day;
                break;
            }
            self.peaks.pop();
        }
        self.peaks.push((self.day, price));
        return self.day - last_peak_day;
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */