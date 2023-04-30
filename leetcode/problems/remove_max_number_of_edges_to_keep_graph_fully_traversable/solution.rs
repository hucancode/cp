use std::collections::HashMap;
struct DisjointSet {
    parent: Vec<usize>,
    len: usize
}
impl DisjointSet {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            len: n,
        }
    }
    fn find(&mut self, x: usize) -> usize {
        let mut x = x;
        while x != self.parent[x] {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        return x;
    }
    fn union(&mut self, x:usize, y:usize) -> bool {
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            return false;
        }
        self.len -= 1;
        self.parent[x] = y;
        return true;
    }
    fn fully_connected(&self) -> bool {
        self.len == 1
    }
    fn connected(&mut self, x:usize, y:usize) -> bool {
        self.find(x) == self.find(y)
    }
}
impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut ret = 0;
        let mut alice = DisjointSet::new(n);
        let mut bob = DisjointSet::new(n);
        
        ret += edges.iter()
            .filter(|x| x[0] == 3)
            .map(|x| (x[1] as usize, x[2] as usize))
            .map(|(u,v)| (u-1, v-1))
            .filter(|&(u, v)| {
                let a = alice.union(u, v);
                let b = bob.union(u, v);
                !a && !b
            })
            .count();
        ret += edges.iter()
            .filter(|x| x[0] == 1)
            .map(|x| (x[1] as usize, x[2] as usize))
            .map(|(u,v)| (u-1, v-1))
            .filter(|&(u, v)| !alice.union(u, v))
            .count();
        ret += edges.iter()
            .filter(|x| x[0] == 2)
            .map(|x| (x[1] as usize, x[2] as usize))
            .map(|(u,v)| (u-1, v-1))
            .filter(|&(u, v)| !bob.union(u, v))
            .count();

        if alice.fully_connected() && bob.fully_connected() {
            return ret as i32;
        }
        return -1;
    }
}