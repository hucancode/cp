use std::collections::HashSet;
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
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let n = strs.len();
        let mut dset = DisjointSet::new(n);
        let similar = |a: &String, b: &String| {
            return a.chars().zip(b.chars())
                .fold(0, |acc, (a, b)| acc + if a==b {0} else {1}) <= 2;
        };
        for i in 0..n {
            for j in (i+1)..n {
                if dset.connected(i, j) {
                    continue;
                }
                if similar(&strs[i], &strs[j]) {
                    dset.union(i, j);
                }
            }
        }
        let mut set = HashSet::new();
        for i in 0..n {
            set.insert(dset.find(i));
        }
        return set.len() as i32;
    }
}