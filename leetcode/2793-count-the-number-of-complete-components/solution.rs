impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut p = vec![0;n];
        let mut degree = vec![0;n];
        let mut count = vec![1;n];
        for i in 0..n {
            p[i] = i;
        }
        let mut find = |p: &Vec<usize>, u| {
            let mut ret = u;
            while p[ret] != ret {
                ret = p[ret];
            }
            ret
        };
        let mut union_uv = |p: &mut Vec<usize>, u, v| {
            let x = find(p, u);
            let y = find(p, v);
            p[y] = x;
            if x != y {
                count[x] += count[y];
            }
        };
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            union_uv(&mut p, u, v);
            degree[u] += 1;
            degree[v] += 1;
        }
        let mut is_complete = vec![true;n];
        for i in 0..n {
            let u = find(&p, i);
            if u != i {
                is_complete[i] = false;
            }
            if degree[i] != count[u] - 1 {
                is_complete[u] = false;
            }
        }
        is_complete.into_iter().filter(|&v| v).count() as i32
    }
}
