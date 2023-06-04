struct DisjointSet {
    parent: Vec<usize>,
    set_count: usize,
    set_len: Vec<usize>,
}
impl DisjointSet {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            set_count: n,
            set_len: vec![1;n],
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
        self.set_count -= 1;
        self.parent[x] = y;
        self.set_len[y] += self.set_len[x];
        return true;
    }
    fn fully_connected(&self) -> bool {
        self.set_count == 1
    }
    fn connected(&mut self, x:usize, y:usize) -> bool {
        self.find(x) == self.find(y)
    }
}

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut dset = DisjointSet::new(n);
        for i in 0..n {
            for j in 0..n {
                if is_connected[i][j] == 0 {
                    continue;
                }
                dset.union(i,j);
            }
        }
        dset.set_count as i32
    }
}