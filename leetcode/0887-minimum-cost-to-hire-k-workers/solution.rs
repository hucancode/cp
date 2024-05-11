impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        use std::collections::BinaryHeap;
        use std::cmp::Ordering;
        let n = quality.len();
        let mut workers: Vec<_> = quality.into_iter()
            .zip(wage.into_iter())
            .map(|(q,w)| (w as f64/q as f64, q))
            .collect();
        workers.sort_by(|(a,_), (b,_)| a.partial_cmp(&b)
            .unwrap_or(Ordering::Equal));
        let mut q = BinaryHeap::new();
        let mut total_quality = 0;
        let mut ret = f64::MAX;
        for (rate, quality) in workers {
            total_quality += quality;
            q.push(quality);
            if q.len() > k as usize {
                if let Some(quality) = q.pop() {
                    total_quality -= quality;
                }
            }
            if q.len() == k as usize {
                let score = rate * total_quality as f64;
                if score < ret {
                    ret = score;
                }
            }
        }
        ret
    }
}
