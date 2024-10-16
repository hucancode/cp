impl Solution {
    pub fn longest_diverse_string(mut a: i32, mut b: i32, mut c: i32) -> String {
        use std::collections::BinaryHeap;
        let mut sum = a+b+c;
        let mut pool = BinaryHeap::from([(a, 'a'), (b, 'b'), (c, 'c')]);
        let mut ret = String::new();
        let mut last = 'x';
        let mut last_count = -1;
        while let Some((count, key)) = pool.pop() {
            if count == 0 {
                break;
            }
            let remaining = sum - count;
            pool.push((last_count, last));
            let too_much = count > 1 && count >= remaining*2+1;
            let take = if too_much { 2 } else { 1 };
            sum -= take;
            for i in 0..take {
                ret.push(key);
            }
            last = key;
            last_count = count - take;
        }
        return ret;
    }
}
