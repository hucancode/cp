impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        use std::collections::BinaryHeap;
        let mut gifts: BinaryHeap<_> = gifts.into_iter().map(|x| x as i64).collect();
        for _ in 0..k {
            let x = gifts.pop().unwrap();
            gifts.push((x as f64).sqrt() as i64);
        }
        gifts.into_iter().sum()
    }
}
