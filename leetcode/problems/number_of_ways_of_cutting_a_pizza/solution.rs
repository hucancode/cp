use std::collections::HashMap;
impl Solution {
    fn solve(prefix: &Vec<Vec<i32>>, f: &mut HashMap<(usize, usize, usize), i32>, x: usize, y: usize, k: usize) -> i32 {
        let hash_key = (x, y, k);
        if let Some(ret) = f.get_mut(&hash_key) {
            return *ret;
        }
        let mut ret = 0;
        let m = prefix.len();
        let n = prefix[0].len();
        
        let has_apple_in = |x1: usize, y1: usize, x2: usize, y2: usize| -> bool {
            let mut apple = prefix[x2][y2];
            if x1 > 0 {
                apple -= prefix[x1-1][y2];
            }
            if y1 > 0 {
                apple -= prefix[x2][y1-1];
            }
            if x1 > 0 && y1 > 0 {
                apple += prefix[x1-1][y1-1];
            }
            return apple > 0;
        };
        if k == 0 {
            ret = if has_apple_in(x, y, m-1, n-1) {1} else {0};
            f.insert(hash_key, ret);
            return ret;
        }
        const INF: i32 = 1e9 as i32 + 7;
        let mut xi = x;
        while xi < m && !has_apple_in(xi, y, xi, n-1) {
            xi += 1;
        }
        while xi < m-1 {
            xi += 1;
            ret += Self::solve(prefix, f, xi, y, k-1);
            ret = ret % INF;
        }
        let mut yi = y;
        while yi < n && !has_apple_in(x, yi, m-1, yi) {
            yi += 1;
        }
        while yi < n-1 {
            yi += 1;
            ret += Self::solve(prefix, f, x, yi, k-1);
            ret = ret % INF;
        }
        f.insert(hash_key, ret);
        return ret;
    }
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        let mut f: HashMap<(usize, usize, usize), i32> = HashMap::new();
        let n = pizza.len();
        let m = pizza[0].len();
        let mut prefix = vec![vec![0;m];n];
        for (i, str) in pizza.iter().enumerate() {
            for (j, c) in str.chars().enumerate() {
                prefix[i][j] = if c == 'A' {1} else {0};
                if i > 0 {
                    prefix[i][j] += prefix[i-1][j];
                }
                if j > 0 {
                    prefix[i][j] += prefix[i][j-1];
                }
                if i > 0 && j > 0 {
                    prefix[i][j] -= prefix[i-1][j-1];
                }
            }
        }
        return Self::solve(&prefix, &mut f, 0, 0, (k - 1) as usize);
    }
}