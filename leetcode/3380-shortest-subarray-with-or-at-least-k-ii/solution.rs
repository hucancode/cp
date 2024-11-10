impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let INF = 1000_000_000i32;
        let mut count = vec![0;32];
        let mut prefix = vec![count.clone()];
        for i in nums.iter() {
            for b in 0..32 {
                if i & (1<<b) != 0 {
                    count[b] += 1;
                }
            }
            prefix.push(count.clone());
        }
        let at_least_k = |i: usize,j: usize| {
            for b in (0..32).rev() {
                let x = k & (1<<b) != 0;
                let y = prefix[j][b] - prefix[i][b] > 0;
                if !x && y {
                    return true;
                }
                if x && !y {
                    return false;
                }
            }
            return true;
        };
        //println!("{prefix:?}, k = {k:b}");
        let mut ret = INF;
        for i in 0..n {
            //println!("start at {i}");
            let mut l = i;
            let mut r = n-1;
            while l < r {
                let m = (l+r)/2;
                //println!("search [{l}~{r}], mid {m}");
                let ok = at_least_k(i, m+1);
                if ok {
                    r = m;
                } else {
                    l = m+1;
                }
            }
            let ok = at_least_k(i, l+1);
            //println!("arrived at {l} ok = {ok}");
            if ok {
                ret = std::cmp::min(ret, (l-i+1) as i32);
            }
        }
        if ret >= INF {
            return -1;
        }
        return ret;
    }
}
