impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let n = n as usize;
        let mut parent: Vec<usize> = (0..n).collect();
        let find = |mut x: usize, parent: &Vec<usize>| {
            while parent[x] != x {
                x = parent[x];
            }
            x
        };
        let union_xy = |mut x: usize, mut y: usize, parent: &mut Vec<usize>| {
            x = find(x, parent);
            y = find(y, parent);
            parent[x] = y;
        };
        for e in edges {
            union_xy(e[0] as usize, e[1] as usize, &mut parent);
        }
        find(source as usize, &parent) == find(destination as usize, &parent)
    }
}
