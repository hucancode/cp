impl Solution {
    pub fn can_distribute(nums: Vec<i32>, mut quantity: Vec<i32>) -> bool {
        let mut pool = vec![0i32; 1001];
        for i in nums {
            pool[i as usize] += 1;
        }
        pool.sort_by(|a,b| b.cmp(a));
        quantity.sort_by(|a,b| b.cmp(a));
        while let Some(&x) = pool.last() {
            if x == 0 {
                pool.pop();
            } else {
                break;
            }
        }
        let n = quantity.len() as i32;
        let m = pool.len();
        let mut q = Vec::new();
        for j in (0..m).rev() {
            q.push((true, 0i32, j));
        }
        while let Some((forward, i, j)) = q.pop() {
            if i >= n {
                return true;
            }
            if !forward {
                pool[j] += quantity[i as usize];
                continue;
            }
            if quantity[i as usize] > pool[j] {
                continue;
            }
            pool[j] -= quantity[i as usize];
            q.push((false, i, j));
            for j in (0..m).rev() {
                q.push((true, i+1, j));
            }
        }
        return false;
    }
}