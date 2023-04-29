use std::collections::HashMap;
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
        if x == y {
            return;
        }
        let x = self.find(x);
        let y = self.find(y);
        self.parent[x] = y;
    }
    fn connected(&self, x:usize, y:usize) -> bool {
        return self.find(x) == self.find(y);
    }
}
impl Solution {
    pub fn group_strings(words: Vec<String>) -> Vec<i32> {
        let n = words.len();
        let mut words: Vec<i32> = words.into_iter()
            .map(|s| s.chars()
                .map(|c| c as i32 - 'a' as i32)
                .map(|x| 1<<x)
                .sum())
            .collect();
        words.sort();
        let mut dset = DisjointSet::new(n);
        let mut pattern = HashMap::new();
        for i in 0..n {
            let x = words[i];
            pattern.insert(x, i);
            for b in 0..26 {
                let y = x & !(1<<b);
                if x == y {
                    continue;
                }
                if let Some(j) = pattern.get(&y) {
                    dset.union(i, *j);
                }
                pattern.insert(y, i);
            }
        }
        let mut set = HashMap::new();
        for i in 0..n {
            let x = set.entry(dset.find(i)).or_insert(0);
            *x += 1;
        }
        let n = set.len() as i32;
        let k = *set.values().max().unwrap_or(&0);
        return vec![n,k];
    }
}