impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut trusted_count = vec![0;n+1];
        let mut trusting_count = vec![0;n+1];
        for e in trust {
            let u = e[0] as usize;
            let v = e[1] as usize;
            if u == v {
                continue;
            }
            trusted_count[v] += 1;
            trusting_count[u] += 1;
        }
        let mut ret = -1;
        for i in 1..=n {
            if trusted_count[i] == n - 1 && trusting_count[i] == 0 {
                if ret != -1 {
                    ret = -1;
                    break;
                }
                ret = i as i32;
            }
        }
        return ret;
    }
}