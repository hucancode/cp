impl Solution {
    pub fn make_string_good(s: String) -> i32 {
        use std::cmp::{min, max};
        let n = 26;
        let mut occ = vec![0i32;n];
        for c in s.chars() {
            let k = c as usize - 'a' as usize;
            occ[k] += 1;
        }
        let mut low = *occ.iter().min().unwrap();
        let mut high = *occ.iter().max().unwrap();
        let calculate = |threshold: i32| {
            let mut f = vec![0i32;n]; // f[i] = cost to make character i good and keep character i
            let mut g = vec![0i32;n]; // g[i] = cost to make character i good and throw character i
            f[0] = (occ[0] - threshold).abs();
            g[0] = occ[0];
            for i in 1..n {
                g[i] = min(g[i-1], f[i-1]) + occ[i];
                let extra_f = max(0, occ[i-1] - threshold);
                let extra_g = occ[i-1];
                if threshold > occ[i] {
                    let needed = threshold - occ[i];
                    f[i] = min(f[i-1] + max(0, needed - extra_f), g[i-1] + max(0, needed - extra_g));
                } else {
                    let needed = occ[i] - threshold;
                    f[i] = min(f[i-1] + needed, g[i-1] + needed);
                }
            }
            min(f[n-1], g[n-1])
        };
        (low..=high)
            .map(|threshold| calculate(threshold))
            .min()
            .unwrap()
    }
}
