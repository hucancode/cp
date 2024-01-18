use std::collections::HashMap;
use std::cmp::min;
impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let mut group = HashMap::new();
        for x in tasks {
            group.entry(x)
                .and_modify(|n| *n+= 1)
                .or_insert(1);
        }
        let INF = 1000_000_000;
        let mut ret = 0;
        let mut f = vec![INF+1;4];
        f[0] = 0;
        f[2] = 1;
        f[3] = 1;
        for (k,v) in group {
            while v >= f.len() {
                let n = f.len();
                f.push(min(INF, min(f[n-3], f[n-2])+1));
            }
            if f[v] > INF {
                return -1;
            }
            ret += f[v];
        }
        return ret;
    }
}