struct DisjointSet {
    parent: Vec<usize>,
}
impl DisjointSet {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }
    fn find(&self, x: usize) -> usize {
        let mut x = x;
        while x != self.parent[x] {
            x = self.parent[x];
        }
        return x;
    }
    fn union(&mut self, x:usize, y:usize) {
        let x = self.find(x);
        let y = self.find(y);
        self.parent[x] = self.parent[y];
    }
    fn connected(&self, x:usize, y:usize) -> bool {
        return self.find(x) == self.find(y);
    }
}
impl Solution {
    pub fn distance_limited_paths_exist(n: i32, edge_list: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut dset = DisjointSet::new(n as usize);
        let mut tasks: Vec<usize> = (0..queries.len()).collect();
        tasks.sort_by(|i, j| queries[*i][2].cmp(&queries[*j][2]));
        let mut edge_list = edge_list;
        edge_list.sort_by(|a, b| b[2].cmp(&a[2]));
        let mut ret = vec![false;queries.len()];
        for i in tasks {
            let p = queries[i][0] as usize;
            let q = queries[i][1] as usize;
            let limit = queries[i][2];
            while let Some(edge) = edge_list.last() {
                let u = edge[0] as usize;
                let v = edge[1] as usize;
                let d = edge[2];
                if d >= limit {
                    break;
                }
                dset.union(u, v);
                edge_list.pop();
            }
            ret[i] = dset.connected(p, q);
        }
        return ret;
    }
}