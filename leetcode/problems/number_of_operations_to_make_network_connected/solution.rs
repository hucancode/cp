use std::collections::HashSet;

pub struct Union {
    group: Vec<usize>,
}
impl Union {
    pub fn new(n: usize) -> Union {
        Union {
            group: (0..n).collect(),
        }
    }
    pub fn find(&self, x: usize) -> usize {
        if(self.group[x] == x) {
            return x;
        }
        return Union::find(self, self.group[x]);
    }
    pub fn union(&mut self, x: usize, y: usize) {
        let px = Union::find(self, x);
        let py = Union::find(self, y);
        self.group[px] = py;
    }
}
impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        if(connections.len() < n - 1) {
            return -1;
        }
        let mut u = Union::new(n);
        for c in &connections {
            u.union(c[0] as usize, c[1] as usize);
        }
        let mut set = HashSet::new();
        for i in 0..n {
            set.insert(u.find(i));
        }
        return set.len() as i32 - 1;
    }
}