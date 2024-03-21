use std::cmp::{min, max};
impl Solution {
    pub fn get_probability(balls: Vec<i32>) -> f64 {
        let mut fact = |n| {
            (1..=n).map(|i| i as f64).product::<f64>()
        };
        let mut raising_fact = |n: i32, k: i32| {
            (n..n+k).map(|i| i as f64).product::<f64>()
        };
        let mut falling_fact = |n: i32, k: i32| {
            (n-k+1..=n).map(|i| i as f64).product::<f64>()
        };
        let mut ncr = |n: i32, mut r: i32| {
            r = max(r, n-r);
            falling_fact(n,n-r)/fact(n-r)
        };
        let mut all_posibility = 0.0;
        let mut matched = 0.0;
        let all_balls: i32 = balls.iter().sum();
        let m = all_balls/2;
        let n = balls.len();
        let mut q = Vec::new();
        q.push((0,0,0,0,0,1.0));
        while let Some((c1,c2,k1,k2,i,res)) = q.pop() {
            if i >= n {
                if k1 == k2 {
                    matched += res;
                }
                all_posibility += res;
                continue;
            }
            let b = balls[i];
            for x in 0..=b {
                let y = b-x;
                if c1 + x > m || c2 + y > m {
                    continue;
                }
                q.push((c1+x, 
                    c2+y, 
                    k1 + min(x,1), 
                    k2 + min(y,1), 
                    i+1, 
                    res * ncr(c1+x,x) * ncr(c2+y,y)));
            }
        }
        matched / all_posibility
    }
}
