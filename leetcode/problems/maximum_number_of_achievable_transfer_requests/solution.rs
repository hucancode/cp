impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        let is_balanced = |mask| {
            let mut come = vec![0;n as usize];
            let mut go = vec![0;n as usize];
            for (a,b) in requests.iter()
                .enumerate()
                .filter(|(i,_)| mask & 1<<i != 0)
                .map(|(_, r)| (r[0], r[1])) {
                go[a as usize] += 1;
                come[b as usize] += 1;
            }
            come.iter().zip(go.iter()).all(|(x, y)| x == y)
        };
        let mut ret = 0;
        let m: i32 = 1<<requests.len();
        for mask in (1..m).rev() {
            let k = mask.count_ones() as i32;
            if k <= ret {
                continue;
            }
            if is_balanced(mask) {
                ret = k;
            }
        }
        return ret;
    }
}